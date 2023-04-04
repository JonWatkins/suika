export interface Observable {
    [_: string]: any;
    deleteProperty: Function;
    set: Function;
    get: Function;
}
export declare const observable: (target: any, listener?: Function, tree?: Array<String>) => Observable;
