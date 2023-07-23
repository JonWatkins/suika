export const escapeRegExp = (string: string): string => {
  return string.replace(/[.*+?^${}()|[\]\\]/g, "\\$&");
};

export const pathToRegex = (path: string): RegExp => {
  const escapedPath = escapeRegExp(path);
  return new RegExp(`^${escapedPath.replace(/:\w+/g, "([^/]+)")}$`);
};

export const navigate = (path = "", mode = "hash"): void => {
  if (mode === "history") {
    window.history.pushState(null, "", path);
  } else {
    window.location.href = `${window.location.href.replace(
      /#(.*)$/,
      ""
    )}#${path}`;
  }
};
