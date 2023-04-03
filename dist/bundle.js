function _(t) {
  return Array.isArray(t);
}
function f(t) {
  return t != null;
}
const x = (t) => t !== null && typeof t == "object", l = (t) => typeof t == "function", b = (t) => typeof t == "string";
function g(t, n) {
  const e = /* @__PURE__ */ new Map(), s = t.split(",");
  for (let i = 0; i < s.length; i++)
    e.set(s[i], !0);
  return (i) => e.has(n ? i.toLowerCase() : i);
}
function C(t) {
  return E(t) || S(t);
}
const E = g(
  "html,body,base,head,link,meta,style,title,address,article,aside,footer,header,h1,h2,h3,h4,h5,h6,hgroup,nav,section,div,dd,dl,dt,figcaption,figure,picture,hr,img,li,main,ol,p,pre,ul,a,b,abbr,bdi,bdo,br,cite,code,data,dfn,em,i,kbd,mark,q,rp,rt,rtc,ruby,s,samp,small,span,strong,sub,sup,time,u,var,wbr,area,audio,map,track,video,embed,object,param,source,canvas,script,noscript,del,ins,caption,col,colgroup,table,thead,tbody,td,th,tr,button,datalist,fieldset,form,input,label,legend,meter,optgroup,option,output,progress,select,textarea,details,dialog,menu,menuitem,summary,content,element,shadow,template,blockquote,iframe,tfoot"
), S = g(
  "svg,animate,circle,clippath,cursor,defs,desc,ellipse,filter,font-face,foreignobject,g,glyph,image,line,marker,mask,missing-glyph,path,pattern,polygon,polyline,rect,switch,symbol,text,textpath,tspan,use,view",
  !0
), j = (t, n) => !1;
function d(t, n, e = []) {
  if (!x(t))
    return t;
  const s = (r) => e.concat(r).join(".");
  for (const r in t)
    t[r] = d(
      t[r],
      n,
      e.concat(r)
    );
  const i = (r, a) => {
    const u = Reflect.deleteProperty(r, a);
    return l(n) && n({
      path: s(a),
      target: r,
      name: a
    }), u;
  }, c = (r, a, u, v) => {
    const M = Reflect.set(
      r,
      a,
      d(u, n),
      v
    );
    return l(n) && n({
      path: s(a),
      target: r,
      name: a,
      value: u
    }), M;
  }, h = (r, a, u) => Reflect.get(r, a, u);
  return new Proxy(t, {
    deleteProperty: i,
    set: c,
    get: h
  });
}
const P = () => "fragment", k = (t) => t.prototype instanceof H, F = (t) => t === P && t() === t(), W = (t, n, e) => ({
  kind: "element",
  tag: t,
  attrs: n,
  children: e
}), z = (t, n) => ({
  kind: "component",
  attrs: n,
  component: t
}), A = (t, n, e) => ({
  kind: "function",
  attrs: n,
  component: t,
  children: e
}), D = (t) => ({
  kind: "fragment",
  children: t
}), y = (t) => ({
  kind: "text",
  value: t.toString()
}), R = (t) => t.filter((n) => f(n)).map(
  (n) => b(n) ? y(n) : n
), B = (t, n = {}, ...e) => {
  const s = R(e);
  return b(t) && C(t) ? W(t, n, s) : l(t) ? F(t) ? D(s) : k(t) ? z(t, n) : A(t, n, s) : y(t);
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
}, I = (t, n) => {
  if (!f(t) || !k(t))
    throw new Error("Must pass a component to render");
  if (!f(t) || !(n instanceof Element))
    throw new Error("Must pass a dom node to mount");
  const e = new t();
  e._initState();
  const s = o(e._initVnode({}));
  return e._notifyMounted(s), n.replaceWith(s), e;
};
function m(t, n) {
  if (!t)
    return (i) => {
      const c = o(n);
      return i.replaceWith(c), c;
    };
  if (!n)
    return (i) => {
      _(t.children) && p(t.children, []), i.remove();
    };
  if (t.kind === "text" || n.kind === "text")
    return t.value !== n.value ? (i) => {
      const c = o(n);
      return i.replaceWith(c), c;
    } : (i) => i;
  if (t.kind === "function" && n.kind === "function")
    return m(
      t,
      n.component(n.attrs, n.children)
    );
  if (t.kind === "component" && n.kind === "component" && t.component === n.component && t.instance)
    return n.instance = t.instance, j(t.attrs, n.attrs), n.instance._setAttrs(n.attrs), n.instance._getDiff();
  if (n.kind === "component")
    return n.instance = new n.component(), n.instance._initState(), n.instance._initVnode(n.attrs), (i) => {
      const c = o(n);
      return i.replaceWith(c), n.instance._notifyMounted(i), c;
    };
  if (t.tag !== n.tag)
    return (i) => {
      const c = o(n);
      return _(t.children) && p(t.children, n.children), i.replaceWith(c), c;
    };
  const e = U(t.attrs, n.attrs), s = q(t.children, n.children);
  return (i) => (e(i), s(i), i);
}
function U(t, n) {
  const e = {
    remove: Object.keys(t || {}).filter(
      (s) => !f(n[s])
    ),
    set: Object.keys(n || {}).filter(
      (s) => t[s] !== n[s] && f(n[s])
    ).reduce((s, i) => ({ ...s, [i]: n[i] }), {})
  };
  return (s) => {
    for (const i of e.remove)
      s.removeAttribute(i);
    for (const i in e.set)
      s[i] = e.set[i];
  };
}
function q(t, n) {
  const e = [], s = [];
  t.forEach((i, c) => {
    e.push(m(i, n[c]));
  });
  for (const i of n.slice(t.length))
    s.push((c) => (c.appendChild(o(i)), c));
  return (i) => {
    if (i) {
      for (const [c, h] of L(
        e,
        i.childNodes
      ))
        c(h);
      for (const c of s)
        c(i);
    }
    return i;
  };
}
function p(t, n) {
  let e = t.length;
  for (; e--; ) {
    const s = t[e];
    if (s.kind === "component") {
      const i = O(s, n);
      i && p(
        s.instance._vNode.children,
        i.instance._vNode.children
      ), i || s.instance._unmount();
    }
  }
}
function O(t, n) {
  return n.find(
    (e) => t.component === e.component
  );
}
const L = (t, n) => {
  const e = [];
  for (let s = 0; s < Math.min(t.length, n.length); s++)
    e.push([t[s], n[s]]);
  return e;
};
let G = 0;
class H {
  constructor() {
    this._isSuika = !0, this._uid = G++;
  }
  _update() {
    if (this._el) {
      const n = this._getDiff();
      this._el = n(this._el), this.onUpdated();
    }
  }
  _getDiff() {
    const n = this.render(), e = m(this._vNode, n);
    return this._vNode = n, e;
  }
  _initVnode(n) {
    return this._setAttrs(n), this._vNode = this.render(), this._vNode;
  }
  _initState() {
    this.state = d(this.state || {}, this._update.bind(this));
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
const J = "1.2.4";
export {
  H as Component,
  P as Fragment,
  B as h,
  I as mount,
  J as version
};
