import { isObject, vAttrs } from "suika";

export const getTagName = (options: vAttrs, tag = "div"): string => {
  if (isObject(options) && typeof options.is === "string") {
    tag = options.is;
  }
  return tag;
};
