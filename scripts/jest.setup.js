global.requestIdleCallback = (callback) => {
  const start = Date.now();
  return setTimeout(() => {
    callback({
      didTimeout: false,
      timeRemaining: () => Math.max(0, 50 - (Date.now() - start)),
    });
  }, 1);
};

global.cancelIdleCallback = (id) => {
  clearTimeout(id);
};
