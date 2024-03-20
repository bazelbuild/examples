// Based on https://github.com/vercel/next.js/blob/28454c6ddbc310419467e5415aee26e48d079b46/examples/with-jest/jest.config.js

const nextJest = require('next/jest');

const createJestConfig = nextJest({
  // Provide the path to your Next.js app to load next.config.js and .env files in your test environment.
  // This path is relative to the WORKSPACE file when testing under Bazel.
  dir: './next.js',
});

// Add any custom config to be passed to Jest
const customJestConfig = {
  moduleNameMapper: {
    // Handle module aliases (this will be automatically configured for you soon)
    '^@/components/(.*)$': '<rootDir>/components/$1',

    '^@/pages/(.*)$': '<rootDir>/pages/$1',
  },
  testEnvironment: 'jest-environment-jsdom',
};

// createJestConfig is exported this way to ensure that next/jest can load the Next.js config which is async
module.exports = createJestConfig(customJestConfig);
