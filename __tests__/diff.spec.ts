import { h } from "../src/vdom";
import { Component } from "../src/Component";
import { render, mount } from "../src/render";
import { diff } from "../src/diff";

describe("diff", () => {
  it("should be replace the node if the old vnode is null", () => {
    const newVnode = h("div", { id: "test" });

    const patch = diff(null, newVnode);
    let el = patch(document.createElement("div"));

    expect(el.outerHTML).toMatchSnapshot();
    expect(el.id).toEqual("test");
  });

  it("should remove the node if the new vnode is undefined", () => {
    const oldVnode = h("div", { id: "test" });
    let el = render(oldVnode);

    const patch = diff(oldVnode, null);
    el = patch(el);

    expect(el).toBe(undefined);
  });

  it("should be able to update child nodes", () => {
    const oldVnode = h(
      "div",
      { id: "test" },
      h("h1", { id: "title" }, "Hello")
    );

    const newVnode = h("div", { id: "test" }, h("h1", {}, "Hello"));

    let el = render(oldVnode);

    expect(el.outerHTML).toMatchSnapshot();
    expect(el.childNodes[0].id).toEqual("title");

    const patch = diff(oldVnode, newVnode);
    el = patch(el);

    expect(el.outerHTML).toMatchSnapshot();
    expect(el.childNodes[0].id).toEqual("");
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

    let el = render(oldVnode);

    expect(el.outerHTML).toMatchSnapshot();
    expect(el.childNodes.length).toEqual(1);

    const patch = diff(oldVnode, newVnode);
    el = patch(el);

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

    let el = render(oldVnode);

    expect(el.outerHTML).toMatchSnapshot();
    expect(el.childNodes.length).toEqual(2);

    const patch = diff(oldVnode, newVnode);
    el = patch(el);

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

    let el = render(oldVnode);

    expect(el.outerHTML).toMatchSnapshot();
    expect(el.childNodes[0].childNodes[0].textContent).toEqual("Hello");

    const patch = diff(oldVnode, newVnode);
    el = patch(el);

    expect(el.outerHTML).toMatchSnapshot();
    expect(el.childNodes[0].childNodes[0].textContent).toEqual("World");
  });

  it("should be able to diff functional components", () => {
    const Header = ({ title }) => h("h1", {}, title);
    const oldVnode = h(Header, { title: "test" });
    const newVnode = h(Header, { title: "updated" });
    let el = render(oldVnode);

    expect(el.outerHTML).toMatchSnapshot();
    expect(el.innerHTML).toEqual("test");

    const patch = diff(oldVnode, newVnode);
    el = patch(el);

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

    let instance = mount(Ctx, document.createElement("div"));

    expect(instance._el.outerHTML).toMatchSnapshot();
    expect(instance._el.innerHTML).toEqual("Hello");

    text = "World";
    const patch = instance._getDiff();
    patch(instance._el);

    expect(instance._el.outerHTML).toMatchSnapshot();
    expect(instance._el.innerHTML).toEqual("World");
  });

  it("should be able to diff child components", () => {
    class Btn extends Component {
      render() {
        return h("button", {}, this.attrs.text);
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
          h(Btn, { text: this.state.text }),
          h(Btn, { text: this.state.text })
        );
      }
    }

    let instance = mount(Ctx, document.createElement("div"));
    let childInstance = instance._vNode.children[0].instance;

    expect(instance._el.outerHTML).toMatchSnapshot();
    expect(childInstance._el.innerHTML).toEqual("Hello");

    instance.state.text = "World";
    const patch = instance._getDiff();
    patch(instance._el);

    expect(instance._el.outerHTML).toMatchSnapshot();
    expect(childInstance._el.innerHTML).toEqual("World");
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

    let instance = mount(Ctx, document.createElement("div"));

    expect(instance._el.outerHTML).toMatchSnapshot();
    expect(instance._el.firstChild.innerHTML).toEqual("hi");

    render = h("div", {}, h(Btn, {}));
    const patch = instance._getDiff();
    patch(instance._el);

    expect(instance._el.outerHTML).toMatchSnapshot();
    expect(instance._el.firstChild.innerHTML).toEqual("Hello");
  });

  it("should be able to diff multiple child components", () => {
    class Btn extends Component {
      render() {
        return h("button", {}, this.attrs.text);
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
        return h("div", {}, h(Container, { text: this.state.text }));
      }
    }

    let instance = mount(Ctx, document.createElement("div"));
    let childInstance =
      instance._vNode.children[0].instance._vNode.children[0].instance;

    expect(instance._el.outerHTML).toMatchSnapshot();
    expect(childInstance._el.innerHTML).toEqual("Hello");

    instance.state.text = "World";
    const patch = instance._getDiff();
    patch(instance._el);

    expect(instance._el.outerHTML).toMatchSnapshot();
    expect(childInstance._el.innerHTML).toEqual("World");
  });
});
