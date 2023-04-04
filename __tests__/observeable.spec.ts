import { observable } from "../src/observable";

type Ob2 = { b?: number | Array<any> };
type Ob1 = { a: Ob2 };

describe("observable", () => {
  it("should not make an observable if the target is not an object", () => {
    const ob = observable(1);
    expect(typeof ob).toBe("number");
  });

  it("should be able to make an observable", () => {
    const ob = observable({});
    expect(typeof ob).toBe("object");
  });

  it("should be able to make nested observable", () => {
    const ob = observable({ a: { b: 1 } });
    expect(typeof ob).toBe("object");
  });

  it("should allow property access", () => {
    const ob: Ob1 = observable({ a: { b: 1 } });
    expect(ob.a.b).toBe(1);
  });

  it("should allow property write", () => {
    const ob: Ob1 = observable({ a: { b: 1 } });
    expect(ob.a.b).toBe(1);
    ob.a.b = 2;
    expect(ob.a.b).toBe(2);
  });

  it("should trigger the listener on write", (done) => {
    const ob: Ob1 = observable({ a: { b: 1 } }, (e) => {
      expect(e.value).toBe(2);
      expect(e.name).toBe("b");
      expect(e.target[e.name]).toBe(2);
      expect(e.path).toBe("a.b");
      done();
    });

    expect(ob.a.b).toBe(1);
    ob.a.b = 2;
  });

  it("should trigger the listener on delete", (done) => {
    const ob: Ob1 = observable({ a: { b: 1 } }, (e) => {
      expect(e.name).toBe("b");
      expect(e.target[e.name]).toBe(undefined);
      expect(e.path).toBe("a.b");
      done();
    });

    expect(ob.a.b).toBe(1);

    delete ob.a.b;
  });

  it("should not trigger listener on delete if not defined", () => {
    const ob: Ob1 = observable({ a: { b: 1 } });

    expect(ob.a.b).toBe(1);

    delete ob.a.b;

    expect(ob.a.b).toBe(undefined);
  });

  it("should call the listener when an array changes", () => {
    const spy = jest.fn();

    type Ob4 = { b: Array<any> };
    type Ob3 = { a: Ob4 };

    const ob: Ob3 = observable({ a: { b: [1] } }, (e) => {
      spy();
    });

    expect(ob.a.b[0]).toBe(1);

    expect(ob.a.b.length).toBe(1);

    ob.a.b.push(2);

    expect(ob.a.b.length).toBe(2);

    ob.a.b.splice(0, 1);

    expect(ob.a.b.length).toBe(1);
    expect(spy).toBeCalledTimes(5);
  });
});
