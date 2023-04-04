import { isObject } from "./utils";
export const observable = (target, listener, tree = []) => {
  if (!isObject(target)) return target;
  const getPath = (prop) => tree.concat(prop).join(".");
  for (const property in target) {
    target[property] = observable(
      target[property],
      listener,
      tree.concat(property)
    );
  }
  const deleteProperty = (target, name) => {
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
  const set = (target, name, value, receiver) => {
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
  const get = (target, name, receiver) => {
    return Reflect.get(target, name, receiver);
  };
  return new Proxy(target, {
    deleteProperty,
    set,
    get,
  });
};
//# sourceMappingURL=observable.js.map
