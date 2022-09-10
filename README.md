# sha2-wasm

A simple library to calculate sha256 and sha512 hashes from `string` built with Rust+WASM

## Installation

```bash
npm i sha2-wasm
```

## Usage

```javascript

const sha2lib = require('sha2-wasm')

const sha256 = sha2lib.sha256("hello");
const sha512 = sha2lib.sha512("hello");
```
