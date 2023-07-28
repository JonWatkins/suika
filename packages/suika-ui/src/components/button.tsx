// @ts-nocheck

import { createElement } from "suika";

export const Button = ({ children, onClick }) => {
  return <button onClick={onClick}>{children}</button>;
};
