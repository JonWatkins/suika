![GitHub Workflow Status](https://img.shields.io/github/actions/workflow/status/JonWatkins/suika/main.yml) ![npm](https://img.shields.io/npm/v/suika) ![GitHub](https://img.shields.io/github/license/JonWatkins/suika) [![codecov](https://codecov.io/gh/JonWatkins/suika/branch/main/graph/badge.svg?token=CZ8QB5X8S5)](https://codecov.io/gh/JonWatkins/suika)

Suika UI (WIP) is a lightweight UI library for the Suika framework.

## Installation

You can use `suika-ui` as a package on `npm`

```bash
npm install suika@latest suika-ui@latest
```

## Documentation

Please follow the documentation at [jonwatkins.github.io/suika/](https://jonwatkins.github.io/suika/).

## Usage

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
          <CardTitle is="h2">
            Count: {this.state.value.count.toString()}
          </CardTitle>
        </CardHeader>
        <CardBody>
          <Button onclick={() => this.state.value.count++}>Inc</Button>
        </CardBody>
      </Card>
    );
  }
}

mount(App, root);
```

## License

[MIT](https://opensource.org/licenses/MIT)

Copyright (c) 2023-present, Jon Watkins
