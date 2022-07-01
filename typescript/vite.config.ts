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
        minify: "terser",
    },
    test: {
        includeSource: ["src/**/*.ts"],
    },
    define: {
        "import.meta.vitest": false,
    },
    plugins: [dts()],
})
