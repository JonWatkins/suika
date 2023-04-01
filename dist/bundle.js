function h(t) {
  return Object.prototype.toString.call(t);
}
function d(t) {
  return Array.isArray(t);
}
function E(t) {
  return t == null;
}
function F(t) {
  return t != null;
}
function R(t) {
  return t === !0;
}
function A(t) {
  return t === !1;
}
function D(t, e) {
  return Object.prototype.hasOwnProperty.call(t, e);
}
const g = (t) => t !== null && typeof t == "object", l = (t) => typeof t == "function";
function G(t) {
  return !d(t) && !d(t) && h(t) === "[object Object]";
}
function q(t) {
  return h(t) === "[object RegExp]";
}
const m = (t) => typeof t == "string";
function k(t, e) {
  const n = /* @__PURE__ */ new Map(), r = t.split(",");
  for (let i = 0; i < r.length; i++)
    n.set(r[i], !0);
  return (i) => n.has(e ? i.toLowerCase() : i);
}
function v(t) {
  return x(t) || j(t);
}
const x = k(
  "html,body,base,head,link,meta,style,title,address,article,aside,footer,header,h1,h2,h3,h4,h5,h6,hgroup,nav,section,div,dd,dl,dt,figcaption,figure,picture,hr,img,li,main,ol,p,pre,ul,a,b,abbr,bdi,bdo,br,cite,code,data,dfn,em,i,kbd,mark,q,rp,rt,rtc,ruby,s,samp,small,span,strong,sub,sup,time,u,var,wbr,area,audio,map,track,video,embed,object,param,source,canvas,script,noscript,del,ins,caption,col,colgroup,table,thead,tbody,td,th,tr,button,datalist,fieldset,form,input,label,legend,meter,optgroup,option,output,progress,select,textarea,details,dialog,menu,menuitem,summary,content,element,shadow,template,blockquote,iframe,tfoot"
), j = k(
  "svg,animate,circle,clippath,cursor,defs,desc,ellipse,filter,font-face,foreignobject,g,glyph,image,line,marker,mask,missing-glyph,path,pattern,polygon,polyline,rect,switch,symbol,text,textpath,tspan,use,view",
  !0
), O = (t, e) => !1, et = /* @__PURE__ */ Object.freeze(/* @__PURE__ */ Object.defineProperty({
  __proto__: null,
  default: k,
  hasOwn: D,
  isArray: d,
  isDef: F,
  isEqual: O,
  isFalse: A,
  isFunction: l,
  isHTMLTag: x,
  isObject: g,
  isPlainObject: G,
  isRegExp: q,
  isReservedTag: v,
  isSVG: j,
  isString: m,
  isTrue: R,
  isUndef: E,
  toString: h
}, Symbol.toStringTag, { value: "Module" }));
function P(t, e, n = []) {
  if (!g(t))
    return t;
  const r = (s) => n.concat(s).join(".");
  for (const s in t)
    t[s] = P(
      t[s],
      e,
      n.concat(s)
    );
  const i = (s, u) => {
    const p = Reflect.deleteProperty(s, u);
    return l(e) && e({
      path: r(u),
      target: s,
      name: u
    }), p;
  }, c = (s, u, p, S) => {
    const T = Reflect.set(s, u, p, S);
    return l(e) && e({
      path: r(u),
      target: s,
      name: u,
      value: p
    }), T;
  }, o = (s, u, p) => Reflect.get(s, u, p);
  return new Proxy(t, {
    deleteProperty: i,
    set: c,
    get: o
  });
}
const K = () => ({ kind: "skip" }), f = (t) => ({
  kind: "replace",
  newNode: t
}), L = () => ({ kind: "remove" }), z = (t) => ({ kind: "insert", node: t }), H = (t, e) => ({
  kind: "update",
  attrs: t,
  childeren: e
}), y = (t, e) => {
  if (!t)
    return f(e);
  if (t.kind === "text" && e.kind === "text" && t.value === e.value)
    return K();
  if (t.kind === "text" || e.kind === "text")
    return f(e);
  if (t.kind === "component" && e.kind === "component" && t.component === e.component && t.instance)
    return e.instance = t.instance, O(t.attrs, e.attrs), e.instance._setProps(e.attrs);
  if (e.kind === "component")
    return e.instance = new e.component(), {
      kind: "replace",
      newNode: e.instance._initProps(e.attrs),
      callback: (i) => {
        var c;
        return (c = e.instance) == null ? void 0 : c._notifyMounted(i);
      }
    };
  if (t.kind === "function" && e.kind === "function" && t.component === e.component)
    return y(t, e.component(e.attrs));
  if (t.tag !== e.tag)
    return f(e);
  const n = {
    remove: Object.keys(t.attrs).filter(
      (i) => Object.keys(e).indexOf(i) === -1
    ),
    set: Object.keys(e.attrs).filter((i) => t.attrs[i] !== e.attrs[i]).reduce(
      (i, c) => ({ ...i, [c]: e.attrs[c] }),
      {}
    )
  }, r = W(
    t.children,
    e.children
  );
  return H(n, r);
}, W = (t, e) => {
  const n = t.map((o) => [o.key, o]), r = e.map((o) => [o.key, o]), i = [];
  let [c] = n.find(
    (o) => r.map((s) => s[0]).indexOf(o[0]) != -1
  ) || [null];
  for (; c; )
    b(i, n, c), _(i, r, c), i.push(
      y(n.shift()[1], r.shift()[1])
    ), [c] = n.find(
      (o) => r.map((s) => s[0]).indexOf(o[0]) != -1
    ) || [null];
  return b(i, n, void 0), _(i, r, void 0), i;
}, b = (t, e, n) => {
  for (; e[0] && e[0][0] != n; )
    e[0][1].kind == "component" && (e[0][1].instance._unmount(), e[0][1].instance = null), t.push(L()), e.shift();
}, _ = (t, e, n) => {
  for (; e[0] && e[0][0] != n; )
    t.push(z(e.shift()[1]));
};
function a(t) {
  if (t.kind === "text")
    return document.createTextNode(t.value);
  if (t.kind === "component") {
    if (t.instance) {
      const i = a(t.instance.render());
      return t.instance._notifyMounted(i), i;
    }
    t.instance = new t.component(), t.instance._initState();
    const n = t.instance._initProps(t.attrs), r = a(n);
    return t.instance._notifyMounted(r), r;
  }
  if (t.kind === "function") {
    const n = t.component(t.attrs), r = a(n);
    return t.children = n.children, r;
  }
  const e = document.createElement(t.tag);
  for (const n in t.attrs)
    e[n] = t.attrs[n];
  return B(e, t.children), e;
}
function B(t, e) {
  e.forEach((n) => {
    t.appendChild(a(n));
  });
}
function C(t, e) {
  if (e.kind === "skip")
    return t;
  if (e.kind == "replace") {
    const n = a(e.newNode);
    return t == null || t.replaceWith(n), e.callback && e.callback(n), n;
  }
  for (const n in e.attrs.remove)
    t.removeAttribute(n);
  for (const n in e.attrs.set)
    t[n] = e.attrs.set[n];
  return I(t, e.childeren), t;
}
const I = (t, e) => {
  let n = 0;
  for (let r = 0; r < e.length; r++) {
    const i = e[r];
    if (i.kind == "skip")
      continue;
    if (i.kind == "insert") {
      t.childNodes[r + n - 1] ? t.childNodes[r + n - 1].after(a(i.node)) : t.appendChild(a(i.node));
      continue;
    }
    const c = t.childNodes[r + n];
    if (i.kind == "remove") {
      c.remove(), n -= 1;
      continue;
    }
    C(c, i);
  }
};
let J = 0;
class U {
  constructor() {
    this._uid = J++;
  }
  _getDiff() {
    const e = this.render(), n = y(this._vNode, e);
    return n.kind === "replace" && (n.callback = (r) => this._el = r), this._vNode = e, n;
  }
  _update() {
    this._el && (C(this._el, this._getDiff()), this._notifyUpdated());
  }
  _initState() {
    this.state = P(this.state || {}, this._update.bind(this));
  }
  _unmount() {
    this.beforeUnmount(), this._el = null;
  }
  _notifyMounted(e) {
    this._el = e, setTimeout(() => this.onMounted());
  }
  _notifyUpdated() {
    setTimeout(() => this.onUpdated());
  }
  onMounted() {
  }
  onUpdated() {
  }
  beforeUnmount() {
  }
}
class nt extends U {
  constructor() {
    super();
  }
  _mount(e) {
    this._el = e, this._mounted = !0, this._initState(), this._update();
  }
}
class Q extends U {
  constructor() {
    super();
  }
  _initProps(e) {
    return this.props = e, this._vNode = this.render(), this._vNode;
  }
  _setProps(e) {
    return this.props = e, this.willGetProps(this.props, this.state), this._getDiff();
  }
  willGetProps(e, n) {
    return this.state;
  }
}
const X = () => "FRAGMENT", Y = (t) => m(t) ? M(t) : t, Z = (t) => t.prototype instanceof Q, $ = (t) => t === X, N = (t, e, n) => {
  e || (e = {});
  const r = e.key;
  return delete e.key, {
    tag: t,
    attrs: e,
    children: n,
    key: r,
    kind: "element"
  };
}, w = (t, e) => {
  const n = e.key;
  return delete e.key, {
    attrs: e,
    key: n,
    kind: "component",
    component: t,
    tag: void 0,
    children: []
  };
}, V = (t, e) => {
  const n = e.key;
  return delete e.key, {
    attrs: e,
    key: n,
    kind: "function",
    component: t,
    tag: void 0,
    children: []
  };
}, M = (t, e = "") => ({
  key: e,
  kind: "text",
  value: t.toString(),
  children: []
}), tt = (t, e, n) => {
  e || (e = {});
  const r = e.key;
  return delete e.key, {
    key: r,
    tag: t,
    kind: "fragment",
    attrs: e,
    children: n
  };
}, it = (t, e, ...n) => {
  if (m(t) && v(t)) {
    const r = n.map(Y);
    return N(t, e, r);
  } else
    return l(t) ? $(t) ? tt(t(), e, n) : Z(t) ? w(t, e || {}) : V(t, e || {}) : M(t);
}, rt = (t, e) => {
  const n = new t();
  return n._mount(e), n;
}, st = "1.1.0";
export {
  Q as Component,
  X as Fragment,
  nt as Suika,
  y as diff,
  it as h,
  rt as mount,
  P as observable,
  C as patch,
  a as render,
  et as utils,
  st as version
};
