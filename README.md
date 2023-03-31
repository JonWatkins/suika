# @nekojs/core ![GitHub Workflow Status](https://img.shields.io/github/actions/workflow/status/JonWatkins/nekojs/main.yml) ![npm](https://img.shields.io/npm/v/@nekojs/core) ![GitHub](https://img.shields.io/github/license/JonWatkins/nekojs) [![codecov](https://codecov.io/gh/JonWatkins/nekojs/branch/main/graph/badge.svg?token=CZ8QB5X8S5)](https://codecov.io/gh/JonWatkins/nekojs)


Nekojs is a lightweight component based Javascript library for building user interfaces.

## Installation

You can use NekoJS as a `<script>` tag from a CDN, or as a `@nekojs/core` package on npm.

```bash
npm i @nekojs/core
```

## Examples

```jsx
import { App, Component, mount } from '@nekojs/framework'

const root = document.getElementById('app')

class Counter extends Component {
  constructor() {
    super()
  }
  state = {
    count: 0
  }
  render() {
    return (
      <div id="counter">
        <h1>Count: {this.state.count}</h1>
        <button onclick={() => ++this.state.count}>
          inc
        </button>
      </div>
    )
  }
}

class App extends NekoJS {
  constructor() {
    super()
  }
  render() {
    return (
      <div id="container">
        <Counter />
      </div>
    )
  }
}

mount(App, root)
```

NekoJs is [MIT licensed](./LICENSE).
