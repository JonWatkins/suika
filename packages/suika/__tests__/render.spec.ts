import { h, vComponent } from "../src/vdom";
import { render, mount } from "../src/render";
import { Component } from "../src/Component";

describe("render", () => {
  it("Should render a basic vDomNode", () => {
    const vDomNode = h("div");
    const el = render(vDomNode) as HTMLElement;
    expect(el.nodeName).toEqual("DIV");
    expect(el.outerHTML).toMatchSnapshot();
  });

  it("should be able to render text nodes", () => {
    const vDomNode = h("hello");
    const el = render(vDomNode);
    expect(el.nodeName).toEqual("#text");
    expect(el.textContent).toMatchSnapshot();
  });

  it("should be able to add attributes to a node", () => {
    const vDomNode = h("div", { id: "container" });
    const el = render(vDomNode) as HTMLElement;
    expect(el.nodeName).toEqual("DIV");
    expect(el.id).toEqual("container");
    expect(el.outerHTML).toMatchSnapshot();
  });

  it("should be able to add event handlers to a node", () => {
    const spy = jest.fn();
    const vDomNode = h("button", { onclick: spy });
    const el = render(vDomNode) as HTMLElement;

    el.click();
    expect(spy).toHaveBeenCalled();
    expect(el.outerHTML).toMatchSnapshot();
  });

  it("should be able to render child nodes", () => {
    const vDomNode = h("div", { id: "container" }, h("h1", {}, "Hello World"));
    const el = render(vDomNode) as HTMLElement;
    expect(el.outerHTML).toMatchSnapshot();
  });

  it("should be able to handle function child nodes", () => {
    const Title = ({ id, text }) => h("h1", { id }, text);
    const el = render(h(Title, { id: "1", text: "hello" })) as HTMLElement;
    expect(el.outerHTML).toMatchSnapshot();
  });

  it("should be able to render a component", () => {
    const mountedSpy = jest.fn();
    const stateSpy = jest.fn();

    class Ctx extends Component {
      constructor() {
        super();
      }
      render() {
        return h("div", {}, h("h1", {}, "Hello World"));
      }
      _initState() {
        stateSpy();
      }
      _notifyMounted() {
        mountedSpy();
      }
    }
    const vDomNode = h(Ctx);
    const el = render(vDomNode) as HTMLElement;
    expect(stateSpy).toHaveBeenCalled();
    expect(mountedSpy).toHaveBeenCalled();
    expect(el.outerHTML).toMatchSnapshot();
  });

  it("should be able to render a component that already has an instance", () => {
    const mountedSpy = jest.fn();
    class Ctx extends Component {
      constructor() {
        super();
      }
      render() {
        return h("div", {}, h("h1", {}, "Hello World"));
      }
      _notifyMounted() {
        mountedSpy();
      }
    }
    const vDomNode = h(Ctx) as vComponent;
    vDomNode.instance = new vDomNode.component();

    const el = render(vDomNode) as HTMLElement;
    expect(mountedSpy).toHaveBeenCalled();
    expect(el.outerHTML).toMatchSnapshot();
  });

  describe("mount", () => {
    it("should error if not given a component", () => {
      expect(() => {
        // @ts-ignore:next-line
        mount();
      }).toThrow();
    });

    it("should throw if the element is not a dom not", () => {
      class App extends Component {
        render() {
          return h("h1", {}, "hi");
        }
      }

      expect(() => {
        // @ts-ignore:next-line
        mount(App);
      }).toThrow();
    });

    it("should be able to mount a component", () => {
      class App extends Component {
        render() {
          return h("h1", {}, "hi");
        }
      }

      const instance = mount(App, document.createElement("div"));
      expect(instance._isSuika).toBe(true);
    });
  });

  it("should be able too dangerously set html", () => {
    class App extends Component {
      render() {
        return h("div", {
          id: "page",
          dangerouslySetHtml: { __html: "<h1>Hello World</h1>" },
        });
      }
    }
    const instance = mount(App, document.createElement("div"));
    const el = instance._el as HTMLElement;

    expect(instance._isSuika).toBe(true);
    expect(el.outerHTML).toMatchSnapshot();
    expect(el.firstChild?.nodeName).toBe("H1");
    expect(el.firstChild?.textContent).toBe("Hello World");
  });
});
