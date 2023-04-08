export interface Observable {
    [_: string]: any;
}
export interface ProxyHandler {
    get: (target: Target, key: string) => any;
    set: (target: Target, key: string, value: any) => boolean;
    deleteProperty: (target: Target, key: string) => boolean;
}
export interface Changes {
    target: Target;
    path: string;
    key: string;
    value?: any;
}
export type Target = {
    [_: string]: any;
};
export declare const observable: (target: any, listener?: Function) => Observable;
