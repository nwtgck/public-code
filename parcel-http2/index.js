const http2 = require("http2");

http2.createServer((req, res)=>{
  res.end("hello, world\n");
}).listen(3000, ()=>{
  console.log("Listening...");
});
