import makeMap, * as utils from "../../src/lib/utils";

const HTML_ELEMENTS = [
  "html",
  "body",
  "base",
  "head",
  "link",
  "meta",
  "style",
  "title",
  "address",
  "article",
  "aside",
  "footer",
  "header",
  "h1",
  "h2",
  "h3",
  "h4",
  "h5",
  "h6",
  "hgroup",
  "nav",
  "section",
  "div",
  "dd",
  "dl",
  "dt",
  "figcaption",
  "figure",
  "picture",
  "hr",
  "img",
  "li",
  "main",
  "ol",
  "p",
  "pre",
  "ul",
  "a",
  "b",
  "abbr",
  "bdi",
  "bdo",
  "br",
  "cite",
  "code",
  "data",
  "dfn",
  "em",
  "i",
  "kbd",
  "mark",
  "q",
  "rp",
  "rt",
  "rtc",
  "ruby",
  "s",
  "samp",
  "small",
  "span",
  "strong",
  "sub",
  "sup",
  "time",
  "u",
  "var",
  "wbr",
  "area",
  "audio",
  "map",
  "track",
  "video",
  "embed",
  "object",
  "param",
  "source",
  "canvas",
  "script",
  "noscript",
  "del",
  "ins",
  "caption",
  "col",
  "colgroup",
  "table",
  "thead",
  "tbody",
  "td",
  "th",
  "tr",
  "button",
  "datalist",
  "fieldset",
  "form",
  "input",
  "label",
  "legend",
  "meter",
  "optgroup",
  "option",
  "output",
  "progress",
  "select",
  "textarea",
  "details",
  "dialog",
  "menu",
  "menuitem",
  "summary",
  "content",
  "element",
  "shadow",
  "template",
  "blockquote",
  "iframe",
  "tfoot",
];

const SVG_ELEMENTS = [
  "svg",
  "animate",
  "circle",
  "clippath",
  "cursor",
  "defs",
  "desc",
  "ellipse",
  "filter",
  "font-face",
  "foreignobject",
  "g",
  "glyph",
  "image",
  "line",
  "marker",
  "mask",
  "missing-glyph",
  "path",
  "pattern",
  "polygon",
  "polyline",
  "rect",
  "switch",
  "symbol",
  "text",
  "textpath",
  "tspan",
  "use",
  "view",
];

