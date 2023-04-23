import * as utils from "../src/utils";

describe("utils", () => {
  it("should have a function to get tag names", () => {
    expect(utils.getTagName({ is: "div" })).toBe("div");
  });
});
