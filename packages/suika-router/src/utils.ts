export const pathToRegex = (path: string): RegExp => {
  return new RegExp(`^${path.replace(/\//g, "\\/").replace(/:\w+/g, "(.+)")}$`);
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
