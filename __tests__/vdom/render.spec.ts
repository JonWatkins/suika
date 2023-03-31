import { h } from "../../src/vdom/h";
import { render } from "../../src/vdom/render";

describe("render", () => {
  it("Should render a basic vDomNode", () => {
    // @ts-ignore:next-line
    const vDomNode = h("div");
    const el = render(vDomNode);
    expect(el.nodeName).toEqual("DIV");
    // @ts-ignore:next-line
    expect(el.outerHTML).toMatchSnapshot();
  });

  it("should be able to render text nodes", () => {
    // @ts-ignore:next-line
    const vDomNode = h("hello");
    const el = render(vDomNode);
    expect(el.nodeName).toEqual("#text");
    expect(el.textContent).toMatchSnapshot();
  });

  it("should be able to add attributes to a node", () => {
    const vDomNode = h("div", { id: "container" });
    const el = render(vDomNode);
    expect(el.nodeName).toEqual("DIV");
    // @ts-ignore:next-line
    expect(el.id).toEqual("container");
    // @ts-ignore:next-line
    expect(el.outerHTML).toMatchSnapshot();
  });

  it("should be able to add event handlers to a node", () => {
    const spy = jest.fn();
    const vDomNode = h("button", { onclick: spy });
    const el = render(vDomNode);

    // @ts-ignore:next-line
    el.click();
    expect(spy).toHaveBeenCalled();
    // @ts-ignore:next-line
    expect(el.outerHTML).toMatchSnapshot();
  });

  it("should be able to render child nodes", () => {
    const vDomNode = h("div", { id: "container" }, h("h1", {}, "Hello World"));
    const el = render(vDomNode);
    // @ts-ignore:next-line
    expect(el.outerHTML).toMatchSnapshot();
  });

  it("should be able to render a component", () => {
    const mountedSpy = jest.fn();
    const stateSpy = jest.fn();
    class Component {
      _initProps() {
        return h("div", {}, h("h1", {}, "Hello World"));
      }
      _initState() {
        stateSpy();
      }
      _notifyMounted() {
        mountedSpy();
      }
    }
    // @ts-ignore:next-line
    const vDomNode = h(Component);
    const el = render(vDomNode);
    expect(stateSpy).toHaveBeenCalled();
    expect(mountedSpy).toHaveBeenCalled();
    // @ts-ignore:next-line
    expect(el.outerHTML).toMatchSnapshot();
  });

  it("should be able to render a component that already has an instance", () => {
    const mountedSpy = jest.fn();
    class Component {
      render() {
        return h("div", {}, h("h1", {}, "Hello World"));
      }
      _notifyMounted() {
        mountedSpy();
      }
    }
    // @ts-ignore:next-line
    const vDomNode = h(Component);
    // @ts-ignore:next-line
    vDomNode.instance = new vDomNode.component();

    const el = render(vDomNode);
    expect(mountedSpy).toHaveBeenCalled();
    // @ts-ignore:next-line
    expect(el.outerHTML).toMatchSnapshot();
  });
});
