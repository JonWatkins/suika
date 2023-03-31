import { h } from "../../src/vdom/h";

describe("h", () => {
  it("should be a function", () => {
    expect(typeof h).toEqual("function");
  });

  it("should be able to make a vDomNode", () => {
    // @ts-ignore:next-line
    const vDomNode = h("div");
    // @ts-ignore:next-line
    expect(vDomNode.tag).toEqual("div");
    // @ts-ignore:next-line
    expect(typeof vDomNode.attrs).toEqual("object");
    expect(Array.isArray(vDomNode.children)).toEqual(true);
    expect(vDomNode.kind).toEqual("element");
  });

  it("should be able to apply atributes to a vDomNode", () => {
    const vDomNode = h("div", { id: "container", onclick: () => {} });
    // @ts-ignore:next-line
    expect(vDomNode.attrs.id).toEqual("container");
    // @ts-ignore:next-line
    expect(typeof vDomNode.attrs.onclick).toEqual("function");
  });

  it("should be able to add a key to a vDomNode", () => {
    const vDomNode = h("div", { key: "root" });
    expect(vDomNode.key).toEqual("root");
  });

  it("should be able to make child vDomNodes", () => {
    const vDomNode = h("div", {}, h("h1", {}));

    expect(vDomNode.children.length).toEqual(1);
    // @ts-ignore:next-line
    expect(vDomNode.children[0].tag).toEqual("h1");
  });

  it("should handle multiple child nodes", () => {
    const vDomNode = h("div", {}, h("h1", {}), h("p", {}));

    expect(vDomNode.children.length).toEqual(2);
    // @ts-ignore:next-line
    expect(vDomNode.children[0].tag).toEqual("h1");
    // @ts-ignore:next-line
    expect(vDomNode.children[1].tag).toEqual("p");
  });

  it("should be able to make vDomText nodes", () => {
    const vDomNode = h("div", {}, "Hello World");

    expect(vDomNode.children.length).toEqual(1);
    // @ts-ignore:next-line
    expect(vDomNode.children[0].value).toEqual("Hello World");
    expect(vDomNode.children[0].kind).toEqual("text");
  });

  it("should be able to make vDomComponent nodes", () => {
    class Component {}
    // @ts-ignore:next-line
    const vDomNode = h(Component);
    expect(vDomNode.kind).toEqual("component");
  });

  it("should be able to apply attributes vDomComponent nodes", () => {
    class Component {}
    const vDomNode = h(Component, { id: "container" });
    expect(vDomNode.kind).toEqual("component");
    // @ts-ignore:next-line
    expect(vDomNode.attrs.id).toEqual("container");
  });

  it("should create a text node if the tag is not a reserved tag", () => {
    // @ts-ignore:next-line
    const vDomNode = h("Hello");
    expect(vDomNode.kind).toEqual("text");
  });
});
