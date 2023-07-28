export default {
  verbose: false,
  testEnvironment: "jsdom",
  collectCoverage: true,
  coverageDirectory: "coverage",
  collectCoverageFrom: ["**/*.ts", "!**/*.d.ts", "!jest.config.ts"],
};
