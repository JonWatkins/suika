function h(t) {
  return Object.prototype.toString.call(t);
}
function d(t) {
  return Array.isArray(t);
}
function E(t) {
  return t == null;
}
function R(t) {
  return t != null;
}
function A(t) {
  return t === !0;
}
function D(t) {
  return t === !1;
}
function F(t, e) {
  return Object.prototype.hasOwnProperty.call(t, e);
}
const _ = (t) => t !== null && typeof t == "object", p = (t) => typeof t == "function";
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
function g(t) {
  return v(t) || x(t);
}
const v = k(
  "html,body,base,head,link,meta,style,title,address,article,aside,footer,header,h1,h2,h3,h4,h5,h6,hgroup,nav,section,div,dd,dl,dt,figcaption,figure,picture,hr,img,li,main,ol,p,pre,ul,a,b,abbr,bdi,bdo,br,cite,code,data,dfn,em,i,kbd,mark,q,rp,rt,rtc,ruby,s,samp,small,span,strong,sub,sup,time,u,var,wbr,area,audio,map,track,video,embed,object,param,source,canvas,script,noscript,del,ins,caption,col,colgroup,table,thead,tbody,td,th,tr,button,datalist,fieldset,form,input,label,legend,meter,optgroup,option,output,progress,select,textarea,details,dialog,menu,menuitem,summary,content,element,shadow,template,blockquote,iframe,tfoot"
), x = k(
  "svg,animate,circle,clippath,cursor,defs,desc,ellipse,filter,font-face,foreignobject,g,glyph,image,line,marker,mask,missing-glyph,path,pattern,polygon,polyline,rect,switch,symbol,text,textpath,tspan,use,view",
  !0
), j = (t, e) => !1, $ = /* @__PURE__ */ Object.freeze(/* @__PURE__ */ Object.defineProperty({
  __proto__: null,
  default: k,
  hasOwn: F,
  isArray: d,
  isDef: R,
  isEqual: j,
  isFalse: D,
  isFunction: p,
  isHTMLTag: v,
  isObject: _,
  isPlainObject: G,
  isRegExp: q,
  isReservedTag: g,
  isSVG: x,
  isString: m,
  isTrue: A,
  isUndef: E,
  toString: h
}, Symbol.toStringTag, { value: "Module" }));
function O(t, e, n = []) {
  if (!_(t))
    return t;
  const r = (s) => n.concat(s).join(".");
  for (const s in t)
    t[s] = O(
      t[s],
      e,
      n.concat(s)
    );
  const i = (s, u) => {
    const a = Reflect.deleteProperty(s, u);
    return p(e) && e({
      path: r(u),
      target: s,
      name: u
    }), a;
  }, c = (s, u, a, S) => {
    const T = Reflect.set(s, u, a, S);
    return p(e) && e({
      path: r(u),
      target: s,
      name: u,
      value: a
    }), T;
  }, o = (s, u, a) => Reflect.get(s, u, a);
  return new Proxy(t, {
    deleteProperty: i,
    set: c,
    get: o
  });
}
const N = () => ({ kind: "skip" }), f = (t) => ({
  kind: "replace",
  newNode: t
}), K = () => ({ kind: "remove" }), L = (t) => ({ kind: "insert", node: t }), z = (t, e) => ({
  kind: "update",
  attrs: t,
  childeren: e
}), P = (t, e) => {
  if (!t)
    return f(e);
  if (t.kind === "text" && e.kind === "text" && t.value === e.value)
    return N();
  if (t.kind === "text" || e.kind === "text")
    return f(e);
  if (t.kind === "component" && e.kind === "component" && t.component === e.component && t.instance)
    return e.instance = t.instance, j(t.attrs, e.attrs), e.instance._setProps(e.attrs);
  if (e.kind === "component")
    return e.instance = new e.component(), {
      kind: "replace",
      newNode: e.instance._initProps(e.attrs),
      callback: (i) => {
        var c;
        return (c = e.instance) == null ? void 0 : c._notifyMounted(i);
      }
    };
  if (t.tag !== e.tag)
    return f(e);
  const n = {
    remove: Object.keys(t.attrs || {}).filter(
      (i) => Object.keys(e).indexOf(i) === -1
    ),
    set: Object.keys(e.attrs || {}).filter((i) => t.attrs[i] !== e.attrs[i]).reduce(
      (i, c) => ({ ...i, [c]: e.attrs[c] }),
      {}
    )
  }, r = H(
    t.children,
    e.children
  );
  return z(n, r);
}, H = (t, e) => {
  const n = t.map((o) => [o.key, o]), r = e.map((o) => [o.key, o]), i = [];
  let [c] = n.find(
    (o) => r.map((s) => s[0]).indexOf(o[0]) != -1
  ) || [null];
  for (; c; )
    y(i, n, c), b(i, r, c), i.push(
      P(n.shift()[1], r.shift()[1])
    ), [c] = n.find(
      (o) => r.map((s) => s[0]).indexOf(o[0]) != -1
    ) || [null];
  return y(i, n, void 0), b(i, r, void 0), i;
}, y = (t, e, n) => {
  for (; e[0] && e[0][0] != n; )
    e[0][1].kind == "component" && (e[0][1].instance._unmount(), e[0][1].instance = null), t.push(K()), e.shift();
}, b = (t, e, n) => {
  for (; e[0] && e[0][0] != n; )
    t.push(L(e.shift()[1]));
};
function l(t) {
  if (t.kind === "text")
    return document.createTextNode(t.value);
  if (t.kind === "component") {
    if (t.instance) {
      const i = l(t.instance.render());
      return t.instance._notifyMounted(i), i;
    }
    t.instance = new t.component(), t.instance._initState();
    const n = t.instance._initProps(t.attrs), r = l(n);
    return t.instance._notifyMounted(r), r;
  }
  const e = document.createElement(t.tag);
  for (const n in t.attrs)
    e[n] = t.attrs[n];
  return J(e, t.children), e;
}
function J(t, e) {
  e.forEach((n) => {
    t.appendChild(l(n));
  });
}
function U(t, e) {
  if (e.kind === "skip")
    return t;
  if (e.kind == "replace") {
    const n = l(e.newNode);
    return t == null || t.replaceWith(n), e.callback && e.callback(n), n;
  }
  for (const n in e.attrs.remove)
    t.removeAttribute(n);
  for (const n in e.attrs.set)
    t[n] = e.attrs.set[n];
  return W(t, e.childeren), t;
}
const W = (t, e) => {
  let n = 0;
  for (let r = 0; r < e.length; r++) {
    const i = e[r];
    if (i.kind == "skip")
      continue;
    if (i.kind == "insert") {
      t.childNodes[r + n - 1] ? t.childNodes[r + n - 1].after(l(i.node)) : t.appendChild(l(i.node));
      continue;
    }
    const c = t.childNodes[r + n];
    if (i.kind == "remove") {
      c.remove(), n -= 1;
      continue;
    }
    U(c, i);
  }
};
let B = 0;
class C {
  constructor() {
    this._uid = B++;
  }
  _getDiff() {
    const e = this.render(), n = P(this._vNode, e);
    return n.kind === "replace" && (n.callback = (r) => this._el = r), this._vNode = e, n;
  }
  _update() {
    this._el && (U(this._el, this._getDiff()), this._notifyUpdated());
  }
  _initState() {
    this.state = O(this.state || {}, this._update.bind(this));
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
class w extends C {
  constructor() {
    super();
  }
  _mount(e) {
    this._el = e, this._mounted = !0, this._initState(), this._update();
  }
}
class V extends C {
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
const I = () => "FRAGMENT", Q = (t) => m(t) ? M(t) : t, X = (t, e, n) => {
  e || (e = {});
  const r = e.key;
  return delete e.key, {
    tag: t,
    attrs: e,
    children: n,
    key: r,
    kind: "element"
  };
}, Y = (t, e) => {
  const n = e.key;
  return delete e.key, {
    attrs: e,
    key: n,
    kind: "component",
    component: t,
    tag: void 0,
    children: []
  };
}, M = (t, e = "") => ({
  key: e,
  kind: "text",
  value: t.toString(),
  children: []
}), Z = (t, e, n) => {
  e || (e = {});
  const r = e.key;
  return delete e.key, {
    key: r,
    tag: t,
    kind: "fragment",
    attrs: e,
    children: n
  };
}, tt = (t, e, ...n) => {
  if (m(t) && g(t)) {
    const r = n.map(Q);
    return X(t, e, r);
  } else
    return p(t) ? t === I ? Z(t(), e, n) : Y(t, e || {}) : M(t);
}, et = (t, e) => {
  const n = new t();
  return n._mount(e), n;
};
export {
  V as Component,
  I as Fragment,
  w as NekoJS,
  tt as h,
  et as mount,
  O as observable,
  l as render,
  $ as utils
};
