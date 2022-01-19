# Yew Web Component Test

This is test at running a Yew frontend leveraging web components from the following sources:

- [Lit](https://lit.dev/)
- [Material](https://material-components.github.io/material-components-web-catalog/#/)

This project was generated with the [yew trunk minimal template](https://github.com/yewstack/yew-trunk-minimal-template) and is inspired by [patternfly-yew](https://github.com/ctron/patternfly-yew) from the blog post [rusty-frontend-patternfly-yew](https://dentrassi.de/2021/01/08/rusty-frontend-patternfly-yew).

## Usage

For a more thorough explanation of Trunk and its features, please head over to the [repository][trunk].

### Installation

If you don't already have it installed, it's time to install Rust: <https://www.rust-lang.org/tools/install>.
The rest of this guide assumes a typical Rust installation which contains both `rustup` and Cargo.

To compile Rust to WASM, we need to have the `wasm32-unknown-unknown` target installed.
If you don't already have it, install it with the following command:

```bash
rustup target add wasm32-unknown-unknown
```

Now that we have our basics covered, it's time to install the star of the show: [Trunk].
Simply run the following command to install it:

```bash
cargo install trunk wasm-bindgen-cli
```

That's it, we're done!

### Running

```bash
trunk serve
```

Rebuilds the app whenever a change is detected and runs a local server to host it.

There's also the `trunk watch` command which does the same thing but without hosting it.

### Release

```bash
trunk build --release
```

This builds the app in release mode similar to `cargo build --release`.
You can also pass the `--release` flag to `trunk serve` if you need to get every last drop of performance.

Unless overwritten, the output will be located in the `dist` directory.

[trunk]: https://github.com/thedodd/trunk
