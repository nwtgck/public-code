const pako = require("pako");

// English
{
  const plainText         = "hello, world"
  const compressedBinText = pako.deflate(plainText, {to: 'string', level: 9, windowBits: -8});
  const decompressedText  = pako.inflate(compressedBinText, {to: 'string', windowBits: -8});
  console.log(plainText);
  console.log(decompressedText);
}


// Japanese
{
  const plainText         = "これは日本語です。"
  const compressedBinText = pako.deflate(plainText, {to: 'string', level: 9, windowBits: -8});
  const decompressedText  = pako.inflate(compressedBinText, {to: 'string', windowBits: -8});
  console.log(plainText);
  console.log(decompressedText);
}
