import { h } from "../../src/vdom/h";
import { diff } from "../../src/vdom/diff";
import { render } from "../../src/vdom/render";
import { patch } from "../../src/vdom/patch";

describe("patch", () => {
  it("should handle a skip patch", () => {
    // @ts-ignore:next-line
    const newVnode = h("hello");
    const el = render(newVnode);
    const patches = diff(newVnode, newVnode);

    patch(el, patches);
    expect(el.textContent).toEqual("hello");
  });

  it("should be able to apply a simple replace patch", () => {
    const newVnode = h("div", { key: 1 });
    const el = render(newVnode);
    const patches = diff(null, newVnode);

    patch(el, patches);
  });

  it("should be able to apply a simple replace patch with a callback", (done) => {
    const newVnode = h("div", { key: 1 });
    let el = render(newVnode);
    const patches = diff(null, newVnode);

    // @ts-ignore:next-line
    patches.callback = (newEl) => {
      el = newEl;
      done();
    };

    patch(el, patches);
  });

  it("should be able to apply a simple update patch", () => {
    const oldVnode = h("div", {
      key: 1,
      id: "container",
      className: "d-block",
    });
    const newVnode = h("div", { key: 1, id: "container", className: "d-none" });
    const el = render(oldVnode);
    const patches = diff(oldVnode, newVnode);

    // @ts-ignore:next-line
    expect(el.className).toEqual("d-block");
    // @ts-ignore:next-line
    expect(el.outerHTML).toMatchSnapshot();
    patch(el, patches);
    // @ts-ignore:next-line
    expect(el.className).toEqual("d-none");
    // @ts-ignore:next-line
    expect(el.outerHTML).toMatchSnapshot();
  });

  it("should be able to patch child nodes", () => {
    const oldVnode = h(
      "div",
      { key: 1 },
      h("h1", { key: 2, id: "title" }, "Hello")
    );

    const newVnode = h(
      "div",
      { key: 1 },
      h("h1", { key: 2, id: "title" }, "World")
    );

    const el = render(oldVnode);
    const patches = diff(oldVnode, newVnode);

    // @ts-ignore:next-line
    expect(el.firstChild.textContent).toBe("Hello");
    // @ts-ignore:next-line
    expect(el.outerHTML).toMatchSnapshot();
    patch(el, patches);
    // @ts-ignore:next-line
    expect(el.firstChild.textContent).toBe("World");
    // @ts-ignore:next-line
    expect(el.outerHTML).toMatchSnapshot();
  });

  it("should skip child text nodes if the value is the same", () => {
    const oldVnode = h("div", { key: 1 }, h("h1", { key: 2 }, "hello"));
    const newVnode = h("div", { key: 1 }, h("h1", { key: 2 }, "hello"));
    const el = render(oldVnode);
    const patches = diff(oldVnode, newVnode);
    // @ts-ignore:next-line
    expect(el.outerHTML).toMatchSnapshot();
    patch(el, patches);
    // @ts-ignore:next-line
    expect(el.outerHTML).toMatchSnapshot();
  });

  it("should remove old child nodes", () => {
    const oldVnode = h("div", { key: 1 }, h("h1", { key: 2 }, "hello"));
    const newVnode = h("div", { key: 1 });
    const el = render(oldVnode);
    const patches = diff(oldVnode, newVnode);
    // @ts-ignore:next-line
    expect(el.outerHTML).toMatchSnapshot();
    patch(el, patches);
    // @ts-ignore:next-line
    expect(el.outerHTML).toMatchSnapshot();
  });

  it("should insert new child nodes", () => {
    const oldVnode = h("div", { key: 1 });
    const newVnode = h("div", { key: 1 }, h("h1", { key: 2 }, "hello"));
    const el = render(oldVnode);
    const patches = diff(oldVnode, newVnode);
    // @ts-ignore:next-line
    expect(el.outerHTML).toMatchSnapshot();
    patch(el, patches);
    // @ts-ignore:next-line
    expect(el.outerHTML).toMatchSnapshot();
  });
});
