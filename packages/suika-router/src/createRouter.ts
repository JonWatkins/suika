// @ts-nocheck

let currentMode = "hash";

const escapeRegExp = (string: string): string => {
  return string.replace(/[.*+?^${}()|[\]\\]/g, "\\$&");
};

const pathToRegex = (path: string): RegExp => {
  const escapedPath = escapeRegExp(path);
  return new RegExp(`^${escapedPath.replace(/:\w+/g, "([^/]+)")}$`);
};

export const getCurrentMode = () => {
  return currentMode;
};

export const setCurrentMode = (mode) => {
  currentMode = mode;
};

export const createRouter = ({ mode, routes, NotFound }) => {
  setCurrentMode(mode || "hash");

  const mappedRoutes = routes.map((route: Route) => {
    route.regex = pathToRegex(route.path);
    return route;
  });

  const getFragment = () => {
    let fragment = "";

    if (getCurrentMode() === "history") {
      fragment = decodeURI(window.location.pathname + window.location.search);
      fragment = fragment.replace(/\?(.*)$/, "");
    } else {
      const match = window.location.href.match(/#(.*)$/);
      fragment = match ? match[1] : "/";
    }

    return fragment;
  };

  const getHandler = (path = "") => {
    const result = mappedRoutes.find((route: Route) => {
      return route.regex?.test(path);
    });

    if (result && result.component) {
      return result.component;
    } else {
      return NotFound;
    }
  };

  return {
    getFragment,
    getHandler,
  };
};
