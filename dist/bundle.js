var F = Object.defineProperty;
var R = (t, n, e) => n in t ? F(t, n, { enumerable: !0, configurable: !0, writable: !0, value: e }) : t[n] = e;
var a = (t, n, e) => (R(t, typeof n != "symbol" ? n + "" : n, e), e);
function b(t) {
  return Object.prototype.toString.call(t);
}
function p(t) {
  return Array.isArray(t);
}
function z(t) {
  return t == null;
}
function l(t) {
  return t != null;
}
function U(t) {
  return t === !0;
}
function W(t) {
  return t === !1;
}
function A(t, n) {
  return Object.prototype.hasOwnProperty.call(t, n);
}
const v = (t) => t !== null && typeof t == "object", d = (t) => typeof t == "function";
function D(t) {
  return !p(t) && !p(t) && b(t) === "[object Object]";
}
function q(t) {
  return b(t) === "[object RegExp]";
}
const g = (t) => typeof t == "string", L = (t) => new Promise((n) => setTimeout(n, t));
function y(t, n) {
  const e = /* @__PURE__ */ new Map(), s = t.split(",");
  for (let i = 0; i < s.length; i++)
    e.set(s[i], !0);
  return (i) => e.has(n ? i.toLowerCase() : i);
}
function j(t) {
  return M(t) || x(t);
}
const M = y(
  "html,body,base,head,link,meta,style,title,address,article,aside,footer,header,h1,h2,h3,h4,h5,h6,hgroup,nav,section,div,dd,dl,dt,figcaption,figure,picture,hr,img,li,main,ol,p,pre,ul,a,b,abbr,bdi,bdo,br,cite,code,data,dfn,em,i,kbd,mark,q,rp,rt,rtc,ruby,s,samp,small,span,strong,sub,sup,time,u,var,wbr,area,audio,map,track,video,embed,object,param,source,canvas,script,noscript,del,ins,caption,col,colgroup,table,thead,tbody,td,th,tr,button,datalist,fieldset,form,input,label,legend,meter,optgroup,option,output,progress,select,textarea,details,dialog,menu,menuitem,summary,content,element,shadow,template,blockquote,iframe,tfoot"
), x = y(
  "svg,animate,circle,clippath,cursor,defs,desc,ellipse,filter,font-face,foreignobject,g,glyph,image,line,marker,mask,missing-glyph,path,pattern,polygon,polyline,rect,switch,symbol,text,textpath,tspan,use,view",
  !0
), S = (t, n) => !1, V = /* @__PURE__ */ Object.freeze(/* @__PURE__ */ Object.defineProperty({
  __proto__: null,
  default: y,
  hasOwn: A,
  isArray: p,
  isDef: l,
  isEqual: S,
  isFalse: W,
  isFunction: d,
  isHTMLTag: M,
  isObject: v,
  isPlainObject: D,
  isRegExp: q,
  isReservedTag: j,
  isSVG: x,
  isString: g,
  isTrue: U,
  isUndef: z,
  sleep: L,
  toString: b
}, Symbol.toStringTag, { value: "Module" }));
function m(t, n, e = []) {
  if (!v(t))
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
    return d(n) && n({
      path: s(u),
      target: c,
      name: u
    }), f;
  }, r = (c, u, f, E) => {
    const P = Reflect.set(
      c,
      u,
      m(f, n),
      E
    );
    return d(n) && n({
      path: s(u),
      target: c,
      name: u,
      value: f
    }), P;
  }, h = (c, u, f) => Reflect.get(c, u, f);
  return new Proxy(t, {
    deleteProperty: i,
    set: r,
    get: h
  });
}
const G = () => "fragment", O = (t) => t.prototype instanceof w, H = (t) => t === G && t() === t(), B = (t, n, e) => ({
  kind: "element",
  tag: t,
  attrs: n,
  children: e
}), I = (t, n) => ({
  kind: "component",
  attrs: n,
  component: t
}), J = (t, n, e) => ({
  kind: "function",
  attrs: n,
  component: t,
  children: e
}), K = (t) => ({
  kind: "fragment",
  children: t
}), C = (t) => ({
  kind: "text",
  value: t.toString()
}), N = (t) => t.filter((n) => l(n)).map(
  (n) => g(n) ? C(n) : n
), tt = (t, n = {}, ...e) => {
  const s = N(e);
  return g(t) && j(t) ? B(t, n, s) : d(t) ? H(t) ? K(s) : O(t) ? I(t, n) : J(t, n, s) : C(t);
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
    throw new Error("Must pass a component to render");
  if (!l(t) || !(n instanceof Element))
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
    return n.instance = t.instance, S(t.attrs, n.attrs), n.instance._setAttrs(n.attrs), n.instance._getDiff();
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
function Q(t, n) {
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
      for (const [r, h] of Z(
        e,
        i.childNodes
      ))
        r(h);
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
    a(this, "_isSuika", !0);
    this._uid = $++;
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
    this.state = m(this.state || {}, this._update.bind(this));
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
  G as Fragment,
  tt as h,
  nt as mount,
  V as utils
};
