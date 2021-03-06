This is a Heroku buildpack for [Neon] with [cargo], [rustup], [node], and [npm],
as well as build requirements. Features:

- Saves and restores npm and cargo caches.
- Uses the latest stable Rust and Node, or as specified.
- Defaults to running `npm start` as the web process.

[cargo]: http://crates.io/
[rustup]: https://www.rustup.rs/
[npm]: https://www.npmjs.com/
[Neon]: https://www.neon-bindings.com/
[node]: https://nodejs.org/

## Specifying which versions to use

By default, this buildpack installs:

- Rust stable
- Node LTS
- npm latest

However, you can specify exact versions or channels via the `engines` field of
the package.json. You can specify exact versions, ranges (only for node and
npm), or channel names.

Rust is installed with [rustup], Node with [nave], npm installs itself.

```
"engines": {
  "node": "^8.0.0",
  "npm": "^5.0.3",
  "rust": "nightly"
}
```

## Usage

For an example, see the test/neon-test application, which is also used to drive
our CI tests.

This buildpack expects a Node application with a package.json and a
native/Cargo.toml. It will run `npm install` in the root of the app, then set a
default Procfile if none exist to run `npm start`. Neon builds should be hooked
as normal through the `scripts.install` package.json key.

### For non-Neon apps

It's also possible to use this buildpack to run normal non-Neon node apps. Why
use this instead of the official Node buildpack? Perhaps a Neon component is
possible in the future. Perhaps you like the caching and version behaviour of
this buildpack. In any case, simply set `engines.neon` to `false` to disable
both `native/Cargo.toml` detection and the Rust install. Everything else
behaves the same.
