import { Observable, Changes } from "./observable";
export type Listener = (changes: Changes) => void;
export interface ReactiveState {
    _isReactive: boolean;
    _handler: Function;
    _listeners: Listener[];
    addListener: (fn: Listener) => void;
    removeListener: (fn: Listener) => void;
    value?: {
        [_: string]: any;
    };
}
export declare class Reactive implements ReactiveState {
    _isReactive: boolean;
    _listeners: Listener[];
    value: Observable;
    constructor(obj: any);
    _handler(changes: Changes): void;
    addListener(fn: Listener): void;
    removeListener(fn: Listener): void;
}
