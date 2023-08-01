// @ts-nocheck

type MemoizeCache<T> = Map<unknown, T>;

export function getParameterCount(func: Function): number {
  const stringRepresentation = func
    .toString()
    .replace(/((\/\/.*$)|(\/\*[\s\S]*?\*\/))/gm, "");
  const parameterList = stringRepresentation.match(
    /(?:function|)\s*\w*\s*\(([^)]*)\)/,
  )![1];
  if (/\.{3}/.test(parameterList)) {
    throw new Error("Rest parameters are not supported");
  }
  return parameterList.split(",").length;
}

export function memoize<F extends (...args: unknown[]) => unknown>(func: F): F {
  if (typeof func !== "function") {
    throw new TypeError("Can only memoize functions");
  }
  const parameterCount = getParameterCount(func);
  const cache: MemoizeCache<ReturnType<F>> = new Map();

  if (parameterCount === 1) {
    return function (arg: unknown) {
      const key = Symbol(arg);
      if (cache.has(key)) {
        return cache.get(key)!;
      }
      const result = func(arg);
      cache.set(key, result);
      return result;
    } as F;
  }

  return function (...args: Parameters<F>) {
    const key = Symbol(JSON.stringify(args));
    if (cache.has(key)) {
      return cache.get(key)!;
    }
    const result = func(...args);
    cache.set(key, result);
    return result;
  } as F;
}

export const isEvent = (key) => key.startsWith("on");

export const isStyle = (key) => key === "style";

export const isProperty = (key) => key !== "children";

export const isNew = (prev, next) => (key) => prev[key] !== next[key];

export const isGone = (prev, next) => (key) => !(key in next);

export const isUndef = (value) => value === undefined || value === null;

export const isDef = (value): boolean => value !== undefined && value !== null;

export const eventName = (value): string => value.toLowerCase().substring(2);
