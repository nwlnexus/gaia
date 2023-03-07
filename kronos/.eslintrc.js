/** @type {import('@types/eslint').Linter.BaseConfig} */
module.exports = {
    root: true,
    extends: ['plugin:@typescript-eslint/recommended', 'prettier'],
    ignorePatterns: ['node_modules', 'build'],
    settings: {
        files: ['**/*.js', '**/*.jsx', '**/*.ts', '**/*.tsx']
    },
    parser: '@typescript-eslint/parser',
    plugins: ['@typescript-eslint/eslint-plugin', 'vitest']
};