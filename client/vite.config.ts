import babel from "@rolldown/plugin-babel";
import tailwindcss from "@tailwindcss/vite";
import { TanStackRouterVite } from "@tanstack/router-vite-plugin";
import react, { reactCompilerPreset } from "@vitejs/plugin-react";
import { defineConfig } from "vite";
import oxlintPlugin from "vite-plugin-oxlint";
import { VitePWA } from "vite-plugin-pwa";


export default defineConfig({
    resolve: {
        tsconfigPaths: true,
    },
    plugins: [
        react(),
        babel({ presets: [reactCompilerPreset()] }),
        TanStackRouterVite(),
        tailwindcss(),
        oxlintPlugin({ path: "src", failOnError: true }),
        VitePWA({
            registerType: "autoUpdate",
            manifest: {
                name: "Garden Planner",
                short_name: "Garden",
                description: "Plan your vegetable garden",
                theme_color: "#16a34a",
                background_color: "#ffffff",
                display: "standalone",
                icons: [
                    {
                        src: "icons/pwa-192x192.png",
                        sizes: "192x192",
                        type: "image/png",
                    },
                    {
                        src: "icons/pwa-512x512.png",
                        sizes: "512x512",
                        type: "image/png",
                    },
                ],
            },
            workbox: {
                runtimeCaching: [
                    {
                        urlPattern: /\/api\/vegetables/,
                        handler: "StaleWhileRevalidate",
                        options: {
                            cacheName: "vegetables-cache",
                            expiration: { maxAgeSeconds: 60 * 60 * 24 },
                        },
                    },
                ],
            },
        }),
    ],
});
