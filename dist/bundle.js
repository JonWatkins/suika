function d(t) {
  return Object.prototype.toString.call(t);
}
function h(t) {
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
function D(t, n) {
  return Object.prototype.hasOwnProperty.call(t, n);
}
const g = (t) => t !== null && typeof t == "object", f = (t) => typeof t == "function";
function G(t) {
  return !h(t) && !h(t) && d(t) === "[object Object]";
}
function q(t) {
  return d(t) === "[object RegExp]";
}
const m = (t) => typeof t == "string";
function k(t, n) {
  const e = /* @__PURE__ */ new Map(), i = t.split(",");
  for (let r = 0; r < i.length; r++)
    e.set(i[r], !0);
  return (r) => e.has(n ? r.toLowerCase() : r);
}
function v(t) {
  return x(t) || j(t);
}
const x = k(
  "html,body,base,head,link,meta,style,title,address,article,aside,footer,header,h1,h2,h3,h4,h5,h6,hgroup,nav,section,div,dd,dl,dt,figcaption,figure,picture,hr,img,li,main,ol,p,pre,ul,a,b,abbr,bdi,bdo,br,cite,code,data,dfn,em,i,kbd,mark,q,rp,rt,rtc,ruby,s,samp,small,span,strong,sub,sup,time,u,var,wbr,area,audio,map,track,video,embed,object,param,source,canvas,script,noscript,del,ins,caption,col,colgroup,table,thead,tbody,td,th,tr,button,datalist,fieldset,form,input,label,legend,meter,optgroup,option,output,progress,select,textarea,details,dialog,menu,menuitem,summary,content,element,shadow,template,blockquote,iframe,tfoot"
), j = k(
  "svg,animate,circle,clippath,cursor,defs,desc,ellipse,filter,font-face,foreignobject,g,glyph,image,line,marker,mask,missing-glyph,path,pattern,polygon,polyline,rect,switch,symbol,text,textpath,tspan,use,view",
  !0
), O = (t, n) => !1, nt = /* @__PURE__ */ Object.freeze(/* @__PURE__ */ Object.defineProperty({
  __proto__: null,
  default: k,
  hasOwn: D,
  isArray: h,
  isDef: F,
  isEqual: O,
  isFalse: A,
  isFunction: f,
  isHTMLTag: x,
  isObject: g,
  isPlainObject: G,
  isRegExp: q,
  isReservedTag: v,
  isSVG: j,
  isString: m,
  isTrue: R,
  isUndef: E,
  toString: d
}, Symbol.toStringTag, { value: "Module" }));
function P(t, n, e = []) {
  if (!g(t))
    return t;
  const i = (s) => e.concat(s).join(".");
  for (const s in t)
    t[s] = P(
      t[s],
      n,
      e.concat(s)
    );
  const r = (s, u) => {
    const p = Reflect.deleteProperty(s, u);
    return f(n) && n({
      path: i(u),
      target: s,
      name: u
    }), p;
  }, c = (s, u, p, S) => {
    const T = Reflect.set(s, u, p, S);
    return f(n) && n({
      path: i(u),
      target: s,
      name: u,
      value: p
    }), T;
  }, o = (s, u, p) => Reflect.get(s, u, p);
  return new Proxy(t, {
    deleteProperty: r,
    set: c,
    get: o
  });
}
const K = () => ({ kind: "skip" }), l = (t) => ({
  kind: "replace",
  newNode: t
}), L = () => ({ kind: "remove" }), z = (t) => ({ kind: "insert", node: t }), H = (t, n) => ({
  kind: "update",
  attrs: t,
  childeren: n
}), y = (t, n) => {
  if (!t)
    return l(n);
  if (t.kind === "text" && n.kind === "text" && t.value === n.value)
    return K();
  if (t.kind === "text" || n.kind === "text")
    return l(n);
  if (t.kind === "component" && n.kind === "component" && t.component === n.component && t.instance)
    return n.instance = t.instance, O(t.attrs, n.attrs), n.instance._setProps(n.attrs);
  if (n.kind === "component")
    return n.instance = new n.component(), {
      kind: "replace",
      newNode: n.instance._initProps(n.attrs),
      callback: (r) => {
        var c;
        return (c = n.instance) == null ? void 0 : c._notifyMounted(r);
      }
    };
  if (t.kind === "function" && n.kind === "function" && t.component === n.component)
    return y(t, n.component(n.attrs));
  if (t.tag !== n.tag)
    return l(n);
  const e = {
    remove: Object.keys(t.attrs).filter(
      (r) => Object.keys(n).indexOf(r) === -1
    ),
    set: Object.keys(n.attrs).filter((r) => t.attrs[r] !== n.attrs[r]).reduce(
      (r, c) => ({ ...r, [c]: n.attrs[c] }),
      {}
    )
  }, i = W(
    t.children,
    n.children
  );
  return H(e, i);
}, W = (t, n) => {
  const e = t.map((o) => [o.key, o]), i = n.map((o) => [o.key, o]), r = [];
  let [c] = e.find(
    (o) => i.map((s) => s[0]).indexOf(o[0]) != -1
  ) || [null];
  for (; c; )
    b(r, e, c), _(r, i, c), r.push(
      y(e.shift()[1], i.shift()[1])
    ), [c] = e.find(
      (o) => i.map((s) => s[0]).indexOf(o[0]) != -1
    ) || [null];
  return b(r, e, void 0), _(r, i, void 0), r;
}, b = (t, n, e) => {
  for (; n[0] && n[0][0] != e; )
    n[0][1].kind == "component" && (n[0][1].instance._unmount(), n[0][1].instance = null), t.push(L()), n.shift();
}, _ = (t, n, e) => {
  for (; n[0] && n[0][0] != e; )
    t.push(z(n.shift()[1]));
};
function a(t) {
  if (t.kind === "text")
    return document.createTextNode(t.value);
  if (t.kind === "component") {
    if (t.instance) {
      const r = a(t.instance.render());
      return t.instance._notifyMounted(r), r;
    }
    t.instance = new t.component(), t.instance._initState();
    const e = t.instance._initProps(t.attrs), i = a(e);
    return t.instance._notifyMounted(i), i;
  }
  if (t.kind === "function") {
    const e = t.component(t.attrs, t.children), i = a(e);
    return t.children = e.children, i;
  }
  const n = document.createElement(t.tag);
  for (const e in t.attrs)
    n[e] = t.attrs[e];
  return B(n, t.children), n;
}
function B(t, n) {
  n.forEach((e) => {
    t.appendChild(a(e));
  });
}
function C(t, n) {
  if (n.kind === "skip")
    return t;
  if (n.kind == "replace") {
    const e = a(n.newNode);
    return t == null || t.replaceWith(e), n.callback && n.callback(e), e;
  }
  for (const e in n.attrs.remove)
    t.removeAttribute(e);
  for (const e in n.attrs.set)
    t[e] = n.attrs.set[e];
  return I(t, n.childeren), t;
}
const I = (t, n) => {
  let e = 0;
  for (let i = 0; i < n.length; i++) {
    const r = n[i];
    if (r.kind == "skip")
      continue;
    if (r.kind == "insert") {
      t.childNodes[i + e - 1] ? t.childNodes[i + e - 1].after(a(r.node)) : t.appendChild(a(r.node));
      continue;
    }
    const c = t.childNodes[i + e];
    if (r.kind == "remove") {
      c.remove(), e -= 1;
      continue;
    }
    C(c, r);
  }
};
let J = 0;
class U {
  constructor() {
    this._uid = J++;
  }
  _getDiff() {
    const n = this.render(), e = y(this._vNode, n);
    return e.kind === "replace" && (e.callback = (i) => this._el = i), this._vNode = n, e;
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
  _notifyMounted(n) {
    this._el = n, setTimeout(() => this.onMounted());
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
class et extends U {
  constructor() {
    super();
  }
  _mount(n) {
    this._el = n, this._mounted = !0, this._initState(), this._update();
  }
}
class Q extends U {
  constructor() {
    super();
  }
  _initProps(n) {
    return this.props = n, this._vNode = this.render(), this._vNode;
  }
  _setProps(n) {
    return this.props = n, this.willGetProps(this.props, this.state), this._getDiff();
  }
  willGetProps(n, e) {
    return this.state;
  }
}
const X = () => "FRAGMENT", Y = (t) => m(t) ? M(t) : t, Z = (t) => t.prototype instanceof Q, $ = (t) => t === X, N = (t, n, e) => {
  n || (n = {});
  const i = n.key;
  return delete n.key, {
    tag: t,
    attrs: n,
    children: e,
    key: i,
    kind: "element"
  };
}, w = (t, n) => {
  const e = n.key;
  return delete n.key, {
    attrs: n,
    key: e,
    kind: "component",
    component: t,
    tag: void 0,
    children: []
  };
}, V = (t, n, e) => {
  const i = n.key;
  return delete n.key, {
    attrs: n,
    key: i,
    kind: "function",
    component: t,
    tag: void 0,
    children: e
  };
}, M = (t, n = "") => ({
  key: n,
  kind: "text",
  value: t.toString(),
  children: []
}), tt = (t, n, e) => {
  n || (n = {});
  const i = n.key;
  return delete n.key, {
    key: i,
    tag: t,
    kind: "fragment",
    attrs: n,
    children: e
  };
}, it = (t, n, ...e) => {
  if (m(t) && v(t)) {
    const i = e.map(Y);
    return N(t, n, i);
  } else
    return f(t) ? $(t) ? tt(t(), n, e) : Z(t) ? w(t, n || {}) : V(t, n || {}, e) : M(t);
}, rt = (t, n) => {
  const e = new t();
  return e._mount(n), e;
}, st = "1.1.0";
export {
  Q as Component,
  X as Fragment,
  et as Suika,
  y as diff,
  it as h,
  rt as mount,
  P as observable,
  C as patch,
  a as render,
  nt as utils,
  st as version
};
