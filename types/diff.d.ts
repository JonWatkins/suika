export declare function diff(
  oldVTree: object | null,
  newVTree: object | null
): Function;
export declare function diffAttrs(
  oldAttrs?: {},
  newAttrs?: {}
): (node: any) => void;
export declare function diffChildNodes(
  oldChildNodes: any,
  newChildNodes: any
): (parent: HTMLElement) => HTMLElement;
export declare function unmountChildNodes(
  oldChildNodes: any,
  newChildNodes: any
): void;
export declare function findNewChildComponent(
  oldNode: any,
  newChildNodes: any
): any;
export declare const zip: (xs: any, ys: any) => any[][];
