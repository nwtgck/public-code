const fs = require("fs");
const http2 = require("http2");

const server = http2.createSecureServer({
  key: fs.readFileSync("./server.key"),
  cert: fs.readFileSync("./server.crt"),
});

server.on("stream", (stream, headers) => {
  console.log("on stream");
  stream.respond({
    ":status": 200,
  });
  stream.pipe(fs.createWriteStream("/dev/null"));
})

server.listen(8443, () => {
  console.log("HTTPS (HTTP/2) listening on 8443 port");
});
