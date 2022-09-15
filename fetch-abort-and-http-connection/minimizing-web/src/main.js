console.log("GET ...");
const controller = new AbortController();
fetch("http://localhost:3000/mypath", {
  signal: controller.signal,
});
setTimeout(() => {
  controller.abort();
  console.log("aborted!");
}, 3000);
