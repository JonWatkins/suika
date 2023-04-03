export declare function diff(oldVTree: any, newVTree: any): any;
export declare function diffAttrs(oldAttrs: any, newAttrs: any): (node: any) => void;
export declare function diffChildNodes(oldChildNodes: any, newChildNodes: any): (parent: any) => any;
export declare function unmountChildNodes(oldChildNodes: any, newChildNodes: any): void;
export declare function findNewChildComponent(oldNode: any, newChildNodes: any): any;
export declare const zip: (xs: any, ys: any) => any[][];
