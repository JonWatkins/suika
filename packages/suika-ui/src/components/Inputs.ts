import { h, fixOptions, vNode, vAttrs } from "suika";

export const Input = (options: vAttrs = {}): vNode => {
  const props = fixOptions(options, {
    className: "form-control",
  });

  return h("input", props);
};

export const InputGroup = (
  options: vAttrs = {},
  children: vNode[] = []
): vNode => {
  const props = fixOptions(options, {
    className: "input-group",
  });

  return h("div", props, ...children);
};
