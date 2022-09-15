const http = require("http");

const server = http.createServer((req, res) => {
  if (req.method === "GET" && req.url === "/") {
    res.writeHead(200, {
      "Content-Type": "text/html",
    });
    res.end(`\
<script>
const controller = new AbortController();
fetch("/mypath", {
  signal: controller.signal,
});
setTimeout(() => {
  controller.abort();
}, 3000);
</script>
`);
    return;
  }
  console.log("requested");
  req.on("close", () => {
    console.log("on close");
  });

  req.on("end", () => {
    console.log("on end");
  });
});
server.listen(3000);
