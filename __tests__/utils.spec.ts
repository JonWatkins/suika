import * as utils from "../src/utils";

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

  describe("utils.makeMap", () => {
    it("should be able to make a map object from a string", () => {
      const str = "one,two,four";
      const map = utils.makeMap(str);
      expect(map("one")).toEqual(true);
      expect(map("two")).toEqual(true);
      expect(map("three")).toEqual(false);
      expect(map("four")).toEqual(true);
    });

    it("should be able to check with lowercase", () => {
      const str = "one,two,four";
      const map = utils.makeMap(str, true);
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

  describe("isEqual", () => {
    it("should be able to compare 2 objects", () => {
      expect(utils.isEqual({}, {})).toBe(true);
      expect(utils.isEqual({}, { test: 1 })).toBe(false);
      expect(utils.isEqual({ test: 2 }, { test: 1 })).toBe(false);
      expect(utils.isEqual({ test: 1 }, { test: 1 })).toBe(true);
    });
  });

  describe("mergeClassNames", () => {
    it("should have a function to merge class names", () => {
      expect(utils.mergeClassNames("old", "new")).toBe("old new");
    });
  });

  describe("mergeClassNames", () => {
    it("should have a function to fix element options", () => {
      const opts = utils.fixOptions({ a: 1 }, { b: 2 });
      expect(opts.a).toBe(1);
      expect(opts.b).toBe(2);
    });
  });

  describe("fixOptions", () => {
    it("should be able to merge options", () => {
      const a = { test: true };
      const b = { env: "dev" };
      const res = utils.fixOptions(a, b);
      expect(res.test).toBe(true);
      expect(res.env).toBe("dev");
    });

    it("should be able to ommit the second param", () => {
      const a = { test: true };
      const res = utils.fixOptions(a);
      expect(res.test).toBe(true);
    });

    it('should skip the "is" key', () => {
      const a = { is: true };
      const res = utils.fixOptions(a);
      expect(res.test).toBe(undefined);
    });

    it("should be able to merge the className properties", () => {
      const a = { className: "btn" };
      const b = { className: "btn-default" };
      const res = utils.fixOptions(a, b);
      expect(res.className).toBe("btn btn-default");
    });
  });
});
