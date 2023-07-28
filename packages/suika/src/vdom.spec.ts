import { createElement } from "./vdom";

describe("vdom", () => {
  describe("createElement", () => {
    it("should be able to create an element", () => {
      const el = createElement("h1", {}, "Hello World");
      expect(el.type).toEqual("h1");
    });
  });
});
