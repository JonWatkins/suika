import { h } from "../../src/vdom/h";
import { diff } from "../../src/vdom/diff";
import { render } from "../../src/vdom/render";
import { Component } from "../../src/core/Component";

describe("diff", () => {
  it("should return a replace patch when the oldVnode is null", () => {
    const oldVnode = null;
    const newVnode = h("div", {});
    const patches = diff(oldVnode, newVnode);
    expect(typeof patches).toBe("object");
    expect(patches.kind).toEqual("replace");
    // @ts-ignore:next-line
    expect(patches.newNode).toEqual(newVnode);
  });

  it("should return a skip patch when given 2 string vDomNoodes with the same value", () => {
    // @ts-ignore:next-line
    const oldVnode = h("hello");
    // @ts-ignore:next-line
    const newVnode = h("hello");
    const patches = diff(oldVnode, newVnode);
    expect(typeof patches).toBe("object");
    expect(patches.kind).toEqual("skip");
  });

  it("should return a replace patch when given 2 string vDomNoodes with differnt values", () => {
    // @ts-ignore:next-line
    const oldVnode = h("hello");
    // @ts-ignore:next-line
    const newVnode = h("hello world");
    const patches = diff(oldVnode, newVnode);
    expect(typeof patches).toBe("object");
    expect(patches.kind).toEqual("replace");
    // @ts-ignore:next-line
    expect(patches.newNode).toEqual(newVnode);
  });

  it("should return a replace patch if the new node is text", () => {
    const oldVnode = h("div", {});
    // @ts-ignore:next-line
    const newVnode = h("hello world");
    const patches = diff(oldVnode, newVnode);
    expect(typeof patches).toBe("object");
    expect(patches.kind).toEqual("replace");
    // @ts-ignore:next-line
    expect(patches.newNode).toEqual(newVnode);
  });

  it("should return a replace patch if the tags are different", () => {
    const oldVnode = h("div", {});
    const newVnode = h("h1", {});
    const patches = diff(oldVnode, newVnode);
    expect(typeof patches).toBe("object");
    expect(patches.kind).toEqual("replace");
    // @ts-ignore:next-line
    expect(patches.newNode).toEqual(newVnode);
  });

  it("should be able return an update patch", () => {
    const oldVnode = h("div", {});
    const newVnode = h("div", {});
    const patches = diff(oldVnode, newVnode);
    expect(typeof patches).toBe("object");
    expect(patches.kind).toEqual("update");
    // @ts-ignore:next-line
    expect(typeof patches.attrs).toEqual("object");
  });

  it("should return patches for keyd child nodes", () => {
    const oldVnode = h(
      "div",
      {},
      // @ts-ignore:next-line
      h("h1", { key: 1 }, "Hello"),
      // @ts-ignore:next-line
      h("p", { key: 2 }, "Lorem ipsum"),
      // @ts-ignore:next-line
      h("p", { key: 3 }, "Lorem ipsum")
    );
    const newVnode = h(
      "div",
      {},
      // @ts-ignore:next-line
      h("h1", { key: 1 }, "Hello"),
      // @ts-ignore:next-line
      h("p", { key: 2 }, "Lorem ipsum")
    );

    const patches = diff(oldVnode, newVnode);
    expect(typeof patches).toBe("object");
    expect(patches.kind).toEqual("update");
    // @ts-ignore:next-line
    expect(typeof patches.attrs).toEqual("object");
  });

  it("should handle child component nodes", () => {
    const componentVdom = h(
      "div",
      { key: 2 },
      // @ts-ignore:next-line
      h("h1", { key: 3 }, "Hello World")
    );

    // @ts-ignore:next-line
    class Ctx extends Component {
      constructor() {
        super();
      }
      _initProps() {
        return componentVdom;
      }
      _setProps() {
        return componentVdom;
      }
      _initState() {}
      _notifyMounted() {}
    }

    const newVnode = h("div", {}, h(Ctx, { key: 1 }));

    // need to call render on vDomNode to create the
    // instance
    render(newVnode);

    const patches = diff(newVnode, newVnode);
    expect(typeof patches).toBe("object");
    expect(patches.kind).toEqual("update");
    // @ts-ignore:next-line
    expect(typeof patches.attrs).toEqual("object");
  });

  it("should handle different child component nodes", () => {
    const component1Vdom = h(
      "div",
      { key: 2 },
      // @ts-ignore:next-line
      h("h1", { key: 3 }, "Hello World")
    );
    const component2Vdom = h(
      "div",
      { key: 4 },
      // @ts-ignore:next-line
      h("h2", { key: 5 }, "Hello World!")
    );

    // @ts-ignore:next-line
    class Ctx1 extends Component {
      constructor() {
        super();
      }
      _initProps() {
        return component1Vdom;
      }
      _setProps() {
        return component1Vdom;
      }
      _initState() {}
      _notifyMounted() {}
      _unmount() {}
    }

    // @ts-ignore:next-line
    class Ctx2 extends Component {
      constructor() {
        super();
      }
      _initProps() {
        return component2Vdom;
      }
      _setProps() {
        return component2Vdom;
      }
      _initState() {}
      _notifyMounted() {}
      _unmount() {}
    }

    const oldVnode = h("div", {}, h(Ctx1, { key: 1 }));

    const newVnode = h("div", {}, h(Ctx2, { key: 6 }));

    // need to call render on vDomNode to create the
    // instance
    render(oldVnode);
    render(newVnode);

    const patches = diff(oldVnode, newVnode);
    expect(typeof patches).toBe("object");
    expect(patches.kind).toEqual("update");
    // @ts-ignore:next-line
    expect(typeof patches.attrs).toEqual("object");
  });
});
