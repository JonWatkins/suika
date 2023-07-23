import { h, Component, vNode } from "suika";

export class NotFound extends Component {
  render(): vNode {
    return h(
      "div",
      {},
      h("h1", {}, "404"),
      h("p", {}, `Page ${this.attrs.currentRoute} not found.`)
    );
  }
}
