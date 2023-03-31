import * as NekoJs from "../src/index";

describe("NekoJS", () => {
  describe("NekoJS.h", () => {
    it("should be a function", () => {
      expect(typeof NekoJs.h).toEqual("function");
    });

    it("should be able to make a vNode", () => {
      const vDomNode = NekoJs.h("div", {});
      // @ts-ignore:next-line
      expect(vDomNode.tag).toEqual("div");
    });
  });
});
