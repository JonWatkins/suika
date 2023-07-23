import { h, fixOptions, vNode, vAttrs } from "suika";

export const ListGroup = (options: vAttrs, children: vNode[]): vNode => {
  const props = fixOptions(options, {
    className: "list-group",
  });

  return h("ul", props, ...children);
};

export const ListGroupItem = (options: vAttrs, children: vNode[]): vNode => {
  const props = fixOptions(options, {
    className: "list-group-item",
  });

  return h("li", props, ...children);
};
