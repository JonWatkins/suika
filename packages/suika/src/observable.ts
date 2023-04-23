import { isObject } from "./utils";

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

export const observable = (target: any, listener?: Function): Observable => {
  if (!isObject(target)) return target;

  const createHandler = (path: Array<String> = []): ProxyHandler => {
    const getPath = (prop: String): String => {
      return path.concat(prop).join(".");
    };

    return {
      get(target: Target, key: string): any {
        if (key == "isProxy") return true;

        const prop = target[key];

        if (typeof prop == "undefined") {
          return;
        }

        if (!prop.isProxy && typeof prop === "object") {
          target[key] = new Proxy(target[key], createHandler(path.concat(key)));
        }

        return target[key];
      },

      set(target: Target, key: string, value: any): boolean {
        target[key] = value;

        if (typeof listener === "function") {
          listener({
            path: getPath(key),
            target,
            key,
            value,
          } as Changes);
        }

        return true;
      },

      deleteProperty(target: Target, key: string): boolean {
        delete target[key];

        if (typeof listener === "function") {
          listener({
            path: getPath(key),
            target,
            key,
          } as Changes);
        }

        return true;
      },
    };
  };

  return new Proxy(target, createHandler());
};
