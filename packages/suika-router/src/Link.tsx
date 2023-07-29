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

  return (
    <a
      href={to}
      className={className}
      onClick={(event) => preventReload(event)}
    >
      {children}
    </a>
  );
};
