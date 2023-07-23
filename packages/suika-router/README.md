![GitHub Workflow Status](https://img.shields.io/github/actions/workflow/status/JonWatkins/suika/main.yml) ![npm](https://img.shields.io/npm/v/suika) ![GitHub](https://img.shields.io/github/license/JonWatkins/suika) [![codecov](https://codecov.io/gh/JonWatkins/suika/branch/main/graph/badge.svg?token=CZ8QB5X8S5)](https://codecov.io/gh/JonWatkins/suika)

Suika Router (WIP) is a lightweight routing library for the Suika framework.

## Installation

You can use `suika-router` as a package on `npm`

```bash
npm install suika@latest suika-router@latest
```

## Documentation

Please follow the documentation at [jonwatkins.github.io/suika/](https://jonwatkins.github.io/suika/).

## Usage

```jsx
import { Component, mount, h } from "suika";
import { createRouter, RouterView, RouterLink } from "suika-router";

const root = document.getElementBytId("app");

class About extends Component {
  render() {
    return (
      <section class="content">
        <h1>About Us</h1>
      </section>
    )
  }
}

class Home extends Component {
  render() {
    return (
      <section class="content">
        <h1>Hello World</h1>
      </section>
    )
  }
}

const router = createRouter({
  mode: 'hash',
  routes: [
    { path: '/', component: Home },
    { path: '/about', component: About }
  ]
})

class App extends Component {
  render() {
    return (
      <div id="container">
        <ul class="navigation">
          <li><RouterLink to="/">Home</RouterLink></li>
          <li><RouterLink to="/about">About</RouterLink></li>
        </ul>
        <RouterView router={router}>
      </div>
    );
  }
}

mount(App, root);
```

## License

[MIT](https://opensource.org/licenses/MIT)

Copyright (c) 2023-present, Jon Watkins
