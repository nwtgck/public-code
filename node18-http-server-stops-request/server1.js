const http = require("http");

const server = http.createServer({ requestTimeout: 0 }, (req, res) => {
  console.log(`${req.method} ${req.url}`);
  req.on("data", (data) => console.log(`on data: ${data.length}B`));
  req.on("close", () => console.log("req closed"));
  req.on("aborted", () => console.log("req aborted"));
  req.on("error", (err) => console.log("req error", err));
  req.on("end", () => {
    console.log("req end");
    res.end("Finished!\n");
  });
  res.writeHead(200);
  res.write("Handling...\n");
});

server.listen(3000, () => {
  console.log(`Listening on ${server.address().port}...`);
});
