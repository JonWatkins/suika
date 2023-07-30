// @ts-nocheck

import { createElement } from "suika";

export const Button = ({ children, className, onClick }) => {
  const classes = `btn btn-md btn-secondary ${className}`;
  return createElement("button", { className: classes, onClick }, children);
};
