import { observable } from "./observable";
import { diff } from "./diff";
let uid = 0;
export class Component {
  _uid;
  _el;
  _vNode;
  _mounted;
  _isSuika;
  state;
  attrs;
  constructor() {
    this._uid = uid++;
    this._el = null;
    this._vNode = null;
    this._mounted = false;
    this._isSuika = true;
    this.state = {};
    this.attrs = {};
  }
  _update() {
    if (this._el) {
      const patch = this._getDiff();
      this._el = patch(this._el);
      this.onUpdated();
    }
  }
  _getDiff() {
    const vNode = this.render();
    const patch = diff(this._vNode, vNode);
    this._vNode = vNode;
    return patch;
  }
  _initVnode(attrs) {
    this._setAttrs(attrs);
    const vNode = this.render();
    this._vNode = vNode;
    return vNode;
  }
  _initState() {
    this.state = observable(this.state, this._update.bind(this));
  }
  _setAttrs(attrs) {
    this.attrs = attrs;
  }
  _notifyMounted(el) {
    this._el = el;
    this._mounted = true;
    this.onMounted();
  }
  _unmount() {
    this.beforeUnmount();
    this._el = null;
  }
  onMounted() {}
  beforeUnmount() {}
  onUpdated() {}
}
//# sourceMappingURL=Component.js.map
