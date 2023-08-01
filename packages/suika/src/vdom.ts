// @ts-nocheck

import { TEXT_ELEMENT } from "./globals";
import { isProperty, isDef, isUndef, isEvent, eventName } from "./utils";

let nextKey = 0;

export const createElement = (type, props, ...children) => {
  // Add a unique key prop to each element
  const key = (props && props.key) || nextKey++;
  return {
    type,
    props: {
      ...props,
      key,
      children: children
        .flat()
        .map((child) =>
          typeof child === "object" ? child : createTextElement(child),
        ),
    },
  };
};

export const createTextElement = (text) => {
  return {
    type: TEXT_ELEMENT,
    props: {
      nodeValue: text,
      children: [],
    },
  };
};

export const createDom = (fiber) => {
  const dom =
    fiber.type === TEXT_ELEMENT
      ? document.createTextNode("")
      : document.createElement(fiber.type);

  updateDom(dom, {}, fiber.props);

  return dom;
};

export const updateDom = (dom, prevProps, nextProps) => {
  const remove = Object.keys(prevProps)
    .filter(isProperty)
    .filter((attr) => isUndef(nextProps[attr]));

  const set = Object.keys(nextProps)
    .filter(isProperty)
    .filter(
      (attr) => prevProps[attr] !== nextProps[attr] && isDef(nextProps[attr]),
    )
    .reduce((updated, attr) => ({ ...updated, [attr]: nextProps[attr] }), {});

  for (const attr of remove) {
    if (isEvent(attr)) {
      dom.removeEventListener(eventName(attr), remove[attr]);
    } else {
      dom.removeAttribute(attr);
    }
  }

  for (const attr in set) {
    if (isEvent(attr)) {
      dom.addEventListener(eventName(attr), set[attr]);
    } else {
      dom[attr] = set[attr];
    }
  }
};
