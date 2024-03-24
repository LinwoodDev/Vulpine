<div align="center">

<img src="https://raw.githubusercontent.com/LinwoodDev/Vulpine/main/public/logo.png" width="350px" />

# Vulpine

> WIP: 🖼️ A user friendly gui for command line tools 🖼️

[![Latest release)](https://img.shields.io/github/v/release/LinwoodDev/Vulpine?color=FBAC11&style=for-the-badge&logo=github&logoColor=FBAC11)](https://github.com/LinwoodDev/Vulpine/releases)
[![GitHub License badge](https://img.shields.io/github/license/LinwoodDev/Vulpine?color=FBAC11&style=for-the-badge&logo=data:image/svg+xml;base64,PHN2ZyB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciIHdpZHRoPSIxOTIiIGhlaWdodD0iMTkyIiBmaWxsPSIjZWJiNzMzIiB2aWV3Qm94PSIwIDAgMjU2IDI1NiI%2BPHJlY3Qgd2lkdGg9IjI1NiIgaGVpZ2h0PSIyNTYiIGZpbGw9Im5vbmUiPjwvcmVjdD48cmVjdCB4PSIzMiIgeT0iNDgiIHdpZHRoPSIxOTIiIGhlaWdodD0iMTYwIiByeD0iOCIgc3Ryb2tlLXdpZHRoPSIxNiIgc3Ryb2tlPSIjZWJiNzMzIiBzdHJva2UtbGluZWNhcD0icm91bmQiIHN0cm9rZS1saW5lam9pbj0icm91bmQiIGZpbGw9Im5vbmUiPjwvcmVjdD48bGluZSB4MT0iNzYiIHkxPSI5NiIgeDI9IjE4MCIgeTI9Ijk2IiBmaWxsPSJub25lIiBzdHJva2U9IiNlYmI3MzMiIHN0cm9rZS1saW5lY2FwPSJyb3VuZCIgc3Ryb2tlLWxpbmVqb2luPSJyb3VuZCIgc3Ryb2tlLXdpZHRoPSIxNiI%2BPC9saW5lPjxsaW5lIHgxPSI3NiIgeTE9IjEyOCIgeDI9IjE4MCIgeTI9IjEyOCIgZmlsbD0ibm9uZSIgc3Ryb2tlPSIjZWJiNzMzIiBzdHJva2UtbGluZWNhcD0icm91bmQiIHN0cm9rZS1saW5lam9pbj0icm91bmQiIHN0cm9rZS13aWR0aD0iMTYiPjwvbGluZT48bGluZSB4MT0iNzYiIHkxPSIxNjAiIHgyPSIxODAiIHkyPSIxNjAiIGZpbGw9Im5vbmUiIHN0cm9rZT0iI2ViYjczMyIgc3Ryb2tlLWxpbmVjYXA9InJvdW5kIiBzdHJva2UtbGluZWpvaW49InJvdW5kIiBzdHJva2Utd2lkdGg9IjE2Ij48L2xpbmU%2BPC9zdmc%2B)](https://github.com/LinwoodDev/Vulpine/blob/main/LICENSE)
[![GitHub Repo stars](https://img.shields.io/github/stars/LinwoodDev/Vulpine?color=FBAC11&logo=github&logoColor=FBAC11&style=for-the-badge)](https://github.com/LinwoodDev/Vulpine)
[![Matrix badge](https://img.shields.io/matrix/linwood:matrix.org?style=for-the-badge&color=FBAC11&logo=matrix&logoColor=FBAC11&label=Matrix)](https://linwood.dev/matrix)
[![Discord badge](https://img.shields.io/discord/735424757142519848?style=for-the-badge&color=FBAC11&logo=discord&logoColor=FBAC11&label=Discord)](https://discord.linwood.dev)
[![Download](https://img.shields.io/github/downloads/LinwoodDev/Vulpine/total?color=FBAC11&style=for-the-badge&logo=data:image/svg+xml;base64,PHN2ZyB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciIHdpZHRoPSIxOTIiIGhlaWdodD0iMTkyIiBmaWxsPSIjZWJiNzMzIiB2aWV3Qm94PSIwIDAgMjU2IDI1NiI+PHJlY3Qgd2lkdGg9IjI1NiIgaGVpZ2h0PSIyNTYiIGZpbGw9Im5vbmUiPjwvcmVjdD48cG9seWxpbmUgcG9pbnRzPSI4NiAxMTAuMDExIDEyOCAxNTIgMTcwIDExMC4wMTEiIGZpbGw9Im5vbmUiIHN0cm9rZT0iI2ViYjczMyIgc3Ryb2tlLWxpbmVjYXA9InJvdW5kIiBzdHJva2UtbGluZWpvaW49InJvdW5kIiBzdHJva2Utd2lkdGg9IjE2Ij48L3BvbHlsaW5lPjxsaW5lIHgxPSIxMjgiIHkxPSI0MCIgeDI9IjEyOCIgeTI9IjE1MS45NzA1NyIgZmlsbD0ibm9uZSIgc3Ryb2tlPSIjZWJiNzMzIiBzdHJva2UtbGluZWNhcD0icm91bmQiIHN0cm9rZS1saW5lam9pbj0icm91bmQiIHN0cm9rZS13aWR0aD0iMTYiPjwvbGluZT48cGF0aCBkPSJNMjE2LDE1MnY1NmE4LDgsMCwwLDEtOCw4SDQ4YTgsOCwwLDAsMS04LThWMTUyIiBmaWxsPSJub25lIiBzdHJva2U9IiNlYmI3MzMiIHN0cm9rZS1saW5lY2FwPSJyb3VuZCIgc3Ryb2tlLWxpbmVqb2luPSJyb3VuZCIgc3Ryb2tlLXdpZHRoPSIxNiI+PC9wYXRoPjwvc3ZnPg==)](https://vulpine.linwood.dev)
</div>

<p align="center">
    <a href="http://linwood.dev"><b>Website</b></a> •
    <a href="http://linwood.dev"><b>Download</b></a> •
    <a href="https://linwood.dev/matrix"><b>Matrix</b></a> •
    <a href="https://go.linwood.dev/discord"><b>Discord</b></a> •
    <a href="https://twitter.com/LinwoodDev"><b>Twitter</b></a> •
    <a href="CONTRIBUTING.md"><b>Contribute</b></a>
</p>  

---

**This project is still in development and not ready for production use. Please join the matrix or discord server for updates!**

Vulpine helps you to visualize and interact with command line tools. It is a simple and user cross-platform, opensource app built with tauri and rust.

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
rustup toolchain install nightly
rustup default nightly
rustup target add wasm32-unknown-unknown
cargo install tauri-cli --version "^2.0.0-beta" --locked
cargo install trunk
pnpm install
```

### Running in development

To have faster compile times, you can set the number of threads to use in the RUSTFLAGS environment variable.

Windows:

```bash
$env:RUSTFLAGS="-Z threads=8"
cargo tauri dev
```

Linux:

```bash
RUSTFLAGS="-Z threads=8" cargo tauri dev
```

### Building for production

```bash
cargo tauri build
```
