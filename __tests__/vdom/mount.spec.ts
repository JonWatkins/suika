import { mount } from "../../src/vdom/mount";

describe("mount", () => {
  it("should be able to mount an app", () => {
    class App {
      _el: HTMLElement | null;
      _isMounted: Boolean;

      constructor() {
        this._el = null;
        this._isMounted = false;
      }

      _mount(el) {
        this._el = el;
        this._isMounted = true;
      }
    }

    const el = document.createElement("div");
    const instance = mount(App, el);

    // @ts-ignore:next-line
    expect(instance._el).toEqual(el);

    // @ts-ignore:next-line
    expect(instance._isMounted).toEqual(true);
  });
});
