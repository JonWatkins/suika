# Suika ![GitHub Workflow Status](https://img.shields.io/github/actions/workflow/status/JonWatkins/suika/main.yml) ![npm](https://img.shields.io/npm/v/suika) ![GitHub](https://img.shields.io/github/license/JonWatkins/suika) [![codecov](https://codecov.io/gh/JonWatkins/suika/branch/main/graph/badge.svg?token=CZ8QB5X8S5)](https://codecov.io/gh/JonWatkins/suika)

Suika is a lightweight component based `Javascript` library for building user interfaces, that relies on a virtual DOM with `JSX` support.

## Installation

You can use `suika` as a package on `npm`

```bash
npm install suika@latest
```

Or you can use Suika as a `<script>` tag from a CDN.

```html
<script src="https://unpkg.com/suika@1.3.8/dist/bundle.umd.js"></script>
```

## Documentation

Documentation is comming soon.

## Example

A simple todo example can be found [here](https://jonwatkins.github.io/suika-example/). Or you can checkout the repo [here](https://github.com/JonWatkins/suika-example).

## Usage

The easiest way to use Suika is to use the [vite](https://vitejs.dev/) bundler, as this supports `SCSS` and `Typescript` out of the box. Below is an example `vite.config.js` for bundling a Suika application.

```js
import { defineConfig } from "vite";

export default defineConfig(() => {
  return {
    esbuild: {
      jsxFactory: "h",
      jsxFragment: "Fragment",
    },
  };
});
```

## TypeScript

```jsx
import { App, Component, mount, h } from "suika";
import logoImg from "./public/images/logo.png";
import "./scss/styles.scss";

const Title = (props, children) => <h1 className="title">{...children}</h1>;

const Button = ({ inc, text }, children) => (
  <button onclick={() => inc()}>{...children}</button>
);

const Counter = ({ count, inc }) => (
  <div id="counter">
    <Title>Count: {count.toString()}</Title>
    <Button inc={inc}>Inc</Button>
  </div>
);

export default class App extends Component {
  constructor() {
    super();
  }
  state = {
    count: 0,
  }
  inc() {
    this.state.count++;
  }
  render() {
    return (
      <div id="container">
        <img src={logoImg}>
        <Counter count={this.state.count} inc={() => this.inc()} />
      </div>
    );
  }
}

mount(App, root);
```

## JavaScript

You don't have to use `Typescript` to use Suika, you can use plain old `JavaScript`. You can use the `UMD` or `ESM` bundle.

```html
<div id="app"></div>
<script
  src="https://unpkg.com/suika@1.3.8/dist/bundle.umd.js"
  type="text/javascript"
></script>
<script type="text/javascript">
  const root = document.getElementById("app");

  class App extends suika.Component {
    render() {
      return suika.h("div", {}, suika.h("h1", {}, "Hello World"));
    }
  }

  suika.mount(App, root);
</script>
```

```html
<div id="app"></div>
<script type="module">
  import {
    Component,
    mount,
    h,
  } from "https://unpkg.com/suika@1.3.8/dist/bundle.esm.js";

  const root = document.getElementById("app");

  class App extends Component {
    render() {
      return h(
        "div",
        { className: "container" },
        h("h1", { id: "title" }, "Hello World")
      );
    }
  }

  mount(App, root);
</script>
```

Suika is [MIT licensed](./LICENSE).
