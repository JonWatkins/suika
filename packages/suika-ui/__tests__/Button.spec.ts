import { Component, mount, h } from "suika";
import { Button, ButtonGroup } from "../src/components/Button";

describe("buttons", () => {
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
      expect(el.outerHTML).toMatchSnapshot();
    });

    it("should be able to render a button with text", () => {
      class Renderer extends Component {
        render() {
          return h(Button, {}, 'Hello');
        }
      }

      const app = mount(Renderer, document.createElement("div"));
      const el = app._el as HTMLElement;
      expect(el.nodeName).toBe("BUTTON");
      expect(el.className).toBe("btn btn-primary btn-md");
      expect(el.outerHTML).toMatchSnapshot();
    });
  });

  describe("button group", () => {
    it("should be able to render a button", () => {
      class Renderer extends Component {
        render() {
          return h(ButtonGroup, {});
        }
      }

      const app = mount(Renderer, document.createElement("div"));
      const el = app._el as HTMLElement;
      expect(el.nodeName).toBe("DIV");
      expect(el.className).toBe("btn-group");
      expect(el.outerHTML).toMatchSnapshot();
    });

    it('should be able to have nested buttons', () => {
      class Renderer extends Component {
        render() {
          return h(ButtonGroup, {},
            h(Button, {}, 'Hello')
          );
        }
      }

      const app = mount(Renderer, document.createElement("div"));
      const el = app._el as HTMLElement;
      expect(el.nodeName).toBe("DIV");
      expect(el.className).toBe("btn-group");
      expect(el.outerHTML).toMatchSnapshot();
    })
  });
});
