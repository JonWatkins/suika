# Suika Contributing Guide

Hi! I'm really excited that you are interested in contributing to Suika. Before submitting your contribution, please make sure to take a moment and read through the following guidelines:

- [Code of Conduct](./CODE_OF_CONDUCT.md)
- [Issue Reporting Guidelines](#issue-reporting-guidelines)
- [Pull Request Guidelines](#pull-request-guidelines)
- [Development Setup](#development-setup)
- [Project Structure](#project-structure)

## Issue Reporting Guidelines

- Always use [https://github.com/JonWatkins/suika/issues/](https://github.com/JonWatkins/suika/issues/) to create new issues.

## Pull Request Guidelines

- The `master` branch is just a snapshot of the latest stable release. All development should be done in dedicated branches. **Do not submit PRs against the `master` branch.**

- Checkout a topic branch from the relevant branch, e.g. `dev`, and merge back against that branch.

- Work in the `src` folder and **DO NOT** checkin `dist` in the commits.

- Make sure `npm test` passes. (see [development setup](#development-setup))

- If adding a new feature:

  - Add accompanying test case.
  - Provide a convincing reason to add this feature. Ideally, you should open a suggestion issue first and have it approved before working on it.

- If fixing bug:
  - Provide a detailed description of the bug in the PR. Live demo preferred.
  - Add appropriate test coverage if applicable.

## Development Setup

You will need [Node.js](https://nodejs.org) **version 18+**, and [PNPM](https://pnpm.io) **version 8+**.

After cloning the repo, run:

```bash
$ pnpm i # install the dependencies of the project
```

A high level overview of tools used:

- [TypeScript](https://www.typescriptlang.org/) as the development language
- [Rollup](https://rollupjs.org) for development and production bundling
- [Jest](https://jestjs.io/) for unit testing
- [Prettier](https://prettier.io/) for code formatting
- [ESLint](https://eslint.org/) for static error prevention (outside of types)

### Commonly used NPM scripts

```bash
# run unit tests
$ pnpm run test

# build all dist files, including npm packages
$ pnpm run build
```

There are some other scripts available in the `scripts` section of the `package.json` file.

The default test script will do the following: lint with ESLint -> unit tests with coverage. **Please make sure to have this pass successfully before submitting a PR.** Although the same tests will be run against your PR on the CI server, it is better to have it working locally.

## Project Structure

This repository employs a [monorepo](https://en.wikipedia.org/wiki/Monorepo) setup which hosts a number of associated packages under the `packages` directory:

- `suika`: The main framework.
- `suika-ui`: A simple UI component library.
- `suika-router`: A simple router.
