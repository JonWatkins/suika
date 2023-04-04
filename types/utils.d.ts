export type MapHas = (value: any) => boolean;
export declare function isUndef(value: any): boolean;
export declare function isDef(value: any): boolean;
export declare const isObject: (value: any) => boolean;
export declare const isEqual: (a: any, b: any) => boolean;
export default function makeMap(str: string, lowerCase?: boolean): MapHas;
export declare function isReservedTag(value: any): boolean;
export declare const isHTMLTag: MapHas;
export declare const isSVG: MapHas;
