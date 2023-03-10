/// <reference types="vitest" />
import { defineConfig } from 'vitest/config';
import { fileURLToPath } from 'url';

export default defineConfig({
    resolve: {
        alias: {
            '~': fileURLToPath(new URL('./src', import.meta.url))
        }
    },
    test: {
        root: 'src',
        include: ['./**/*.{spec.ts,spec.js,test.ts,test.js}'],
        exclude: ['**/node_modules/**', '**/dist/**'],
        coverage: {
            provider: "c8",
            exclude: ['node_modules'],
            reporter: ['html'],
            reportsDirectory: 'coverage'
        },
        // environment: 'miniflare',
        // environmentOptions: {
        //     modules: true,
        //     scriptPath: './dist/index.mjs'
        // },
        passWithNoTests: true
    }
});