describe("utils", () => {
  describe("utils.isObject", () => {
    it("should have be able to check if a value is an object", () => {
      expect(utils.isObject(1)).toEqual(false);
      expect(utils.isObject("1")).toEqual(false);
      expect(utils.isObject(() => ({}))).toEqual(false);
      expect(utils.isObject([])).toEqual(true);
      expect(utils.isObject({})).toEqual(true);
      expect(utils.isObject(/a/i)).toEqual(true);
    });
  });

  describe("utils.isPlainObject", () => {
    it("should have be able to check if a value is an object", () => {
      expect(utils.isPlainObject(1)).toEqual(false);
      expect(utils.isPlainObject("1")).toEqual(false);
      expect(utils.isPlainObject(() => ({}))).toEqual(false);
      expect(utils.isPlainObject([])).toEqual(false);
      expect(utils.isPlainObject(/a/i)).toEqual(false);
      expect(utils.isPlainObject({})).toEqual(true);
    });
  });

  describe("utils.isArray", () => {
    it("should have be able to check if a value is an array", () => {
      expect(utils.isArray(1)).toEqual(false);
      expect(utils.isArray("1")).toEqual(false);
      expect(utils.isArray({})).toEqual(false);
      expect(utils.isArray(() => ({}))).toEqual(false);
      expect(utils.isArray([])).toEqual(true);
    });
  });

  describe("utils.isFunction", () => {
    it("should have be able to check if a value is a function", () => {
      expect(utils.isFunction(1)).toEqual(false);
      expect(utils.isFunction("1")).toEqual(false);
      expect(utils.isFunction({})).toEqual(false);
      expect(utils.isFunction([])).toEqual(false);
      expect(utils.isFunction(() => ({}))).toEqual(true);
    });
  });

  describe("utils.isString", () => {
    it("should have be able to check if a value is a function", () => {
      expect(utils.isString(1)).toEqual(false);
      expect(utils.isString({})).toEqual(false);
      expect(utils.isString([])).toEqual(false);
      expect(utils.isString(() => ({}))).toEqual(false);
      expect(utils.isString("1")).toEqual(true);
    });
  });

  describe("utils.isRegExp", () => {
    it("should have be able to check if a value is a function", () => {
      expect(utils.isRegExp(1)).toEqual(false);
      expect(utils.isRegExp({})).toEqual(false);
      expect(utils.isRegExp([])).toEqual(false);
      expect(utils.isRegExp(() => ({}))).toEqual(false);
      expect(utils.isRegExp("1")).toEqual(false);
      expect(utils.isRegExp(/a/i)).toEqual(true);
    });
  });

  describe("utils.isUndef", () => {
    it("should have be able to check if a value is a function", () => {
      expect(utils.isUndef(1)).toEqual(false);
      expect(utils.isUndef({})).toEqual(false);
      expect(utils.isUndef([])).toEqual(false);
      expect(utils.isUndef(() => ({}))).toEqual(false);
      expect(utils.isUndef("1")).toEqual(false);
      expect(utils.isUndef(/a/i)).toEqual(false);
      expect(utils.isUndef(null)).toEqual(true);
      expect(utils.isUndef(undefined)).toEqual(true);
    });
  });

  describe("utils.isDef", () => {
    it("should have be able to check if a value is a function", () => {
      expect(utils.isDef(null)).toEqual(false);
      expect(utils.isDef(undefined)).toEqual(false);
      expect(utils.isDef(1)).toEqual(true);
      expect(utils.isDef({})).toEqual(true);
      expect(utils.isDef([])).toEqual(true);
      expect(utils.isDef(() => ({}))).toEqual(true);
      expect(utils.isDef("1")).toEqual(true);
      expect(utils.isDef(/a/i)).toEqual(true);
    });
  });

  describe("utils.isTrue", () => {
    it("should have be able to check if a value is a function", () => {
      expect(utils.isTrue(null)).toEqual(false);
      expect(utils.isTrue(undefined)).toEqual(false);
      expect(utils.isTrue(1)).toEqual(false);
      expect(utils.isTrue({})).toEqual(false);
      expect(utils.isTrue([])).toEqual(false);
      expect(utils.isTrue(() => ({}))).toEqual(false);
      expect(utils.isTrue("1")).toEqual(false);
      expect(utils.isTrue(/a/i)).toEqual(false);
      expect(utils.isTrue(false)).toEqual(false);
      expect(utils.isTrue(true)).toEqual(true);
    });
  });

  describe("utils.isFalse", () => {
    it("should have be able to check if a value is a function", () => {
      expect(utils.isFalse(null)).toEqual(false);
      expect(utils.isFalse(undefined)).toEqual(false);
      expect(utils.isFalse(1)).toEqual(false);
      expect(utils.isFalse({})).toEqual(false);
      expect(utils.isFalse([])).toEqual(false);
      expect(utils.isFalse(() => ({}))).toEqual(false);
      expect(utils.isFalse("1")).toEqual(false);
      expect(utils.isFalse(/a/i)).toEqual(false);
      expect(utils.isFalse(true)).toEqual(false);
      expect(utils.isFalse(false)).toEqual(true);
    });
  });

  describe("hasOwn", () => {
    it("should return true if an object has a property of its own", () => {
      const obj = { foo: "bar" };
      expect(utils.hasOwn(obj, "foo")).toBe(true);
    });

    it("should return false if an object does not have a property of its own", () => {
      const obj = { foo: "bar" };
      expect(utils.hasOwn(obj, "bar")).toBe(false);
      expect(utils.hasOwn(obj, "hasOwnProperty")).toBe(false);
    });
  });

  describe("utils.makeMap", () => {
    it("should be able to make a map object from a string", () => {
      const str = "one,two,four";
      const map = makeMap(str);
      expect(map("one")).toEqual(true);
      expect(map("two")).toEqual(true);
      expect(map("three")).toEqual(false);
      expect(map("four")).toEqual(true);
    });

    it("should be able to check with lowercase", () => {
      const str = "one,two,four";
      const map = makeMap(str, true);
      expect(map("One")).toEqual(true);
      expect(map("Two")).toEqual(true);
      expect(map("Three")).toEqual(false);
      expect(map("Four")).toEqual(true);
    });
  });

  describe("utils.isHTMLTag", () => {
    HTML_ELEMENTS.forEach((i) => {
      it(`should return true for ${i} element`, () => {
        expect(utils.isHTMLTag(i)).toEqual(true);
      });
    });

    it("should return false for non html elements", () => {
      expect(utils.isHTMLTag("my-tag")).toBe(false);
    });
  });

  describe("utils.isSVG", () => {
    SVG_ELEMENTS.forEach((i) => {
      it(`should return true for ${i} element`, () => {
        expect(utils.isSVG(i)).toEqual(true);
      });
    });

    HTML_ELEMENTS.forEach((i) => {
      it(`should return false for ${i} element`, () => {
        expect(utils.isSVG(i)).toEqual(false);
      });
    });

    it("should return false for non svg tag", () => {
      expect(utils.isSVG("my-tag")).toBe(false);
    });
  });

  describe("utils.isReservedTag", () => {
    [...HTML_ELEMENTS, ...SVG_ELEMENTS].forEach((i) => {
      it(`should return true for ${i} element`, () => {
        expect(utils.isReservedTag(i)).toEqual(true);
      });
    });

    it("should return false for non reserved tag", () => {
      expect(utils.isReservedTag("my-tag")).toBe(false);
    });
  });
});
