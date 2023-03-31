import { observable } from "../../src/lib/observable";

type Ob2 = { b: number };
type Ob1 = { a?: Ob2 };

describe("observable", () => {
  it("should not make an observable if the target is not an object", () => {
    // @ts-ignore:next-line
    const ob = observable(1);
    expect(typeof ob).toBe("number");
  });

  it("should be able to make an observable", () => {
    // @ts-ignore:next-line
    const ob = observable({});
    expect(typeof ob).toBe("object");
  });

  it("should be able to make nested observable", () => {
    // @ts-ignore:next-line
    const ob = observable({ a: { b: 1 } });
    expect(typeof ob).toBe("object");
  });

  it("should allow property access", () => {
    // @ts-ignore:next-line
    const ob: Ob1 = observable({ a: { b: 1 } });
    // @ts-ignore:next-line
    expect(ob.a.b).toBe(1);
  });

  it("should allow property write", () => {
    // @ts-ignore:next-line
    const ob: Ob1 = observable({ a: { b: 1 } });
    // @ts-ignore:next-line
    expect(ob.a.b).toBe(1);
    // @ts-ignore:next-line
    ob.a.b = 2;
    // @ts-ignore:next-line
    expect(ob.a.b).toBe(2);
  });

  it("should trigger the listener on write", (done) => {
    // @ts-ignore:next-line
    const ob: Ob1 = observable({ a: { b: 1 } }, (e) => {
      expect(e.value).toBe(2);
      expect(e.name).toBe("b");
      expect(e.target[e.name]).toBe(2);
      expect(e.path).toBe("a.b");
      done();
    });

    // @ts-ignore:next-line
    expect(ob.a.b).toBe(1);
    // @ts-ignore:next-line
    ob.a.b = 2;
  });

  it("should trigger the listener on delete", (done) => {
    // @ts-ignore:next-line
    const ob: Ob1 = observable({ a: { b: 1 } }, (e) => {
      expect(e.name).toBe("b");
      expect(e.target[e.name]).toBe(undefined);
      expect(e.path).toBe("a.b");
      done();
    });

    // @ts-ignore:next-line
    expect(ob.a.b).toBe(1);

    // @ts-ignore:next-line
    delete ob.a.b;
  });

  it("should not trigger listener on delete if not defined", () => {
    // @ts-ignore:next-line
    const ob: Ob1 = observable({ a: { b: 1 } });

    // @ts-ignore:next-line
    expect(ob.a.b).toBe(1);

    // @ts-ignore:next-line
    delete ob.a.b;

    // @ts-ignore:next-line
    expect(ob.a.b).toBe(undefined);
  });

  it("should call the listener when an array changes", () => {
    const spy = jest.fn();

    // @ts-ignore:next-line
    const ob: Ob1 = observable({ a: { b: [1] } }, (e) => {
      spy();
    });

    // @ts-ignore:next-line
    expect(ob.a.b[0]).toBe(1);

    // @ts-ignore:next-line
    expect(ob.a.b.length).toBe(1);

    // @ts-ignore:next-line
    ob.a.b.push(2);

    // @ts-ignore:next-line
    expect(ob.a.b.length).toBe(2);

    // @ts-ignore:next-line
    ob.a.b.splice(0, 1);

    // @ts-ignore:next-line
    expect(ob.a.b.length).toBe(1);
    expect(spy).toBeCalledTimes(5);
  });
});
