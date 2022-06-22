module.exports = {
    env: {
        browser: true,
        node: true,
    },
    parser: "@typescript-eslint/parser",
    plugins: ["@typescript-eslint"],
    overrides: [
        {
            files: ["*.ts"],
            extends: [
                "eslint:recommended",
                "plugin:@typescript-eslint/recommended",
            ],
            rules: {
                "no-undef": "off",
                // "unused-imports/no-unused-imports": "error",
            },
        },
        {
            files: ["*.ts"],
            extends: ["plugin:@typescript-eslint/recommended"],
            rules: {
                "no-undef": "off",
            },
        },
    ],
    rules: {},
}
