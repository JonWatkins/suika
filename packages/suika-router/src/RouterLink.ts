import { h, fixOptions, vNode, vAttrs } from "suika";
import { navigate } from "./utils";

export const RouterLink = (options: vAttrs, children: vNode[]): vNode => {
  const props = fixOptions(options, {
    onclick: (e: Event) => {
      e.preventDefault();
      navigate(options.to || "/");
    },
  });
  return h("a", props, ...children);
};
