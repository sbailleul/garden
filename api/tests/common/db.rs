use deadpool_postgres::{Manager, ManagerConfig, Pool, RecyclingMethod};
use tokio::sync::OnceCell;
use tokio_postgres::NoTls;

mod embedded {
    use refinery::embed_migrations;
    embed_migrations!("migrations");
}

// ---------------------------------------------------------------------------
// Database URL resolution (.env.test → DATABASE_URL)
// ---------------------------------------------------------------------------

fn database_url() -> Option<String> {
    dotenvy::from_filename(".env.test").ok();
    std::env::var("DATABASE_URL").ok()
}

async fn build_pool(url: &str) -> Pool {
    let pg_config: tokio_postgres::Config = url.parse().expect("invalid database URL");
    let mgr_config = ManagerConfig {
        recycling_method: RecyclingMethod::Fast,
    };
    let mgr = Manager::from_config(pg_config, NoTls, mgr_config);
    Pool::builder(mgr)
        .max_size(20)
        .build()
        .expect("failed to build pool")
}

// ---------------------------------------------------------------------------
// Migration guard — migrations run exactly once per test binary execution.
// ---------------------------------------------------------------------------

static MIGRATIONS_DONE: OnceCell<()> = OnceCell::const_new();

async fn ensure_migrations() {
    MIGRATIONS_DONE
        .get_or_init(|| async {
            let url = database_url().expect(".env.test must define DATABASE_URL");
            let pool = build_pool(&url).await;
            let mut client = pool.get().await.expect("failed to get DB client");
            embedded::migrations::runner()
                .run_async(&mut **client)
                .await
                .expect("migrations failed");
        })
        .await;
}

async fn test_pool() -> Pool {
    migrated_pool().await
}

/// Runs migrations (once) then returns a fresh pool.
pub async fn migrated_pool() -> Pool {
    ensure_migrations().await;
    let url = database_url().expect(".env.test must define DATABASE_URL");
    build_pool(&url).await
}

/// Runs all `.down.sql` rollback files in reverse version order (highest → lowest).
///
/// # Panics
/// Panics if any SQL fails or the database URL is not configured.
async fn run_all_down_migrations() {
    let migrations_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("migrations");
    let mut down_files: Vec<(u32, std::path::PathBuf)> = std::fs::read_dir(&migrations_dir)
        .expect("failed to read migrations dir")
        .filter_map(|e| e.ok())
        .map(|e| e.path())
        .filter(|p| {
            p.file_name()
                .and_then(|n| n.to_str())
                .map(|n| n.ends_with(".down.sql"))
                .unwrap_or(false)
        })
        .filter_map(|p| {
            let name = p.file_name()?.to_str()?;
            let version: u32 = name
                .trim_start_matches('V')
                .split("__")
                .next()?
                .parse()
                .ok()?;
            Some((version, p))
        })
        .collect();

    // Apply in reverse version order so FK constraints are satisfied.
    down_files.sort_by(|a, b| b.0.cmp(&a.0));

    let url = database_url().expect(".env.test must define DATABASE_URL");
    let pool = build_pool(&url).await;
    let client = pool.get().await.expect("failed to get DB client");

    for (version, path) in &down_files {
        let sql = std::fs::read_to_string(path)
            .unwrap_or_else(|e| panic!("failed to read {}: {e}", path.display()));
        client
            .batch_execute(&sql)
            .await
            .unwrap_or_else(|e| panic!("down migration V{version} failed: {e}"));
    }
}
