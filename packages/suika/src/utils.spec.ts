import { isEvent, isStyle, isProperty, isNew, isGone } from "./utils";

describe("Unit tests for utility functions", () => {
  test("isEvent", () => {
    expect(isEvent("onClick")).toBe(true);
    expect(isEvent("onMouseOver")).toBe(true);
    expect(isEvent("onInput")).toBe(true);

    expect(isEvent("className")).toBe(false);
    expect(isEvent("style")).toBe(false);
    expect(isEvent("children")).toBe(false);
  });

  test("isStyle", () => {
    expect(isStyle("style")).toBe(true);

    expect(isStyle("className")).toBe(false);
    expect(isStyle("onClick")).toBe(false);
    expect(isStyle("children")).toBe(false);
  });

  test("isProperty", () => {
    expect(isProperty("className")).toBe(true);
    expect(isProperty("src")).toBe(true);

    expect(isProperty("onClick")).toBe(false);
    expect(isProperty("style")).toBe(true);
    expect(isProperty("children")).toBe(false);
  });

  test("isNew", () => {
    const onClick = () => ({});
    const prev = { className: "old", onClick };
    const next = { className: "new", onClick };

    expect(isNew(prev, next)("className")).toBe(true);
    expect(isNew(prev, next)("onClick")).toBe(false);

    // Non-existing key should not be considered new
    expect(isNew(prev, next)("style")).toBe(false);
  });

  test("isGone", () => {
    const prev = { className: "old", onClick: () => {} };
    const next = { className: "new", onClick: () => {} };

    expect(isGone(prev, next)("className")).toBe(false);
    expect(isGone(prev, next)("style")).toBe(true);

    // Non-existing key in next should be considered gone
    expect(isGone(prev, next)("onMouseOver")).toBe(true);
  });
});
