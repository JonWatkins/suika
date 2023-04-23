![GitHub Workflow Status](https://img.shields.io/github/actions/workflow/status/JonWatkins/suika/main.yml) ![npm](https://img.shields.io/npm/v/suika) ![GitHub](https://img.shields.io/github/license/JonWatkins/suika) [![codecov](https://codecov.io/gh/JonWatkins/suika/branch/main/graph/badge.svg?token=CZ8QB5X8S5)](https://codecov.io/gh/JonWatkins/suika)

Suika UI (WIP) is a lightweight UI library for the Suika framework.

## Installation

You can use `suika-ui` as a package on `npm`

```bash
npm install suika-ui@latest
```

Or you can use Suika from a CDN.

```html
<link href="https://unpkg.com/suika-ui@1.1.7/dist/style.css" />
<script src="https://unpkg.com/suika@1.4.1/dist/bundle.umd.js"></script>
<script src="https://unpkg.com/suika-ui@1.1.7/dist/bundle.umd.js"></script>
```

## Documentation

Documentation is comming soon.

## TypeScript

```jsx
import { Component, mount, h } from "suika";
import { Card, CardHeader, CardTitle, CardBody, Button } from "suika-ui";

const root = document.getElementBytId("app");

class App extends Component {
  state = {
    count: 0,
  };

  inc() {
    this.state.count++;
  }

  render() {
    return (
      <Card>
        <CardHeader className="bg-light">
          <CardTitle is="h2">{this.state.count}</CardTitle>
        </CardHeader>
        <CardBody>
          <Button onclick={() => this.inc()}>Inc</Button>
        </CardBody>
      </Card>
    );
  }
}

mount(App, root);
```
