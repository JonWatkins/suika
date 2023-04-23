import { h, vComponent, vElement } from "../src/vdom";
import { Component } from "../src/Component";
import { render, mount } from "../src/render";
import { diff } from "../src/diff";

describe("diff", () => {
  it("should return the old element if old and new nodes are null", () => {
    const el = document.createElement("div");
    const patch = diff(null, null);
    expect(patch(document.createElement("div"))).toEqual(el);
  });

  it("should be replace the node if the old vnode is null", () => {
    const newVnode = h("div", { id: "test" });

    const patch = diff(null, newVnode);
    const el = patch(document.createElement("div")) as HTMLElement;

    expect(el.outerHTML).toMatchSnapshot();
    expect(el.id).toEqual("test");
  });

  it("should remove the node if the new vnode is null", () => {
    const oldVnode = h("div", { id: "test" });
    let el = render(oldVnode);

    const patch = diff(oldVnode, null);
    el = patch(el);

    expect(el).toBe(null);
  });

  it("can remove text nodes", () => {
    const oldVnode = h("div", {}, "Hi");
    const newVnode = h("div", {});
    let el = render(oldVnode);

    expect(el.childNodes.length).toBe(1);

    const patch = diff(oldVnode, newVnode);
    el = patch(el);

    expect(el.childNodes.length).toBe(0);
  });

  it("should be able to update child nodes", () => {
    const oldVnode = h(
      "div",
      { id: "test" },
      h("h1", { id: "title" }, "Hello")
    );

    const newVnode = h("div", { id: "test" }, h("h1", {}, "Hello"));

    let el = render(oldVnode) as HTMLElement;
    let childNode = el.childNodes[0] as HTMLElement;

    expect(el.outerHTML).toMatchSnapshot();
    expect(childNode.id).toEqual("title");

    const patch = diff(oldVnode, newVnode);
    el = patch(el) as HTMLElement;
    childNode = el.childNodes[0] as HTMLElement;

    expect(el.outerHTML).toMatchSnapshot();
    expect(childNode.id).toEqual("");
  });

  it("should be able to add child nodes", () => {
    const oldVnode = h(
      "div",
      { id: "test" },
      h("h1", { id: "title" }, "Hello")
    );

    const newVnode = h(
      "div",
      { id: "test" },
      h("h1", { id: "header" }, "Hello"),
      h("p", {}, "Lorem ipsum")
    );

    let el = render(oldVnode) as HTMLElement;

    expect(el.outerHTML).toMatchSnapshot();
    expect(el.childNodes.length).toEqual(1);

    const patch = diff(oldVnode, newVnode);
    el = patch(el) as HTMLElement;

    expect(el.outerHTML).toMatchSnapshot();
    expect(el.childNodes.length).toEqual(2);
  });

  it("should be able to remove child nodes", () => {
    const oldVnode = h(
      "div",
      { id: "test" },
      h("h1", { id: "title" }, "Hello"),
      h("p", {}, "Lorem ipsum")
    );

    const newVnode = h(
      "div",
      { id: "test" },
      h("h1", { id: "header" }, "Hello")
    );

    let el = render(oldVnode) as HTMLElement;

    expect(el.outerHTML).toMatchSnapshot();
    expect(el.childNodes.length).toEqual(2);

    const patch = diff(oldVnode, newVnode);
    el = patch(el) as HTMLElement;

    expect(el.outerHTML).toMatchSnapshot();
    expect(el.childNodes.length).toEqual(1);
  });

  it("should be able to update text child nodes", () => {
    const oldVnode = h(
      "div",
      { id: "test" },
      h("h1", { id: "title" }, "Hello")
    );

    const newVnode = h(
      "div",
      { id: "test" },
      h("h1", { id: "header" }, "World")
    );

    let el = render(oldVnode) as HTMLElement;

    expect(el.outerHTML).toMatchSnapshot();
    expect(el.childNodes[0].childNodes[0].textContent).toEqual("Hello");

    const patch = diff(oldVnode, newVnode);
    el = patch(el) as HTMLElement;

    expect(el.outerHTML).toMatchSnapshot();
    expect(el.childNodes[0].childNodes[0].textContent).toEqual("World");
  });

  it("should be able to diff functional components", () => {
    const Header = ({ title }) => h("h1", {}, title);
    const oldVnode = h(Header, { title: "test" });
    const newVnode = h(Header, { title: "updated" });
    let el = render(oldVnode) as HTMLElement;

    expect(el.outerHTML).toMatchSnapshot();
    expect(el.innerHTML).toEqual("test");

    const patch = diff(oldVnode, newVnode);
    el = patch(el) as HTMLElement;

    expect(el.outerHTML).toMatchSnapshot();
    expect(el.innerHTML).toEqual("updated");
  });

  it("should be able to diff root components", () => {
    let text = "Hello";

    class Ctx extends Component {
      render() {
        return h("h1", {}, text);
      }
    }

    const instance = mount(Ctx, document.createElement("div"));
    const _el = instance._el as HTMLElement;

    expect(_el.outerHTML).toMatchSnapshot();
    expect(_el.innerHTML).toEqual("Hello");

    text = "World";
    const patch = instance._getDiff();
    patch(_el);

    expect(_el.outerHTML).toMatchSnapshot();
    expect(_el.innerHTML).toEqual("World");
  });

  it("should be able to diff child components", () => {
    class Btn extends Component {
      render() {
        return h("button", {}, this.attrs.text as string);
      }
    }

    class Ctx extends Component {
      state = {
        text: "Hello",
      };
      render() {
        return h(
          "div",
          {},
          // @ts-ignore:next-line
          h(Btn, { text: this.state.value.text }),
          // @ts-ignore:next-line
          h(Btn, { text: this.state.value.text })
        );
      }
    }

    const instance = mount(Ctx, document.createElement("div"));
    const _el = instance._el as HTMLElement;
    const vNode = instance._vNode as vElement;
    const childNode = vNode.children[0] as vComponent;
    const childInstance = childNode.instance;
    const _elChild = childInstance?._el as HTMLElement;

    expect(_el.outerHTML).toMatchSnapshot();
    expect(_elChild.innerHTML).toEqual("Hello");

    instance.state.value.text = "World";
    const patch = instance._getDiff();
    patch(instance._el);

    expect(_el.outerHTML).toMatchSnapshot();
    expect(_elChild.innerHTML).toEqual("World");
  });

  it("should handle adding new components", () => {
    let render = h("div", {}, h("p", {}, "hi"));

    class Btn extends Component {
      render() {
        return h("button", {}, "Hello");
      }
    }

    class Ctx extends Component {
      render() {
        return render;
      }
    }

    const instance = mount(Ctx, document.createElement("div"));
    const _el = instance._el as HTMLElement;

    expect(_el.outerHTML).toMatchSnapshot();
    expect(_el.firstChild?.textContent).toEqual("hi");

    render = h("div", {}, h(Btn, {}));
    const patch = instance._getDiff();
    patch(instance._el);

    expect(_el.outerHTML).toMatchSnapshot();
    expect(_el.firstChild?.textContent).toEqual("Hello");
  });

  it("should be able to diff multiple child components", () => {
    class Btn extends Component {
      render() {
        return h("button", {}, this.attrs.text as string);
      }
    }

    class Container extends Component {
      render() {
        return h("div", {}, h(Btn, { text: this.attrs.text }));
      }
    }

    class Ctx extends Component {
      state = {
        text: "Hello",
      };
      render() {
        // @ts-ignore:next-line
        return h("div", {}, h(Container, { text: this.state.value.text }));
      }
    }

    const instance = mount(Ctx, document.createElement("div"));
    const vNode = instance._vNode as vElement;
    const [childNode1] = vNode.children as Array<vComponent>;
    const child1Instance = childNode1.instance as Component;
    const child1Vnode = child1Instance._vNode as vElement;

    const [childNode2] = child1Vnode.children as Array<vComponent>;
    const child2instance = childNode2.instance as Component;

    const _el = instance._el as HTMLElement;
    const _childEl = child2instance._el as HTMLElement;

    expect(_el.outerHTML).toMatchSnapshot();
    expect(_childEl.innerHTML).toEqual("Hello");

    instance.state.value.text = "World";
    const patch = instance._getDiff();
    patch(instance._el);

    expect(_el.outerHTML).toMatchSnapshot();
    expect(_childEl.innerHTML).toEqual("World");
  });

  it("can will unmount a component if needed", () => {
    class Btn extends Component {
      render() {
        return h("button", {}, this.attrs.text as string);
      }
    }

    class Input extends Component {
      render() {
        return h("button", {});
      }
    }

    class Ctx extends Component {
      state = {
        text: "Hello",
        showBtn: true,
      };
      render() {
        return h(
          "div",
          {},
          // @ts-ignore:next-line
          this.state.value.showBtn
            ? // @ts-ignore:next-line
              h(Btn, { text: this.state.value.text })
            : h(Input)
        );
      }
    }

    const instance = mount(Ctx, document.createElement("div"));
    const _el = instance._el as HTMLElement;

    expect(_el.outerHTML).toMatchSnapshot();

    instance.state.value.showBtn = false;

    const patch = instance._getDiff();
    patch(instance._el);

    expect(_el.outerHTML).toMatchSnapshot();
  });
});
