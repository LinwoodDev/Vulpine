# Linwood Vulpine

> A user friendly gui for command line tools

## Features

* Create apps with a simple wizard
* Run apps with a single click
* Add input fields to apps
* Share apps with others
* Cross platform on windows, linux and macos

## Getting started

### Installation

Install rust and pnpm first.
Then install the dependencies of tauri. Click [here](https://beta.tauri.app/guides/prerequisites/) for more information.

```bash
rustup target add wasm32-unknown-unknown
cargo install tauri-cli --version "^2.0.0-beta" --locked
cargo install trunk
pnpm install
```

### Running in development

```bash
cargo tauri dev
```

### Building for production

```bash
cargo tauri build
```
