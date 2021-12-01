# MD5 Web Assembly / Rust Example

This project provides an example of generating an MD5 hash from a byte array (`Uint8Array`)
passed in from JavaScript and using Rust/Wasm to process it.

1. Install `Rust` and `Cargo` in order to build the project.
2. Install `wasm-pack`: `cargo install wasm-pack`.
3. `wasm-pack build --target web --release`
4. Run tests: `wasm-pack test --node`
5. Pack: `wasm-pack pack`

You will need to run a local webserver that properly handles the `application/wasm` mimetype.
Personally, I use the PHP development server: `php -S localhost:8080`, which works fine. You
could also run this with the Python 3 development server: `python3 -m http.server`, but if you
are on Windows you might need to set the ContentType to `application/js` in the registry as
described here: https://bugs.python.org/issue43975

# TODO
- Use WebWorkers.