![GitHub Workflow Status](https://img.shields.io/github/actions/workflow/status/JonWatkins/suika/main.yml) ![npm](https://img.shields.io/npm/v/suika) ![GitHub](https://img.shields.io/github/license/JonWatkins/suika) [![codecov](https://codecov.io/gh/JonWatkins/suika/branch/main/graph/badge.svg?token=CZ8QB5X8S5)](https://codecov.io/gh/JonWatkins/suika)

Suika Router (WIP) is a lightweight routing library for the Suika framework.

## Installation

You can use `suika-router` as a package on `npm`

```bash
npm install suika-router@latest
```

Or you can use Suika from a CDN.

```html
<script src="https://unpkg.com/suika@1.4.1/dist/bundle.umd.js"></script>
<script src="https://unpkg.com/suika-router@1.0.0/dist/bundle.umd.js"></script>
```

## TypeScript

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
