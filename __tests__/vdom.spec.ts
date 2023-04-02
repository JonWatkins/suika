import * as vdom from "../src/vdom";
import { Component } from "../src/Component";

describe("vdom", () => {
  it("should be a function", () => {
    expect(typeof vdom.h).toEqual("function");
  });

  it("should have a fragment function", () => {
    expect(typeof vdom.Fragment).toBe("function");
  });

  it("Fragment function should always return teh same value", () => {
    expect(vdom.Fragment()).toEqual(vdom.Fragment());
  });

  it("should have a method to check if a tag is a fragment", () => {
    expect(vdom.isFragment(vdom.Fragment)).toEqual(true);
  });

  it("should have a funciton to make a element", () => {
    const el = vdom.createElement("div", { id: 1 }, ["hi"]);
    expect(el.kind).toEqual("element");
    expect(el.attrs.id).toEqual(1);
    expect(el.children.length).toEqual(1);
  });

  it("should have a funciton to make an fragment", () => {
    const el = vdom.createFragment(["hi"]);
    expect(el.kind).toEqual("fragment");
    expect(el.children.length).toEqual(1);
  });

  it("should have a function to make a text node", () => {
    const el = vdom.createText("hi");
    expect(el.kind).toEqual("text");
    expect(el.value).toEqual("hi");
  });

  it("should be able to make a vDomNode", () => {
    // @ts-ignore:next-line
    const vDomNode = vdom.h("div");
    // @ts-ignore:next-line
    expect(vDomNode.tag).toEqual("div");
    // @ts-ignore:next-line
    expect(Array.isArray(vDomNode.children)).toEqual(true);
    expect(vDomNode.kind).toEqual("element");
  });

  it("should be able to make a fragment", () => {
    // @ts-ignore:next-line
    const vDomNode = vdom.h(vdom.Fragment);
    expect(vDomNode.kind).toEqual("fragment");
  });

  it("should be able to apply atributes to a vDomNode", () => {
    const vDomNode = vdom.h("div", { id: "container", onclick: () => {} });
    // @ts-ignore:next-line
    expect(vDomNode.attrs.id).toEqual("container");
    // @ts-ignore:next-line
    expect(typeof vDomNode.attrs.onclick).toEqual("function");
  });

  it("should be able to make child vDomNodes", () => {
    const vDomNode = vdom.h("div", {}, vdom.h("h1", {}));

    // @ts-ignore:next-line
    expect(vDomNode.children.length).toEqual(1);
    // @ts-ignore:next-line
    expect(vDomNode.children[0].tag).toEqual("h1");
  });

  it("should handle multiple child nodes", () => {
    const vDomNode = vdom.h("div", {}, vdom.h("h1", {}), vdom.h("p", {}));

    // @ts-ignore:next-line
    expect(vDomNode.children.length).toEqual(2);
    // @ts-ignore:next-line
    expect(vDomNode.children[0].tag).toEqual("h1");
    // @ts-ignore:next-line
    expect(vDomNode.children[1].tag).toEqual("p");
  });

  it("should be able to make vDomText nodes", () => {
    // @ts-ignore:next-line
    const vDomNode = vdom.h("div", {}, "Hello World");

    // @ts-ignore:next-line
    expect(vDomNode.children.length).toEqual(1);
    // @ts-ignore:next-line
    expect(vDomNode.children[0].value).toEqual("Hello World");
    // @ts-ignore:next-line
    expect(vDomNode.children[0].kind).toEqual("text");
  });

  it("should be able to make vDomComponent nodes", () => {
    // @ts-ignore:next-line
    class Ctx extends Component {
      constructor() {
        super();
      }
    }
    // @ts-ignore:next-line
    const vDomNode = vdom.h(Ctx);
    expect(vDomNode.kind).toEqual("component");
  });

  it("should be able to apply attributes vDomComponent nodes", () => {
    // @ts-ignore:next-line
    class Ctx extends Component {
      constructor() {
        super();
      }
    }
    const vDomNode = vdom.h(Ctx, { id: "container" });
    expect(vDomNode.kind).toEqual("component");
    // @ts-ignore:next-line
    expect(vDomNode.attrs.id).toEqual("container");
  });

  it("should create a text node if the tag is not a reserved tag", () => {
    // @ts-ignore:next-line
    const vDomNode = vdom.h("Hello");
    expect(vDomNode.kind).toEqual("text");
  });
});
