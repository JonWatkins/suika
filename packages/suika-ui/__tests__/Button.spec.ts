import { Component, mount, h } from "suika";
import { Button } from "../src/components/Button";

describe("button", () => {
  it("should be able to render a button", () => {
    class Renderer extends Component {
      render() {
        return h(Button, {});
      }
    }

    const app = mount(Renderer, document.createElement("div"));
    const el = app._el as HTMLElement;
    expect(el.nodeName).toBe("BUTTON");
    expect(el.className).toBe("btn btn-primary btn-md");
  });
});
