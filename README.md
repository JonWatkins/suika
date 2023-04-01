# Suika ![GitHub Workflow Status](https://img.shields.io/github/actions/workflow/status/JonWatkins/suika/main.yml) ![npm](https://img.shields.io/npm/v/suika) ![GitHub](https://img.shields.io/github/license/JonWatkins/suika) [![codecov](https://codecov.io/gh/JonWatkins/suika/branch/main/graph/badge.svg?token=CZ8QB5X8S5)](https://codecov.io/gh/JonWatkins/suika)

Suika is a lightweight component based `Javascript` library for building user interfaces, that relies on a virtual DOM with keyed diffs with `JSX` support.

## Installation

You can use as a `suika` package on `npm`

```bash
npm install@latest suika
```

Or you can use Suika as a `<script>` tag from a CDN.

```html
<script src="https://unpkg.com/suika@1.0.4/dist/bundle.umd.cjs"></script>
```

## Documentation

Documentaion is comming soon.

## Usage

The easiest way to use Suika is to use the [vite](https://vitejs.dev/) bundler, as this supports `SCSS` and `Typescript` out of the box. Below is an example `vite.config.js` for bundling a Suika application.

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

## TypeScript

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

## JavaScript

You don't have to use `Typescript` to use Suika, you can use plain old `JavaScript`.

```html
<script src="https://unpkg.com/suika@1.0.4/dist/bundle.umd.cjs"></script>
<script type="text/javascript">
  const root = document.getElementById("app");

  class App extends suika.App {
    render() {
      return suika.h(
        "div",
        { key: "container" },
        suika.h("h1", { key: "title" }, "Hello World")
      );
    }
  }

  suika.mount(App, root);
</script>

Suika is [MIT licensed](./LICENSE).
```
