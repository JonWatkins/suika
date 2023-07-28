// @ts-nocheck

export const TEXT_ELEMENT = "TEXT_ELEMENT";
export const PLACEMENT_TAG = "PLACEMENT";
export const UPDATE_TAG = "UPDATE";
export const DELETION_TAG = "DELETION";

let nextUnitOfWork = null;
let currentRoot = null;
let wipRoot = null;
let deletions = null;
let wipFiber = null;
let hookIndex = null;

export const setNextUnitOfWork = (newUnitOfWork) => {
  nextUnitOfWork = newUnitOfWork;
};

export const getNextUnitOfWork = () => {
  return nextUnitOfWork;
};

export const setCurrentRoot = (newCurrentRoot) => {
  currentRoot = newCurrentRoot;
};

export const getCurrentRoot = () => {
  return currentRoot;
};

export const setWipRoot = (newWipRoot) => {
  wipRoot = newWipRoot;
};

export const getWipRoot = () => {
  return wipRoot;
};

export const setDeletions = (newDeletions) => {
  deletions = newDeletions;
};

export const getDeletions = () => {
  return deletions;
};

export const setWipFiber = (newWipFiber) => {
  wipFiber = newWipFiber;
};

export const getWipFiber = () => {
  return wipFiber;
};

export const setHookIndex = (newHookIndex) => {
  hookIndex = newHookIndex;
};

export const getHookIndex = () => {
  return hookIndex;
};
