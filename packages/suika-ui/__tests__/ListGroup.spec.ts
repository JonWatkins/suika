import { Component, mount, h } from "suika";
import { ListGroup, ListGroupItem } from "../src/components/ListGroup";

describe("ListGroups", () => {
  describe("ListGroup", () => {
    it("should be able to render a list group", () => {
      class Renderer extends Component {
        render() {
          return h(ListGroup, {});
        }
      }

      const app = mount(Renderer, document.createElement("div"));
      const el = app._el as HTMLElement;
      expect(el.nodeName).toBe("UL");
      expect(el.className).toBe("list-group");
    });
  });

  describe("ListGroupItem", () => {
    it("should be able to render a list group item", () => {
      class Renderer extends Component {
        render() {
          return h(ListGroupItem, {});
        }
      }

      const app = mount(Renderer, document.createElement("div"));
      const el = app._el as HTMLElement;
      expect(el.nodeName).toBe("LI");
      expect(el.className).toBe("list-group-item");
    });
  });
});
