module.exports = {
  root: true,
  parser: "@typescript-eslint/parser",
  parserOptions: {
    ecmaVersion: 2020,
    sourceType: "module",
    ecmaFeatures: {
      jsx: true,
    },
  },
  settings: {
    react: {
      version: "detect",
    },
    "import/resolver": {
      node: {
        paths: ["src"],
        extensions: [".js", ".jsx", ".ts", ".tsx"],
      },
    },
  },
  env: {
    browser: true,
    amd: true,
    node: true,
  },
  extends: [
    "eslint:recommended",
    "plugin:@typescript-eslint/recommended",
    "plugin:prettier/recommended",
  ],
  plugins: ["prettier"],
  rules: {
    "linebreak-style": ["error", "windows"],
    "prettier/prettier": ["error", {}, { usePrettierrc: true }],
    "@typescript-eslint/explicit-function-return-type": "off",
    "@typescript-eslint/no-empty-function": "off",
    "@typescript-eslint/ban-types": "off",
    "@typescript-eslint/no-namespace": "off",
    "@typescript-eslint/ban-ts-comment": "off",
    "@typescript-eslint/no-unused-vars": [
      "error",
      {
        varsIgnorePattern: "^(createElement|Fragment)$",
      },
    ],
  },
  globals: {
    describe: true,
    it: true,
    expect: true,
  },
};
