import { makeMap, isUndef, Ctor, MapHas } from "suika";
import { NotFound } from "./NotFound";
import { navigate, pathToRegex } from "./utils";

export type RouterOptions = {
  routes: Route[];
  mode: string;
};

export interface Route {
  component: Ctor;
  path: string;
  regex?: RegExp;
}

const modes: MapHas = makeMap("history,pushstate");

export class Router {
  routes: Route[];
  mode: string;
  current: string;

  constructor(options: RouterOptions) {
    this.routes = options.routes;
    this.mode = options.mode || "hash";
    this.current = this.getFragment();
  }

  public getFragment(): string {
    let fragment = "";

    if (this.mode === "history") {
      fragment = decodeURI(window.location.pathname + window.location.search);
      fragment = fragment.replace(/\?(.*)$/, "");
    } else {
      const match = window.location.href.match(/#(.*)$/);
      fragment = match ? match[1] : "/";
    }

    return fragment;
  }

  public getHandler(path = ""): Ctor {
    const result = this.routes.find((route: Route) => {
      return route.regex?.test(path);
    });

    if (result && result.component) {
      return result.component;
    } else {
      return NotFound;
    }
  }

  public navigate(path = "") {
    navigate(path, this.mode);
  }
}

export const createRouter = (options: RouterOptions) => {
  if (isUndef(options.mode) || !modes(options.mode)) {
    options.mode = "hash";
  }

  if (Array.isArray(options.routes)) {
    options.routes = options.routes.map((route: Route) => {
      route.regex = pathToRegex(route.path);
      return route;
    });
  }

  return new Router(options);
};
