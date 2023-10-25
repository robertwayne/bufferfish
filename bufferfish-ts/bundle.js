await Bun.build({
    entrypoints: ['src/index.ts',],
    outdir: 'dist',
    minify: true,
    naming: {
        entry: 'bufferfish.min.js',
    },
    sourcemap: "external"
})
