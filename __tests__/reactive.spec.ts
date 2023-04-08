import { Reactive } from "../src/Reactive";

describe("Reactive", () => {
  it("should be able to make a reactive object", () => {
    const reactive = new Reactive({ test: true });
    expect(reactive._isReactive).toBe(true);
  });

  it("should have an array of listeners", () => {
    const reactive = new Reactive({ test: true });
    expect(Array.isArray(reactive._listeners)).toBe(true);
  });

  it("should be able to add a new listener", () => {
    const listener = () => ({});
    const reactive = new Reactive({ test: true });
    expect(reactive._listeners.length).toBe(0);
    reactive.addListener(listener);
    expect(reactive._listeners.length).toBe(1);
  });

  it("should not add a listener if it is not a function", () => {
    const listener = 1223;
    const reactive = new Reactive({ test: true });
    expect(reactive._listeners.length).toBe(0);
    // @ts-ignore:next-line
    reactive.addListener(listener);
    expect(reactive._listeners.length).toBe(0);
  });

  it("should be able to remove an existing listener", () => {
    const listener = () => ({});
    const reactive = new Reactive({ test: true });
    reactive.addListener(listener);
    expect(reactive._listeners.length).toBe(1);
    reactive.removeListener(listener);
    expect(reactive._listeners.length).toBe(0);
  });

  it("should not try to remove a listner if not given a function", () => {
    const listener = () => ({});
    const reactive = new Reactive({ test: true });
    reactive.addListener(listener);
    expect(reactive._listeners.length).toBe(1);
    // @ts-ignore:next-line
    reactive.removeListener(1223);
    expect(reactive._listeners.length).toBe(1);
  });

  it("should be able to listen for changes", () => {
    const listener = jest.fn();
    const reactive = new Reactive({ test: true });
    reactive.addListener(listener);
    reactive.value.test = false;
    expect(listener).toBeCalledTimes(1);
  });

  it("can listen to changes on a nested prop", () => {
    const listener = jest.fn();
    const reactive = new Reactive({ foo: { bar: true } });
    reactive.addListener(listener);
    reactive.value.foo.bar = false;
    expect(listener).toBeCalledTimes(1);
  });

  it("can watch arrays", () => {
    const listener = jest.fn();

    const reactive = new Reactive({
      list: ["one", "two"],
    });

    reactive.addListener(listener);
    reactive.value.list.push("three");
    expect(reactive.value.list.length).toBe(3);
    expect(listener).toBeCalledTimes(1);
  });
});
