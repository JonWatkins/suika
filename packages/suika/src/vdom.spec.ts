// @ts-nocheck

import { createElement, createTextElement } from "./vdom";

describe("vdom", () => {
  describe("createTextElement", () => {
    it("Should be able to create a text element", () => {
      const el = createTextElement("Hello World");
      expect(el.props.nodeValue).toEqual("Hello World");
    });
  });

  describe("createElement", () => {
    it("should be able to create an element", () => {
      const el = createElement("h1", { id: "test" }, "Hello World");
      expect(el.type).toEqual("h1");
      expect(el.props.id).toEqual("test");
      expect(el.props.children[0].type).toEqual("TEXT_ELEMENT");
      expect(el.props.children[0].props.nodeValue).toEqual("Hello World");
      expect(el.props.children.length).toEqual(1);
    });

    it("should be able to use a functional component", () => {
      const Component = ({ greeting }) =>
        createElement("h1", { id: "test" }, greeting);

      const el = createElement(
        "div",
        { className: "test" },
        Component({ greeting: "Hello World" }),
      );
      expect(el.type).toEqual("div");
      expect(el.props.className).toEqual("test");
      expect(el.props.children[0].type).toEqual("h1");
    });
  });
});
