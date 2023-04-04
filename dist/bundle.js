var E = Object.defineProperty;
var P = (t, n, e) => n in t ? E(t, n, { enumerable: !0, configurable: !0, writable: !0, value: e }) : t[n] = e;
var a = (t, n, e) => (P(t, typeof n != "symbol" ? n + "" : n, e), e);
function y(t) {
  return Object.prototype.toString.call(t);
}
function p(t) {
  return Array.isArray(t);
}
function F(t) {
  return t == null;
}
function l(t) {
  return t != null;
}
function R(t) {
  return t === !0;
}
function z(t) {
  return t === !1;
}
function U(t, n) {
  return Object.prototype.hasOwnProperty.call(t, n);
}
const h = (t) => t !== null && typeof t == "object", W = (t) => typeof t == "function";
function A(t) {
  return !p(t) && !p(t) && y(t) === "[object Object]";
}
function D(t) {
  return y(t) === "[object RegExp]";
}
const q = (t) => typeof t == "string", L = (t) => new Promise((n) => setTimeout(n, t));
function g(t, n) {
  const e = /* @__PURE__ */ new Map(), s = t.split(",");
  for (let i = 0; i < s.length; i++)
    e.set(s[i], !0);
  return (i) => e.has(n ? i.toLowerCase() : i);
}
function v(t) {
  return j(t) || M(t);
}
const j = g(
  "html,body,base,head,link,meta,style,title,address,article,aside,footer,header,h1,h2,h3,h4,h5,h6,hgroup,nav,section,div,dd,dl,dt,figcaption,figure,picture,hr,img,li,main,ol,p,pre,ul,a,b,abbr,bdi,bdo,br,cite,code,data,dfn,em,i,kbd,mark,q,rp,rt,rtc,ruby,s,samp,small,span,strong,sub,sup,time,u,var,wbr,area,audio,map,track,video,embed,object,param,source,canvas,script,noscript,del,ins,caption,col,colgroup,table,thead,tbody,td,th,tr,button,datalist,fieldset,form,input,label,legend,meter,optgroup,option,output,progress,select,textarea,details,dialog,menu,menuitem,summary,content,element,shadow,template,blockquote,iframe,tfoot",
  !0
), M = g(
  "svg,animate,circle,clippath,cursor,defs,desc,ellipse,filter,font-face,foreignobject,g,glyph,image,line,marker,mask,missing-glyph,path,pattern,polygon,polyline,rect,switch,symbol,text,textpath,tspan,use,view",
  !0
), b = (t, n) => t && n && h(t) && h(n) && typeof t == typeof n ? Object.keys(t).length === Object.keys(n).length && Object.keys(t).every((e) => b(t[e], n[e])) : t === n, V = /* @__PURE__ */ Object.freeze(/* @__PURE__ */ Object.defineProperty({
  __proto__: null,
  default: g,
  hasOwn: U,
  isArray: p,
  isDef: l,
  isEqual: b,
  isFalse: z,
  isFunction: W,
  isHTMLTag: j,
  isObject: h,
  isPlainObject: A,
  isRegExp: D,
  isReservedTag: v,
  isSVG: M,
  isString: q,
  isTrue: R,
  isUndef: F,
  sleep: L,
  toString: y
}, Symbol.toStringTag, { value: "Module" }));
function m(t, n, e = []) {
  if (!h(t))
    return t;
  const s = (c) => e.concat(c).join(".");
  for (const c in t)
    t[c] = m(
      t[c],
      n,
      e.concat(c)
    );
  const i = (c, u) => {
    const f = Reflect.deleteProperty(c, u);
    return typeof n == "function" && n({
      path: s(u),
      target: c,
      name: u
    }), f;
  }, r = (c, u, f, x) => {
    const C = Reflect.set(
      c,
      u,
      m(f, n),
      x
    );
    return typeof n == "function" && n({
      path: s(u),
      target: c,
      name: u,
      value: f
    }), C;
  }, d = (c, u, f) => Reflect.get(c, u, f);
  return new Proxy(t, {
    deleteProperty: i,
    set: r,
    get: d
  });
}
const H = () => "fragment", O = (t) => t.prototype instanceof w, G = (t) => t === H && t() === t(), N = (t, n, e) => ({
  kind: "element",
  tag: t,
  attrs: n,
  children: e
}), B = (t, n) => ({
  kind: "component",
  instance: void 0,
  attrs: n,
  component: t
}), I = (t, n, e) => ({
  kind: "function",
  attrs: n,
  component: t,
  children: e
}), J = (t) => ({
  kind: "fragment",
  children: t
}), S = (t) => ({
  kind: "text",
  value: t.toString()
}), K = (t) => t.filter((n) => l(n)).map((n) => {
  let e;
  return typeof n == "string" ? e = S(n) : e = n, e;
}), tt = (t, n = {}, ...e) => {
  const s = K(e);
  return typeof t == "string" && v(t) ? N(t, n, s) : typeof t == "function" ? G(t) ? J(s) : O(t) ? B(t, n) : I(t, n, s) : S(t);
}, o = (t) => {
  if (t.kind === "text")
    return document.createTextNode(t.value);
  if (t.kind === "function")
    return o(t.component(t.attrs, t.children));
  if (t.kind === "component") {
    if (t.instance) {
      const i = o(t.instance.render());
      return t.instance._notifyMounted(i), i;
    }
    t.instance = new t.component(), t.instance._initState();
    const e = t.instance._initVnode(t.attrs), s = o(e);
    return t.instance._notifyMounted(s), s;
  }
  const n = document.createElement(t.tag);
  for (const e in t.attrs)
    n[e] = t.attrs[e];
  return t.children.forEach((e) => {
    n.appendChild(o(e));
  }), n;
}, nt = (t, n) => {
  if (!l(t) || !O(t))
    throw new Error("Must pass a component to mount");
  if (!l(t) || !(n instanceof HTMLElement))
    throw new Error("Must pass a dom node to mount");
  const e = new t();
  e._initState();
  const s = o(e._initVnode({}));
  return e._notifyMounted(s), n.replaceWith(s), e;
};
function k(t, n) {
  if (!t)
    return (i) => {
      const r = o(n);
      return i.replaceWith(r), r;
    };
  if (!n)
    return (i) => {
      p(t.children) && _(t.children, []), i.remove();
    };
  if (t.kind === "text" || n.kind === "text")
    return t.value !== n.value ? (i) => {
      const r = o(n);
      return i.replaceWith(r), r;
    } : (i) => i;
  if (t.kind === "function" && n.kind === "function")
    return k(
      t,
      n.component(n.attrs, n.children)
    );
  if (t.kind === "component" && n.kind === "component" && t.component === n.component && t.instance)
    return n.instance = t.instance, b(t.attrs, n.attrs) ? (i) => i : (n.instance._setAttrs(n.attrs), n.instance._getDiff());
  if (n.kind === "component")
    return n.instance = new n.component(), n.instance._initState(), n.instance._initVnode(n.attrs), (i) => {
      const r = o(n);
      return i.replaceWith(r), n.instance._notifyMounted(i), r;
    };
  if (t.tag !== n.tag)
    return (i) => {
      const r = o(n);
      return p(t.children) && _(t.children, n.children), i.replaceWith(r), r;
    };
  const e = Q(t.attrs, n.attrs), s = X(t.children, n.children);
  return (i) => (e(i), s(i), i);
}
function Q(t = {}, n = {}) {
  const e = {
    remove: Object.keys(t || {}).filter(
      (s) => !l(n[s])
    ),
    set: Object.keys(n || {}).filter(
      (s) => t[s] !== n[s] && l(n[s])
    ).reduce((s, i) => ({ ...s, [i]: n[i] }), {})
  };
  return (s) => {
    for (const i of e.remove)
      s.removeAttribute(i);
    for (const i in e.set)
      s[i] = e.set[i];
  };
}
function X(t, n) {
  const e = [], s = [];
  t.forEach((i, r) => {
    e.push(k(i, n[r]));
  });
  for (const i of n.slice(t.length))
    s.push((r) => (r.appendChild(o(i)), r));
  return (i) => {
    if (i) {
      for (const [r, d] of Z(
        e,
        i.childNodes
      ))
        r(d);
      for (const r of s)
        r(i);
    }
    return i;
  };
}
function _(t, n) {
  let e = t.length;
  for (; e--; ) {
    const s = t[e];
    if (s.kind === "component") {
      const i = Y(s, n);
      i && _(
        s.instance._vNode.children,
        i.instance._vNode.children
      ), i || s.instance._unmount();
    }
  }
}
function Y(t, n) {
  return n.find(
    (e) => t.component === e.component
  );
}
const Z = (t, n) => {
  const e = [];
  for (let s = 0; s < Math.min(t.length, n.length); s++)
    e.push([t[s], n[s]]);
  return e;
};
let $ = 0;
class w {
  constructor() {
    a(this, "_uid");
    a(this, "_el");
    a(this, "_vNode");
    a(this, "_mounted");
    a(this, "state");
    a(this, "attrs");
    a(this, "_isSuika");
    this._uid = $++, this._el = null, this._vNode = null, this._mounted = !1, this._isSuika = !0, this.state = {}, this.attrs = {};
  }
  _update() {
    if (this._el) {
      const n = this._getDiff();
      this._el = n(this._el), this.onUpdated();
    }
  }
  _getDiff() {
    const n = this.render(), e = k(this._vNode, n);
    return this._vNode = n, e;
  }
  _initVnode(n) {
    return this._setAttrs(n), this._vNode = this.render(), this._vNode;
  }
  _initState() {
    this.state = m(this.state, this._update.bind(this));
  }
  _setAttrs(n) {
    this.attrs = n;
  }
  _notifyMounted(n) {
    this._el = n, this._mounted = !0, this.onMounted();
  }
  _unmount() {
    this.beforeUnmount(), this._el = null;
  }
  onMounted() {
  }
  beforeUnmount() {
  }
  onUpdated() {
  }
}
export {
  w as Component,
  H as Fragment,
  tt as h,
  nt as mount,
  V as utils
};
