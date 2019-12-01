# Simple Synth

A simple web-based synth built with Rust / WebAssembly / WebAudio API.

## Dependencies

Make sure you have [wasm-pack](https://rustwasm.github.io/wasm-pack/) installed:

```
curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
```

## Usage

 - Build the wasm binary `wasm-pack build`
 - Install npm dependencies `cd www && npm install`
 - Run the demo `npm start`
 - Go to http://localhost:8080

## Resources

* [`wasm-bindgen`](https://github.com/rustwasm/wasm-bindgen) for communicating
  between WebAssembly and JavaScript.
* [`console_error_panic_hook`](https://github.com/rustwasm/console_error_panic_hook)
  for logging panic messages to the developer console.
* [`wee_alloc`](https://github.com/rustwasm/wee_alloc), an allocator optimized
  for small code size.
