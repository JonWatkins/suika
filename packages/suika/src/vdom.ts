// @ts-nocheck

import { isEvent, isGone, isProperty, isNew, isStyle } from "./utils";

import { TEXT_ELEMENT } from "./globals";

export const createElement = (type, props, ...children) => {
  return {
    type,
    props: {
      ...props,
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
    fiber.type == TEXT_ELEMENT
      ? document.createTextNode("")
      : document.createElement(fiber.type);

  updateDom(dom, {}, fiber.props);

  return dom;
};

export const updateDom = (dom, prevProps, nextProps) => {
  Object.keys(prevProps)
    .filter(isEvent)
    .filter((key) => !(key in nextProps) || isNew(prevProps, nextProps)(key))
    .forEach((name) => {
      const eventType = name.toLowerCase().substring(2);
      dom.removeEventListener(eventType, prevProps[name]);
    });

  Object.keys(prevProps)
    .filter(isProperty)
    .filter(isGone(prevProps, nextProps))
    .forEach((name) => {
      dom[name] = "";
    });

  Object.keys(nextProps)
    .filter(isProperty)
    .filter(isNew(prevProps, nextProps))
    .forEach((name) => {
      if (isStyle(name) && typeof nextProps[name] === "object") {
        Object.assign(dom.style, nextProps[name]);
      } else {
        dom[name] = nextProps[name];
      }
    });

  Object.keys(nextProps)
    .filter(isEvent)
    .filter(isNew(prevProps, nextProps))
    .forEach((name) => {
      const eventType = name.toLowerCase().substring(2);
      dom.addEventListener(eventType, nextProps[name]);
    });
};
