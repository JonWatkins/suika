import { Component } from "../../src/core/Component";
import { h } from "../../src/vdom/h";
import { render } from "../../src/vdom/render";
import { diff } from "../../src/vdom/diff";
import { patch } from "../../src/vdom/patch";

describe("Component", () => {
  it("Should be able to make a simpe component", () => {
    // @ts-ignore:next-line
    class Ctx extends Component {
      constructor() {
        super();
      }
      render() {
        return h("div", {}, "Hi");
      }
    }

    const ctx = new Ctx();

    // @ts-ignore:next-line
    expect(typeof ctx._uid).toBe("number");
  });

  it("should have a function to get a new diff", () => {
    // @ts-ignore:next-line
    class Ctx extends Component {
      constructor() {
        super();
      }
      render() {
        return h("div", {}, "Hi");
      }
    }

    const ctx = new Ctx();

    // @ts-ignore:next-line
    const patches = ctx._getDiff();

    expect(patches.kind).toEqual("replace");
  });

  it("should return update diff if the vDomNode has not changed", () => {
    // @ts-ignore:next-line
    class Ctx extends Component {
      constructor() {
        super();
      }
      render() {
        return h("div", {}, "Hi");
      }
    }

    const ctx = new Ctx();

    // @ts-ignore:next-line
    ctx._vNode = ctx.render();

    // @ts-ignore:next-line
    const patches = ctx._getDiff();

    expect(patches.kind).toEqual("update");
  });

  it("should be able to render a component", () => {
    // @ts-ignore:next-line
    class Ctx extends Component {
      constructor() {
        super();
      }
      render() {
        return h("div", {}, h("h1", {}, "Hello World"));
      }
    }

    const el = render(h(Ctx, { key: 0 }));

    // @ts-ignore:next-line
    expect(el.outerHTML).toMatchSnapshot();
  });

  it("should be able to patch a component", (done) => {
    let el;

    // @ts-ignore:next-line
    class Ctx extends Component {
      constructor() {
        super();
      }
      render() {
        return h("div", {}, h("h1", {}, "Hello World"));
      }
      onUpdated() {
        expect(el.outerHTML).toMatchSnapshot();
        done();
      }
    }

    const vDom = h(Ctx, { key: 0 });

    el = render(vDom);

    // @ts-ignore:next-line
    vDom.instance._update();
  });

  it("calls the onUpdated function when state changes", () => {
    let el;

    // @ts-ignore:next-line
    class Ctx extends Component {
      constructor() {
        super();
      }
      state = {
        greeting: "Hello",
      };
      render() {
        return h("div", {}, h("h1", {}, this.state.greeting));
      }
    }

    const vDom = h(Ctx, { key: 0 });

    el = render(vDom);

    expect(el.outerHTML).toMatchSnapshot();

    // @ts-ignore:next-line
    vDom.instance.state.greeting = "World";

    // @ts-ignore:next-line
    expect(el.outerHTML).toMatchSnapshot();
  });

  it("should patch a component when the state changes", (done) => {
    let el;

    // @ts-ignore:next-line
    class Ctx extends Component {
      constructor() {
        super();
      }
      state = {
        greeting: "Hello",
      };
      render() {
        return h("div", {}, h("h1", {}, this.state.greeting));
      }
      onUpdated() {
        expect(el.outerHTML).toMatchSnapshot();
        done();
      }
    }

    const vDom = h(Ctx, { key: 0 });

    el = render(vDom);

    expect(el.outerHTML).toMatchSnapshot();

    // @ts-ignore:next-line
    vDom.instance.state.greeting = "World";
  });

  it("should not patch a component when the _el is not defined", () => {
    let el;
    const spy = jest.fn();

    // @ts-ignore:next-line
    class Ctx extends Component {
      constructor() {
        super();
      }
      state = {
        greeting: "Hello",
      };
      render() {
        return h("div", {}, h("h1", {}, this.state.greeting));
      }
      onUpdated() {
        spy();
      }
    }

    const vDom = h(Ctx, { key: 0 });

    el = render(vDom);

    expect(el.outerHTML).toMatchSnapshot();

    // @ts-ignore:next-line
    vDom.instance._el = null;
    // @ts-ignore:next-line
    vDom.instance.state.greeting = "World";

    expect(spy).toHaveBeenCalledTimes(0);
  });

  it("the _unmount function should be called when unmounting a child component", () => {
    // @ts-ignore:next-line
    class Ctx extends Component {
      constructor() {
        super();
      }
      render() {
        return h("div", {}, h("h1", {}, "Hello World"));
      }
    }

    const oldVnode = h("div", { key: 1 }, h(Ctx, { key: 2 }));

    const newVnode = h("div", { key: 1 });

    let el = render(oldVnode);

    // @ts-ignore:next-line
    expect(el.outerHTML).toMatchSnapshot();

    const patches = diff(oldVnode, newVnode);

    patch(el, patches);

    // @ts-ignore:next-line
    expect(el.outerHTML).toMatchSnapshot();
  });

  it("the beforeUnmount function should be called when unmounting a child component", () => {
    const spy = jest.fn();

    // @ts-ignore:next-line
    class Ctx extends Component {
      constructor() {
        super();
      }
      render() {
        return h("div", {}, h("h1", {}, "Hello World"));
      }
      beforeUnmount() {
        spy();
      }
    }

    const oldVnode = h("div", { key: 1 }, h(Ctx, { key: 2 }));

    const newVnode = h("div", { key: 1 });

    let el = render(oldVnode);

    // @ts-ignore:next-line
    expect(el.outerHTML).toMatchSnapshot();

    const patches = diff(oldVnode, newVnode);

    patch(el, patches);

    // @ts-ignore:next-line
    expect(el.outerHTML).toMatchSnapshot();

    expect(spy).toHaveBeenCalledTimes(1);
  });

  it("should change the _el when replacing the root node", () => {
    // @ts-ignore:next-line
    class Ctx extends Component {
      constructor() {
        super();
      }
      render() {
        return h("div", {}, h("h1", {}, "Hello World"));
      }
    }

    const vDom = h(Ctx, { key: 0 });
    const el = render(vDom);

    // @ts-ignore:next-line
    expect(el.outerHTML).toMatchSnapshot();

    // @ts-ignore:next-line
    vDom.instance.render = () => {
      return h("h1", {}, "Hello World");
    };

    // @ts-ignore:next-line
    vDom.instance._update();

    // @ts-ignore:next-line
    expect(el.outerHTML).toMatchSnapshot();

    // @ts-ignore:next-line
    expect(el === vDom.instance._el).toBe(false);
  });
});
