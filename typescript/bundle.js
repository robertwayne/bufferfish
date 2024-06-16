await Bun.build({
    entrypoints: ["src/bufferfish.ts"],
    outdir: "dist",
    minify: true,
    naming: {
        entry: "bufferfish.min.js",
    },
    sourcemap: "external",
})

await Bun.build({
    entrypoints: ["src/bufferfish.ts"],
    outdir: "dist",
    minify: false,
    naming: {
        entry: "bufferfish.js",
    },
})
