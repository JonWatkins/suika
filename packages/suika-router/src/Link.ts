// @ts-nocheck

import { createElement } from "suika";
import { getCurrentMode } from "./createRouter";

export const Link = ({ to, children, className }) => {
  const preventReload = (event) => {
    event.preventDefault();

    if (getCurrentMode() === "history") {
      window.history.pushState(null, "", to);
    } else {
      window.location.href = `${window.location.href.replace(
        /#(.*)$/,
        "",
      )}#${to}`;
    }
  };

  return createElement(
    "a",
    { href: to, className, onClick: (event) => preventReload(event) },
    ...children,
  );
};
