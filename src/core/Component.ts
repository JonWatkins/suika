import type { vDomUpdater } from "../vdom/diff";
import type { vDomNode } from "../vdom/h";
import { Context } from "./Context";

export abstract class Component<P> extends Context {
  protected props: P;

  constructor() {
    super();
  }

  public _initProps(props: P): vDomNode {
    this.props = props;
    this._vNode = this.render();
    return this._vNode;
  }

  public _setProps(props: P): vDomUpdater {
    this.props = props;
    this.willGetProps(this.props, this.state);
    return this._getDiff();
  }

  public willGetProps(props: P, state: object): object {
    return this.state;
  }
}
