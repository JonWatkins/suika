import { Ctor, vNode, vAttrs, h, Component } from "suika";
import { Router } from "./router";
import { NotFound } from "./NotFound";

export class RouterView extends Component {
  router: Router | null;

  constructor() {
    super();
    this.router = null;
  }

  public _setAttrs(attrs: vAttrs) {
    this.attrs = attrs;
    if (attrs.router) {
      this.router = attrs.router;
    }
  }

  public changeHandler(e: Event) {
    e.preventDefault();
    this._update();
  }

  public onMounted() {
    window.addEventListener("popstate", this.changeHandler.bind(this));
  }

  public beforeUnmount(): void {
    window.removeEventListener("popstate", this.changeHandler.bind(this));
  }

  public render(): vNode {
    let current;
    let handler;

    if (this.router) {
      const fragment = this.router.getFragment();
      handler = this.router.getHandler(fragment);
      current = this.router.current;
    } else {
      handler = NotFound;
    }

    return h(handler as Ctor, {
      currentRoute: current,
    });
  }
}
