![GitHub Workflow Status](https://img.shields.io/github/actions/workflow/status/JonWatkins/suika/main.yml) ![npm](https://img.shields.io/npm/v/suika) ![GitHub](https://img.shields.io/github/license/JonWatkins/suika) [![codecov](https://codecov.io/gh/JonWatkins/suika/branch/main/graph/badge.svg?token=CZ8QB5X8S5)](https://codecov.io/gh/JonWatkins/suika)

Suika is a lightweight component based `Javascript` library for building user interfaces, that relies on a virtual DOM with `JSX` support.

## Installation

You can use `suika` as a package on `npm`

```bash
npm install suika@latest
```

## Documentation

Please follow the documentation at [jonwatkins.github.io/suika/](https://jonwatkins.github.io/suika/).

## Example

A simple todo example can be found [here](https://jonwatkins.github.io/suika-example/). Or you can checkout the repo [here](https://github.com/JonWatkins/suika-example).

## Usage

The easiest way to use Suika is to use the [vite](https://vitejs.dev/) bundler.Below is an example `vite.config.js` for bundling a Suika application.

```js
import { defineConfig } from "vite";

export default defineConfig(() => {
  return {
    build: {
      outDir: "./dist",
    },
    esbuild: {
      jsxFactory: "h",
      jsxFragment: "Fragment",
    },
  };
});
```

By default `Vite` will use the `src/index.ts` file for the bundle. Below is an example of a simple `index.ts` and `App.tsx` file for a Suika application.

```ts
import { mount } from "suika";
import { App } from "./App";
const el = document.querySelector("#app");
mount(App, el as HTMLElement);
```

`App.tsx`

```jsx
import { App, Component, mount, h, reactive } from "suika";

export default class App extends Component {
  state = reactive({
    count: 0,
  });
  render() {
    return (
      <div id="counter">
        <h1 className="title">Count: {this.state.value.count.toString()}</h1>
        <button onclick={() => this.state.value.count++}>Inc</button>
      </div>
    );
  }
}

mount(App, root);
```

## License

[MIT](https://opensource.org/licenses/MIT)

Copyright (c) 2023-present, Jon Watkins
