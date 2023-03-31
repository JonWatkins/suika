import { Suika } from "../../src/core/Suika";
import { h } from "../../src/vdom/h";
import { diff } from "../../src/vdom/diff";
import { patch } from "../../src/vdom/patch";
import { mount } from "../../src/vdom/mount";

describe("Suika", () => {
  it("Should be able to make a simpe app", () => {
    // @ts-ignore:next-line
    class Ctx extends Suika {
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
    class Ctx extends Suika {
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
    class Ctx extends Suika {
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
    class Ctx extends Suika {
      constructor() {
        super();
      }
      render() {
        return h("div", {}, h("h1", {}, "Hello World"));
      }
    }

    const el = document.createElement("div");
    mount(Ctx, el);

    // @ts-ignore:next-line
    expect(el.outerHTML).toMatchSnapshot();
  });

  it("should be able to patch a component", (done) => {
    let el;

    // @ts-ignore:next-line
    class Ctx extends Suika {
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

    el = document.createElement("div");
    const instance = mount(Ctx, el);

    // @ts-ignore:next-line
    instance._update();
  });

  it("calls the onUpdated function when state changes", () => {
    let el;

    // @ts-ignore:next-line
    class Ctx extends Suika {
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

    el = document.createElement("div");
    const instance = mount(Ctx, el);

    expect(el.outerHTML).toMatchSnapshot();

    // @ts-ignore:next-line
    instance.state.greeting = "World";

    // @ts-ignore:next-line
    expect(el.outerHTML).toMatchSnapshot();
  });

  it("should patch a component when the state changes", (done) => {
    let el;

    // @ts-ignore:next-line
    class Ctx extends Suika {
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

    el = document.createElement("div");
    const instance = mount(Ctx, el);

    expect(el.outerHTML).toMatchSnapshot();

    // @ts-ignore:next-line
    instance.state.greeting = "World";
  });

  it("should not patch a component when the _el is not defined", () => {
    let el;
    const spy = jest.fn();

    // @ts-ignore:next-line
    class Ctx extends Suika {
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

    el = document.createElement("div");
    const instance = mount(Ctx, el);

    expect(el.outerHTML).toMatchSnapshot();

    // @ts-ignore:next-line
    instance._el = null;
    // @ts-ignore:next-line
    instance.state.greeting = "World";

    expect(spy).toHaveBeenCalledTimes(0);
  });

  it("should change the _el when replacing the root node", () => {
    // @ts-ignore:next-line
    class Ctx extends Suika {
      constructor() {
        super();
      }
      render() {
        return h("div", {}, h("h1", {}, "Hello World"));
      }
    }

    let el = document.createElement("div");
    const instance = mount(Ctx, el);

    // @ts-ignore:next-line
    expect(el.outerHTML).toMatchSnapshot();

    // @ts-ignore:next-line
    instance.render = () => {
      return h("h1", {}, "Hello World");
    };

    // @ts-ignore:next-line
    instance._update();

    // @ts-ignore:next-line
    expect(el.outerHTML).toMatchSnapshot();

    // @ts-ignore:next-line
    expect(el === instance._el).toBe(false);
  });
});
