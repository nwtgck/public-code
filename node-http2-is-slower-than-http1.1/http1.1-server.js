const fs = require("fs");
const https = require("https");

const server = https.createServer({
  key: fs.readFileSync("./server.key"),
  cert: fs.readFileSync("./server.crt"),
}, (req, res) => {
  console.log("request accepted");
  res.writeHead(200);
  req.pipe(fs.createWriteStream("/dev/null"));
});

server.listen(8443, () => {
  console.log("HTTPS (HTTP/1.1) listening on 8443 port");
});
