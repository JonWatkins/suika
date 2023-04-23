import { observable, Observable, Changes } from "./observable";

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

export class Reactive implements ReactiveState {
  _isReactive: boolean;
  _listeners: Listener[];
  value: Observable;

  constructor(obj: any) {
    this._isReactive = true;
    this._listeners = [];
    this.value = observable(obj, this._handler.bind(this));
  }

  _handler(changes: Changes): void {
    if (!changes.path.endsWith(".length")) {
      this._listeners.forEach((listener) => {
        listener(changes);
      });
    }
  }

  addListener(fn: Listener): void {
    if (typeof fn === "function") {
      this._listeners.push(fn);
    }
  }

  removeListener(fn: Listener): void {
    const idx = this._listeners.findIndex((i) => i === fn);
    if (idx > -1) {
      this._listeners.splice(idx, 1);
    }
  }
}
