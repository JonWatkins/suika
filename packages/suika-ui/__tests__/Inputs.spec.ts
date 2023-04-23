import { Component, mount, h } from "suika";
import { Input, InputGroup } from "../src/components/Inputs";

describe("Inputs", () => {
  describe("Input", () => {
    it("should be able to render an input", () => {
      class Renderer extends Component {
        render() {
          return h(Input, {});
        }
      }

      const app = mount(Renderer, document.createElement("div"));
      const el = app._el as HTMLElement;
      expect(el.nodeName).toBe("INPUT");
      expect(el.className).toBe("form-control");
    });
  });

  describe("InputGroup", () => {
    it("should be able to render an input group", () => {
      class Renderer extends Component {
        render() {
          return h(InputGroup, {});
        }
      }

      const app = mount(Renderer, document.createElement("div"));
      const el = app._el as HTMLElement;
      expect(el.nodeName).toBe("DIV");
      expect(el.className).toBe("input-group");
    });
  });
});
