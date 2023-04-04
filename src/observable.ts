import { isObject } from "./utils";

export interface Observable {
  [_: string]: any;
  deleteProperty: Function;
  set: Function;
  get: Function;
}

export const observable = (
  target: any,
  listener?: Function,
  tree: Array<String> = []
): Observable => {
  if (!isObject(target)) return target;

  const getPath = (prop: String): String => tree.concat(prop).join(".");

  for (const property in target) {
    target[property] = observable(
      target[property],
      listener,
      tree.concat(property)
    );
  }

  const deleteProperty = (target: object, name: string) => {
    const res = Reflect.deleteProperty(target, name);

    if (typeof listener === "function") {
      listener({
        path: getPath(name),
        target,
        name,
      });
    }

    return res;
  };

  const set = (target: object, name: string, value: any, receiver: any) => {
    const res = Reflect.set(
      target,
      name,
      observable(value, listener),
      receiver
    );

    if (typeof listener === "function") {
      listener({
        path: getPath(name),
        target,
        name,
        value,
      });
    }

    return res;
  };

  const get = (target: object, name: string, receiver: any) => {
    return Reflect.get(target, name, receiver);
  };

  return new Proxy(target, {
    deleteProperty,
    set,
    get,
  });
};
