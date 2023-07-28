// @ts-nocheck

import { createElement } from "suika";

export const Button = ({ children, className, onClick }) => {
  return createElement("button", { className, onClick }, children);
};
