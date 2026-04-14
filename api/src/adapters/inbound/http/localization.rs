use actix_web::HttpRequest;

/// Extract the primary language tag from the `Accept-Language` header.
/// E.g. `"fr-FR,fr;q=0.9,en;q=0.8"` → `"fr"`. Falls back to `"en"`.
pub fn parse_locale(req: &HttpRequest) -> String {
    req.headers()
        .get("accept-language")
        .and_then(|v| v.to_str().ok())
        .and_then(|s| {
            s.split(',').next().map(|tag| {
                tag.split(';')
                    .next()
                    .unwrap_or(tag)
                    .split('-')
                    .next()
                    .unwrap_or(tag)
                    .trim()
                    .to_lowercase()
            })
        })
        .unwrap_or_else(|| "en".to_string())
}
