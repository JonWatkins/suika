export declare const Fragment: () => string;
export declare const isComponent: (value: any) => boolean;
export declare const isFragment: (value: any) => boolean;
export declare const createElement: (tag: any, attrs: any, children: any) => {
    kind: string;
    tag: any;
    attrs: any;
    children: any;
};
export declare const createComponent: (component: any, attrs: any) => {
    kind: string;
    attrs: any;
    component: any;
};
export declare const createFunction: (component: any, attrs: any, children: any) => {
    kind: string;
    attrs: any;
    component: any;
    children: any;
};
export declare const createFragment: (children: any) => {
    kind: string;
    children: any;
};
export declare const createText: (value: any) => {
    kind: string;
    value: any;
};
export declare const normalizeChildNodes: (childNodes: any) => any;
export declare const h: (tag: any, attrs?: {}, ...children: any[]) => {
    kind: string;
    attrs: any;
    component: any;
} | {
    kind: string;
    children: any;
} | {
    kind: string;
    value: any;
};
