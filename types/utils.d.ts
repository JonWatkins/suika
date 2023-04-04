export type MapHas = (value: any) => boolean;
export declare const isUndef: (value: any) => boolean;
export declare const isDef: (value: any) => boolean;
export declare const isObject: (value: any) => boolean;
export declare const isEqual: (a: any, b: any) => boolean;
export declare const makeMap: (str: string, lowerCase?: boolean) => MapHas;
export declare const isReservedTag: (value: any) => boolean;
export declare const isHTMLTag: MapHas;
export declare const isSVG: MapHas;
