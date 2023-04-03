const http = require("http");

const server = http.createServer(async (req, res) => {
  console.log(`${req.method} ${req.url}`);
  req.on("close", () => console.log("req closed"));
  req.on("end", () => console.log("req end"));
  req.on("aborted", () => console.log("req aborted"));
  req.on("error", (err) => console.log("req error", err));
  res.writeHead(200);
  for (let i = 0; i < 15; i++) {
    res.write(`${new Date()}: Handing...\n`);
    await new Promise(resolve => setTimeout(resolve, 2000));
  }
  // await new Promise(resolve => setTimeout(resolve, 30 * 1000));
  res.end("Finished!\n");
});

server.listen(3000, () => {
  console.log(`Listening on ${server.address().port}...`);
});
