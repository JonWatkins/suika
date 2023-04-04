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

  it("should have a function to make a text node", () => {
    const el = vdom.createText("hi");
    expect(el.kind).toEqual("text");
    expect(el.value).toEqual("hi");
  });

  it("should have a funciton to make a element", () => {
    const el = vdom.createElement("div", { id: 1 }, [vdom.createText("hi")]);
    expect(el.kind).toEqual("element");
    expect(el.attrs.id).toEqual(1);
    expect(el.children.length).toEqual(1);
  });

  it("should have a funciton to make an fragment", () => {
    const el = vdom.createFragment([
      vdom.createElement("h1", {}, [vdom.createText("hello")]),
    ]);
    expect(el.kind).toEqual("fragment");
    expect(el.children.length).toEqual(1);
  });

  it("should be able to make a vDomNode", () => {
    const vDomNode = vdom.h("div") as vdom.vElement;
    expect(vDomNode.tag).toEqual("div");
    expect(Array.isArray(vDomNode.children)).toEqual(true);
    expect(vDomNode.kind).toEqual("element");
  });

  it("should be able to make a fragment", () => {
    const vDomNode = vdom.h(vdom.Fragment);
    expect(vDomNode.kind).toEqual("fragment");
  });

  it("should be able to apply atributes to a vDomNode", () => {
    const vDomNode = vdom.h("div", {
      id: "container",
      onclick: () => {},
    }) as vdom.vElement;
    expect(vDomNode.attrs.id).toEqual("container");
    expect(typeof vDomNode.attrs.onclick).toEqual("function");
  });

  it("should be able to make child vDomNodes", () => {
    const vDomNode = vdom.h("div", {}, vdom.h("h1", {})) as vdom.vElement;

    expect(vDomNode.children.length).toEqual(1);
    expect(vDomNode.children[0].tag).toEqual("h1");
  });

  it("should handle multiple child nodes", () => {
    const vDomNode = vdom.h(
      "div",
      {},
      vdom.h("h1", {}),
      vdom.h("p", {})
    ) as vdom.vElement;

    expect(vDomNode.children.length).toEqual(2);
    expect(vDomNode.children[0].tag).toEqual("h1");
    expect(vDomNode.children[1].tag).toEqual("p");
  });

  it("should be able to make vDomText nodes", () => {
    const vDomNode = vdom.h("div", {}, "Hello World") as vdom.vElement;
    const childNode = vDomNode.children[0] as vdom.vText;

    expect(vDomNode.children.length).toEqual(1);
    expect(childNode.value).toEqual("Hello World");
    expect(childNode.kind).toEqual("text");
  });

  it("should be able to make vDomComponent nodes", () => {
    class Ctx extends Component {
      constructor() {
        super();
      }
      render() {
        return vdom.h("div");
      }
    }
    const vDomNode = vdom.h(Ctx);
    expect(vDomNode.kind).toEqual("component");
  });

  it("should be able to apply attributes vDomComponent nodes", () => {
    class Ctx extends Component {
      constructor() {
        super();
      }
      render() {
        return vdom.h("div", { id: this.attrs.id });
      }
    }
    const vDomNode = vdom.h(Ctx, { id: "container" });
    expect(vDomNode.kind).toEqual("component");
    expect(vDomNode.attrs.id).toEqual("container");
  });

  it("should create a text node if the tag is not a reserved tag", () => {
    const vDomNode = vdom.h("Hello");
    expect(vDomNode.kind).toEqual("text");
  });
});
