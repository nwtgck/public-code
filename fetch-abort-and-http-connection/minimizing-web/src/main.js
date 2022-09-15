(async () => {
  console.log("AbortController", AbortController);
  const controller = new AbortController();
  fetch("http://localhost:8181/m3", {
    signal: controller.signal,
  });
  await new Promise(resolve => setTimeout(resolve, 3000));
  controller.abort();
})();
