var O = Object.defineProperty;
var P = (t, n, e) => n in t ? O(t, n, { enumerable: !0, configurable: !0, writable: !0, value: e }) : t[n] = e;
var d = (t, n, e) => (P(t, typeof n != "symbol" ? n + "" : n, e), e);
const M = (t) => t == null, h = (t) => t != null, _ = (t) => h(t) && typeof t == "object", y = (t, n) => t && n && _(t) && _(n) && typeof t == typeof n ? Object.keys(t).length === Object.keys(n).length && Object.keys(t).every((e) => y(t[e], n[e])) : t === n, b = (t, n) => {
  const e = /* @__PURE__ */ new Map(), s = t.split(",");
  for (let i = 0; i < s.length; i++)
    e.set(s[i], !0);
  return (i) => e.has(n ? i.toLowerCase() : i);
}, j = (t) => x(t) || C(t), x = b(
  "html,body,base,head,link,meta,style,title,address,article,aside,footer,header,h1,h2,h3,h4,h5,h6,hgroup,nav,section,div,dd,dl,dt,figcaption,figure,picture,hr,img,li,main,ol,p,pre,ul,a,b,abbr,bdi,bdo,br,cite,code,data,dfn,em,i,kbd,mark,q,rp,rt,rtc,ruby,s,samp,small,span,strong,sub,sup,time,u,var,wbr,area,audio,map,track,video,embed,object,param,source,canvas,script,noscript,del,ins,caption,col,colgroup,table,thead,tbody,td,th,tr,button,datalist,fieldset,form,input,label,legend,meter,optgroup,option,output,progress,select,textarea,details,dialog,menu,menuitem,summary,content,element,shadow,template,blockquote,iframe,tfoot",
  !0
), C = b(
  "svg,animate,circle,clippath,cursor,defs,desc,ellipse,filter,font-face,foreignobject,g,glyph,image,line,marker,mask,missing-glyph,path,pattern,polygon,polyline,rect,switch,symbol,text,textpath,tspan,use,view",
  !0
), J = /* @__PURE__ */ Object.freeze(/* @__PURE__ */ Object.defineProperty({
  __proto__: null,
  isDef: h,
  isEqual: y,
  isHTMLTag: x,
  isObject: _,
  isReservedTag: j,
  isSVG: C,
  isUndef: M,
  makeMap: b
}, Symbol.toStringTag, { value: "Module" })), g = (t, n, e = []) => {
  if (!_(t))
    return t;
  const s = (o) => e.concat(o).join(".");
  for (const o in t)
    t[o] = g(
      t[o],
      n,
      e.concat(o)
    );
  const i = (o, u) => {
    const f = Reflect.deleteProperty(o, u);
    return typeof n == "function" && n({
      path: s(u),
      target: o,
      name: u
    }), f;
  }, c = (o, u, f, r) => {
    const a = Reflect.set(
      o,
      u,
      g(f, n),
      r
    );
    return typeof n == "function" && n({
      path: s(u),
      target: o,
      name: u,
      value: f
    }), a;
  }, m = (o, u, f) => Reflect.get(o, u, f);
  return new Proxy(t, {
    deleteProperty: i,
    set: c,
    get: m
  });
}, z = () => "fragment", S = (t) => t.prototype instanceof G, A = (t) => t === z && t() === t(), U = (t, n, e) => ({
  kind: "element",
  tag: t,
  attrs: n,
  children: e
}), W = (t, n) => ({
  kind: "component",
  instance: void 0,
  attrs: n,
  component: t
}), D = (t, n, e) => ({
  kind: "function",
  attrs: n,
  component: t,
  children: e
}), F = (t) => ({
  kind: "fragment",
  tag: "fragment",
  children: t,
  attrs: {}
}), E = (t) => ({
  kind: "text",
  value: t.toString()
}), R = (t) => t.filter((n) => h(n)).map((n) => {
  let e;
  return typeof n == "string" ? e = E(n) : e = n, e;
}), K = (t, n = {}, ...e) => {
  const s = R(e);
  return typeof t == "string" && j(t) ? U(t, n, s) : typeof t == "function" ? A(t) ? F(s) : S(t) ? W(t, n) : D(t, n, s) : E(t);
}, l = (t) => {
  if (t.kind === "text")
    return document.createTextNode(t.value);
  if (t.kind === "function")
    return l(t.component(t.attrs, t.children));
  if (t.kind === "component") {
    if (t.instance) {
      const i = l(t.instance.render());
      return t.instance._notifyMounted(i), i;
    }
    t.instance = new t.component(), t.instance._initState();
    const e = t.instance._initVnode(t.attrs), s = l(e);
    return t.instance._notifyMounted(s), s;
  }
  const n = document.createElement(t.tag);
  for (const e in t.attrs)
    n[e] = t.attrs[e];
  return t.children.forEach((e) => {
    n.appendChild(l(e));
  }), n;
}, Q = (t, n) => {
  if (!h(t) || !S(t))
    throw new Error("Must pass a component to mount");
  if (!h(t) || !(n instanceof HTMLElement))
    throw new Error("Must pass a dom node to mount");
  const e = new t();
  e._initState();
  const s = l(e._initVnode({}));
  return e._notifyMounted(s), n.replaceWith(s), e;
}, k = (t, n) => {
  if (!t)
    return (r) => {
      if (n) {
        const a = l(n);
        return r.replaceWith(a), a;
      } else
        return r;
    };
  if (!n)
    return (r) => (t.kind, r.remove(), null);
  if (t.kind === "text" || n.kind === "text") {
    const { value: r } = t, { value: a } = n;
    return r !== a ? (p) => {
      const v = l(n);
      return p.replaceWith(v), v;
    } : (p) => p;
  }
  if (t.kind === "function" && n.kind === "function")
    return k(
      t,
      n.component(n.attrs, n.children)
    );
  if (t.kind === "component" && n.kind === "component" && t.component === n.component && t.instance)
    return n.instance = t.instance, y(t.attrs, n.attrs) ? (r) => r : (n.instance._setAttrs(n.attrs), n.instance._getDiff());
  if (n.kind === "component") {
    const r = new n.component();
    return n.instance, r._initState(), r._initVnode(n.attrs), (a) => {
      const p = l(n);
      return a.replaceWith(p), r._notifyMounted(a), p;
    };
  }
  const {
    tag: e,
    children: s,
    attrs: i
  } = t, {
    tag: c,
    children: m,
    attrs: o
  } = n;
  if (e !== c)
    return (r) => {
      const a = l(n);
      return r.replaceWith(a), a;
    };
  const u = q(i, o), f = L(s, m);
  return (r) => (u(r), f(r), r);
}, q = (t, n) => {
  const e = {
    remove: Object.keys(t).filter((s) => M(n[s])),
    set: Object.keys(n).filter(
      (s) => t[s] !== n[s] && h(n[s])
    ).reduce((s, i) => ({ ...s, [i]: n[i] }), {})
  };
  return (s) => {
    for (const i of e.remove)
      s.removeAttribute(i);
    for (const i in e.set)
      s[i] = e.set[i];
  };
}, L = (t, n) => {
  const e = [], s = [];
  t.forEach((i, c) => {
    e.push(k(i, n[c]));
  });
  for (const i of n.slice(t.length))
    s.push((c) => (c.appendChild(l(i)), c));
  return (i) => {
    for (const [c, m] of H(
      e,
      Array.from(i.childNodes)
    ))
      c(m);
    for (const c of s)
      c(i);
    return i;
  };
}, H = (t, n) => {
  const e = [];
  for (let s = 0; s < Math.min(t.length, n.length); s++)
    e.push([t[s], n[s]]);
  return e;
};
let B = 0;
class G {
  constructor() {
    d(this, "_uid");
    d(this, "_el");
    d(this, "_vNode");
    d(this, "_mounted");
    d(this, "_isSuika");
    d(this, "state");
    d(this, "attrs");
    this._uid = B++, this._el = null, this._vNode = null, this._mounted = !1, this._isSuika = !0, this.state = {}, this.attrs = {};
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
    this._setAttrs(n);
    const e = this.render();
    return this._vNode = e, e;
  }
  _initState() {
    this.state = g(this.state, this._update.bind(this));
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
  G as Component,
  z as Fragment,
  K as h,
  Q as mount,
  J as utils
};
