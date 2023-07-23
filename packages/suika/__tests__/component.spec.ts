import { Component } from "../src/Component";
import { h, vComponent } from "../src/vdom";
import { render, mount } from "../src/render";
import { diff } from "../src/diff";
import { Reactive } from "../src/Reactive";

describe("Component", () => {
  it("Should be able to make a simpe component", () => {
    class Ctx extends Component {
      render() {
        return h("div", {}, "Hi");
      }
    }

    const ctx = new Ctx();

    expect(typeof ctx._uid).toBe("number");
  });

  it("should have a function to get a new diff", () => {
    class Ctx extends Component {
      render() {
        return h("div", {}, "Hi");
      }
    }

    const ctx = new Ctx();

    const patches = ctx._getDiff();

    expect(typeof patches).toEqual("function");
  });

  it("should be able to render a component", () => {
    class Ctx extends Component {
      render() {
        return h("div", {}, h("h1", {}, "Hello World"));
      }
    }

    const el = render(h(Ctx, {})) as HTMLElement;

    expect(el.outerHTML).toMatchSnapshot();
  });

  it("should be able to patch a component", (done) => {
    let el;

    class Ctx extends Component {
      render() {
        return h("div", {}, h("h1", {}, "Hello World"));
      }
      onUpdated() {
        expect(el.outerHTML).toMatchSnapshot();
        done();
      }
    }

    const vDom = h(Ctx, {}) as vComponent;
    el = render(vDom);
    const instance = vDom.instance as Component;

    instance._update();
  });

  it("calls the onUpdated function when state changes", () => {
    let el;

    class Ctx extends Component {
      state = {
        greeting: "Hello",
      };
      render() {
        // @ts-ignore:next-line
        return h("div", {}, h("h1", {}, this.state.value.greeting));
      }
    }

    const vDom = h(Ctx, {}) as vComponent;
    el = render(vDom);
    const instance = vDom.instance as Component;

    expect(el.outerHTML).toMatchSnapshot();

    instance.state.value.greeting = "World";

    expect(el.outerHTML).toMatchSnapshot();
  });

  it("should not try to make a reactive state if one was provided", () => {
    let el;

    class Ctx extends Component {
      state = new Reactive({
        greeting: "Hello",
      });
      render() {
        // @ts-ignore:next-line
        return h("div", {}, h("h1", {}, this.state.value.greeting));
      }
    }

    const vDom = h(Ctx, {}) as vComponent;
    el = render(vDom);
    const instance = vDom.instance as Component;

    expect(el.outerHTML).toMatchSnapshot();

    instance.state.value.greeting = "World";

    expect(el.outerHTML).toMatchSnapshot();
  });

  it("should patch a component when the state changes", (done) => {
    let el;

    class Ctx extends Component {
      state = {
        greeting: "Hello",
      };
      render() {
        // @ts-ignore:next-line
        return h("div", {}, h("h1", {}, this.state.value.greeting));
      }
      onUpdated() {
        expect(el.outerHTML).toMatchSnapshot();
        done();
      }
    }

    const vDom = h(Ctx, {}) as vComponent;

    el = render(vDom);
    const instance = vDom.instance as Component;

    expect(el.outerHTML).toMatchSnapshot();

    instance.state.value.greeting = "World";
  });

  it("should not patch a component when the _el is not defined", () => {
    let el;
    const spy = jest.fn();

    class Ctx extends Component {
      state = {
        greeting: "Hello",
      };
      render() {
        // @ts-ignore:next-line
        return h("div", {}, h("h1", {}, this.state.value.greeting));
      }
      onUpdated() {
        spy();
      }
    }

    const vDom = h(Ctx, {}) as vComponent;

    el = render(vDom);
    const instance = vDom.instance as Component;

    expect(el.outerHTML).toMatchSnapshot();

    instance._el = null;
    instance.state.value.greeting = "World";

    expect(spy).toHaveBeenCalledTimes(0);
  });

  it("the _unmount function should be called when unmounting a child component", () => {
    class Ctx extends Component {
      render() {
        return h("div", {}, h("h1", {}, "Hello World"));
      }
    }

    const oldVnode = h("div", {}, h(Ctx, {}));

    const newVnode = h("div", {});

    const el = render(oldVnode) as HTMLElement;

    expect(el.outerHTML).toMatchSnapshot();

    const patch = diff(oldVnode, newVnode);

    patch(el);

    expect(el.outerHTML).toMatchSnapshot();
  });

  // it("the beforeUnmount function should be called when unmounting a child component", async () => {
  //   const spy = jest.fn();
  //   let showCtx = true;

  //   // @ts-ignore:next-line
  //   class Ctx extends Component {
  //     render() {
  //       return h("h1", {}, "Hello World");
  //     }
  //     beforeUnmount() {
  //       spy();
  //     }
  //   }

  //   class App extends Component {
  //     render() {
  //       return h('div', {},
  //         showCtx ? h(Ctx, {}) : null
  //       )
  //     }
  //   }

  //   let el = document.createElement('div')
  //   const app = mount(App, el);

  //   // @ts-ignore:next-line
  //   expect(app._el.outerHTML).toMatchSnapshot();

  //   showCtx = false

  //   const patch = app._getDiff()

  //   app._el = patch(el);

  //   // @ts-ignore:next-line
  //   expect(app._el.outerHTML).toMatchSnapshot();

  //   expect(spy).toHaveBeenCalledTimes(1);
  // });

  it("should change the _el when replacing the root node", () => {
    class Ctx extends Component {
      render() {
        return h("div", {}, h("h1", {}, "Hello World"));
      }
    }

    const vDom = h(Ctx, {}) as vComponent;
    const el = render(vDom) as HTMLElement;
    const instance = vDom.instance as Component;

    expect(el.outerHTML).toMatchSnapshot();

    instance.render = () => {
      return h("h1", {}, "Hello World");
    };

    instance._update();

    expect(el.outerHTML).toMatchSnapshot();

    expect(el === instance._el).toBe(false);
  });
});
