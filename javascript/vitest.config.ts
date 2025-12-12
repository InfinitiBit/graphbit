import { defineConfig } from 'vitest/config';

export default defineConfig({
  test: {
    globals: true,
    environment: 'node',
    coverage: {
      provider: 'v8',
      reporter: ['text', 'json', 'html', 'lcov'],
      exclude: [
        'node_modules/',
        'build/',
        'tests/',
        '*.config.ts',
        '*.config.js',
        '**/*.d.ts',
        '**/*.node',
      ],
    },
    testTimeout: 30000,
    hookTimeout: 30000,
    include: ['tests/**/*.test.ts', 'tests/**/*.spec.ts'],
    exclude: ['node_modules/', 'build/'],
  },
});
