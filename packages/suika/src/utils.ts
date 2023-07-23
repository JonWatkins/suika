export type MapHas = (value: any) => boolean;

export type ElementOptions = {
  [_: string]: any;
};

export const isUndef = (value: any): boolean => {
  return value === undefined || value === null;
};

export const isDef = (value: any): boolean => {
  return value !== undefined && value !== null;
};

export const isObject = (value: any): boolean => {
  return isDef(value) && typeof value === "object";
};

export const isEqual = (a: any, b: any): boolean => {
  return a && b && isObject(a) && isObject(b) && typeof a === typeof b
    ? Object.keys(a).length === Object.keys(b).length &&
        Object.keys(a).every((key) => isEqual(a[key], b[key]))
    : a === b;
};

export const zip = (xs: Array<any>, ys: Array<any>) => {
  const zipped = [];
  for (let i = 0; i < Math.min(xs.length, ys.length); i++) {
    zipped.push([xs[i], ys[i]]);
  }
  return zipped;
};

export const makeMap = (str: string, lowerCase?: boolean): MapHas => {
  const map = new Map();
  const list = str.split(",");

  for (let i = 0; i < list.length; i++) {
    map.set(list[i], true);
  }

  return (i: string): boolean => {
    return map.has(lowerCase ? i.toLowerCase() : i);
  };
};

export const isReservedTag = (value: any): boolean => {
  return isHTMLTag(value) || isSVG(value);
};

export const isHTMLTag: MapHas = makeMap(
  "html,body,base,head,link,meta,style,title," +
    "address,article,aside,footer,header,h1,h2,h3,h4,h5,h6,hgroup,nav,section," +
    "div,dd,dl,dt,figcaption,figure,picture,hr,img,li,main,ol,p,pre,ul," +
    "a,b,abbr,bdi,bdo,br,cite,code,data,dfn,em,i,kbd,mark,q,rp,rt,rtc,ruby," +
    "s,samp,small,span,strong,sub,sup,time,u,var,wbr,area,audio,map,track,video," +
    "embed,object,param,source,canvas,script,noscript,del,ins," +
    "caption,col,colgroup,table,thead,tbody,td,th,tr," +
    "button,datalist,fieldset,form,input,label,legend,meter,optgroup,option," +
    "output,progress,select,textarea," +
    "details,dialog,menu,menuitem,summary," +
    "content,element,shadow,template,blockquote,iframe,tfoot",
  true
);

export const isSVG: MapHas = makeMap(
  "svg,animate,circle,clippath,cursor,defs,desc,ellipse,filter,font-face," +
    "foreignobject,g,glyph,image,line,marker,mask,missing-glyph,path,pattern," +
    "polygon,polyline,rect,switch,symbol,text,textpath,tspan,use,view",
  true
);

export const mergeClassNames = (className: string, classNames: string) => {
  return `${className} ${classNames}`;
};

export const fixOptions = (
  options: ElementOptions,
  defaultOptions?: ElementOptions | undefined
): ElementOptions => {
  const merged: ElementOptions = {};
  const defaults =
    typeof defaultOptions === "object"
      ? defaultOptions
      : ({} as ElementOptions);

  for (const [key, value] of Object.entries(defaults)) {
    merged[key] = value;
  }

  if (isObject(options)) {
    for (const [key, value] of Object.entries(options)) {
      if (key === "is") continue;
      if (key === "className") {
        merged[key] = mergeClassNames(value, merged[key]);
      } else {
        merged[key] = value;
      }
    }
  }

  return merged;
};
