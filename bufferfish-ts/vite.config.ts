import { defineConfig } from "vitest/config"
import dts from "vite-plugin-dts"
import path from "path"

export default defineConfig({
    build: {
        lib: {
            entry: path.resolve(__dirname, "src/index.ts"),
            name: "bufferfish",
            fileName: (format) => `bufferfish.${format}.js`,
        },
        emptyOutDir: true,
    },
    test: {
        includeSource: ["src/**/*.ts"],
        globals: true,
        environment: "happy-dom",
    },
    define: {
        "import.meta.vitest": false,
    },
    plugins: [dts()],
})
