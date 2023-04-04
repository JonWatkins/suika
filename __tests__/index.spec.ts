import * as Suika from "../src/index";
import { vElement } from "../src/vdom";

describe("Suika", () => {
  describe("Suika.h", () => {
    it("should be a function", () => {
      expect(typeof Suika.h).toEqual("function");
    });

    it("should be able to make a vNode", () => {
      const vDomNode = Suika.h("div", {}) as vElement;
      expect(vDomNode.tag).toEqual("div");
    });
  });
});
