import { Component, mount, h } from "suika";
import {
  Card,
  CardHeader,
  CardTitle,
  CardBody,
  CardFooter,
} from "../src/components/Cards";

describe("Cards", () => {
  describe("Card", () => {
    it("should be able to render a card", () => {
      class Renderer extends Component {
        render() {
          return h(Card, {});
        }
      }

      const app = mount(Renderer, document.createElement("div"));
      const el = app._el as HTMLElement;
      expect(el.nodeName).toBe("DIV");
      expect(el.className).toBe("card");
      expect(el.outerHTML).toMatchSnapshot();
    });
  });

  describe("CardHeader", () => {
    it("should be able to render a card header", () => {
      class Renderer extends Component {
        render() {
          return h(CardHeader, {});
        }
      }

      const app = mount(Renderer, document.createElement("div"));
      const el = app._el as HTMLElement;
      expect(el.nodeName).toBe("DIV");
      expect(el.className).toBe("card-header");
      expect(el.outerHTML).toMatchSnapshot();
    });
  });

  describe("CardTitle", () => {
    it("should be able to render a card header", () => {
      class Renderer extends Component {
        render() {
          return h(CardTitle, {});
        }
      }

      const app = mount(Renderer, document.createElement("div"));
      const el = app._el as HTMLElement;
      expect(el.nodeName).toBe("H1");
      expect(el.className).toBe("card-title");
      expect(el.outerHTML).toMatchSnapshot();
    });
  });

  describe("CardBody", () => {
    it("should be able to render a card body", () => {
      class Renderer extends Component {
        render() {
          return h(CardBody, {});
        }
      }

      const app = mount(Renderer, document.createElement("div"));
      const el = app._el as HTMLElement;
      expect(el.nodeName).toBe("DIV");
      expect(el.className).toBe("card-body");
      expect(el.outerHTML).toMatchSnapshot();
    });
  });

  describe("CardFooter", () => {
    it("should be able to render a card footer", () => {
      class Renderer extends Component {
        render() {
          return h(CardFooter, {});
        }
      }

      const app = mount(Renderer, document.createElement("div"));
      const el = app._el as HTMLElement;
      expect(el.nodeName).toBe("DIV");
      expect(el.className).toBe("card-footer");
      expect(el.outerHTML).toMatchSnapshot();
    });
  });
});
