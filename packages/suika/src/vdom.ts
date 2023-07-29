// @ts-nocheck

import { isEvent, isGone, isProperty, isNew, isStyle } from "./utils";

import { TEXT_ELEMENT } from "./globals";

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
    fiber.type == TEXT_ELEMENT
      ? document.createTextNode("")
      : document.createElement(fiber.type);

  updateDom(dom, {}, fiber.props);

  return dom;
};

export const updateDom = (dom, prevProps, nextProps) => {
  // Remove event listeners for props that are no longer present or have changed
  Object.keys(prevProps)
    .filter(isEvent)
    .forEach((name) => {
      if (!(name in nextProps) || prevProps[name] !== nextProps[name]) {
        const eventType = name.toLowerCase().substring(2);
        dom.removeEventListener(eventType, prevProps[name]);
      }
    });

  // Remove properties that are no longer present in nextProps
  Object.keys(prevProps)
    .filter(isProperty)
    .forEach((name) => {
      if (!(name in nextProps)) {
        dom[name] = "";
      }
    });

  // Update properties and styles in batch
  const propUpdates = Object.keys(nextProps).filter(isProperty);
  propUpdates.forEach((name) => {
    if (
      !isGone(prevProps, nextProps)(name) &&
      isNew(prevProps, nextProps)(name)
    ) {
      if (isStyle(name) && typeof nextProps[name] === "object") {
        Object.assign(dom.style, nextProps[name]);
      } else if (prevProps[name] !== nextProps[name]) {
        dom[name] = nextProps[name];
      }
    }
  });

  // Add event listeners in batch
  const eventUpdates = Object.keys(nextProps).filter(isEvent);
  eventUpdates.forEach((name) => {
    if (!(name in prevProps) || prevProps[name] !== nextProps[name]) {
      const eventType = name.toLowerCase().substring(2);
      dom.addEventListener(eventType, nextProps[name]);
    }
  });
};
