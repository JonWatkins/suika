export default {
  verbose: false,
  testEnvironment: "jsdom",
  collectCoverage: true,
  coverageDirectory: "coverage",
  collectCoverageFrom: ["**/*.ts", "!**/*.d.ts", "!jest.config.ts"],
  setupFiles: ["<rootDir>/scripts/jest.setup.js"],
  transformIgnorePatterns: ["node_modules/(?!suika)"],
  transform: {
    "\\.[jt]sx?$": "babel-jest",
  },
};
