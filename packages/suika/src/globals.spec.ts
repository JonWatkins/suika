import {
  setNextUnitOfWork,
  getNextUnitOfWork,
  setCurrentRoot,
  getCurrentRoot,
  setWipRoot,
  getWipRoot,
  setDeletions,
  getDeletions,
  setWipFiber,
  getWipFiber,
  setHookIndex,
  getHookIndex,
} from "./globals";

describe("Unit tests for utility functions", () => {
  test("setNextUnitOfWork and getNextUnitOfWork", () => {
    const mockUnitOfWork = { id: 1, type: "work" };
    setNextUnitOfWork(mockUnitOfWork);
    const result = getNextUnitOfWork();
    expect(result).toBe(mockUnitOfWork);
  });

  test("setCurrentRoot and getCurrentRoot", () => {
    const mockRoot = { id: 2, type: "root" };
    setCurrentRoot(mockRoot);
    const result = getCurrentRoot();
    expect(result).toBe(mockRoot);
  });

  test("setWipRoot and getWipRoot", () => {
    const mockWipRoot = { id: 3, type: "wipRoot" };
    setWipRoot(mockWipRoot);
    const result = getWipRoot();
    expect(result).toBe(mockWipRoot);
  });

  test("setDeletions and getDeletions", () => {
    const mockDeletions = [{ id: 4, type: "deletion" }];
    setDeletions(mockDeletions);
    const result = getDeletions();
    expect(result).toBe(mockDeletions);
  });

  test("setWipFiber and getWipFiber", () => {
    const mockWipFiber = { id: 5, type: "wipFiber" };
    setWipFiber(mockWipFiber);
    const result = getWipFiber();
    expect(result).toBe(mockWipFiber);
  });

  test("setHookIndex and getHookIndex", () => {
    const mockHookIndex = 6;
    setHookIndex(mockHookIndex);
    const result = getHookIndex();
    expect(result).toBe(mockHookIndex);
  });
});
