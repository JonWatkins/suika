export type MapHas = (value: any) => boolean;

export function isUndef(value: any): boolean {
  return value === undefined || value === null;
}

export function isDef(value: any): boolean {
  return value !== undefined && value !== null;
}

export const isObject = (value: any): boolean => {
  return value !== null && typeof value === "object";
};

export const isEqual = (a: any, b: any): boolean => {
  return a && b && isObject(a) && isObject(b) && typeof a === typeof b
    ? Object.keys(a).length === Object.keys(b).length &&
        Object.keys(a).every((key) => isEqual(a[key], b[key]))
    : a === b;
};

export default function makeMap(str: string, lowerCase?: boolean): MapHas {
  const map = new Map();
  const list = str.split(",");

  for (let i = 0; i < list.length; i++) {
    map.set(list[i], true);
  }

  return (i: string): boolean => {
    return map.has(lowerCase ? i.toLowerCase() : i);
  };
}

export function isReservedTag(value: any): boolean {
  return isHTMLTag(value) || isSVG(value);
}

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
