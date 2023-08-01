// @ts-nocheck

import { createElement } from "suika";
import { getCurrentMode } from "./createRouter";

const preventReload = (to) => {
  return (event) => {
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
};

export const Link = ({ to, children, className }) => {
  return createElement(
    "a",
    { href: to, className, onClick: preventReload(to) },
    ...children,
  );
};
