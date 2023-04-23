/*
 * For a detailed explanation regarding each configuration property and type check, visit:
 * https://jestjs.io/docs/configuration
 */

export default {
  verbose: false,
  testEnvironment: "jsdom",
  collectCoverage: true,
  coverageDirectory: "coverage",
  collectCoverageFrom: ["**/*.ts", "!**/*.d.ts", "!jest.config.ts"],
};
