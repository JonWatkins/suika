import { render } from "suika";
import { Button } from "./button";

describe("button", () => {
  it("should be able to render a button", () => {
    const root = document.createElement("root");
    render(Button({}), root);
  });
});
