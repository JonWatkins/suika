# Suika ![GitHub Workflow Status](https://img.shields.io/github/actions/workflow/status/JonWatkins/suika/main.yml) ![npm](https://img.shields.io/npm/v/suika) ![GitHub](https://img.shields.io/github/license/JonWatkins/suika) [![codecov](https://codecov.io/gh/JonWatkins/suika/branch/main/graph/badge.svg?token=CZ8QB5X8S5)](https://codecov.io/gh/JonWatkins/suika)

Suika is a lightweight component based Javascript library for building user interfaces.

## Installation

You can use Suika as a `<script>` tag from a CDN, or as a `suika` package on npm.

```bash
npm i suika
```

## Compiler

The easiest way to compile a Suika app is with [vite](https://vitejs.dev/). Here is an example `vite.config.js`.

```js
import { defineConfig } from "vite";

export default defineConfig(() => {
  return {
    esbuild: {
      jsxFactory: "h",
      jsxFragment: "Fragment",
      jsxInject: `import { h, Fragment } from 'suika'`,
    },
  };
});
```

## Examples

```jsx
import { App, Component, mount } from "suika";

const root = document.getElementById("app");

class Counter extends Component {
  constructor() {
    super();
  }
  state = {
    count: 0,
  };
  render() {
    return (
      <div id="counter">
        <h1>Count: {this.state.count}</h1>
        <button onclick={() => ++this.state.count}>inc</button>
      </div>
    );
  }
}

class App extends Suika {
  constructor() {
    super();
  }
  render() {
    return (
      <div id="container">
        <Counter />
      </div>
    );
  }
}

mount(App, root);
```

Suika is [MIT licensed](./LICENSE).
