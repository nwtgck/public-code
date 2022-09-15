const http = require("http");

const server = http.createServer((req, res) => {
  console.log(`${req.method} ${req.url} requested`);
  req.on("close", () => {
    console.log(`${req.url} closed`);
  });
});
server.listen(3000, (x) => {
  console.log(`listening on ${server.address().port}`);
});
