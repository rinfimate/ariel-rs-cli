# ariel-rs-cli

[![CI](https://github.com/rinfimate/ariel-rs-cli/actions/workflows/ci.yml/badge.svg)](https://github.com/rinfimate/ariel-rs-cli/actions/workflows/ci.yml)
[![crates.io](https://img.shields.io/crates/v/ariel-rs-cli.svg)](https://crates.io/crates/ariel-rs-cli)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)

`arielc` — a fast Mermaid diagram CLI. Renders `.mmd` files to SVG without Node.js, Puppeteer, or a browser. Compatible with [`mmdc`](https://github.com/mermaid-js/mermaid-cli) flags.

## Install

```sh
cargo install ariel-rs-cli
```

## Usage

```sh
arielc -i diagram.mmd -o output.svg
arielc -i diagram.mmd -o output.svg --theme dark
cat diagram.mmd | arielc -i - -o output.svg
arielc -i diagram.mmd -o -          # stdout
```

## Flags

| Flag | Description | Default |
|------|-------------|---------|
| `-i, --input <FILE>` | Input `.mmd` file (use `-` for stdin) | required |
| `-o, --output <FILE>` | Output `.svg` file (use `-` for stdout) | required |
| `-t, --theme <NAME>` | Theme: `default`, `dark`, `forest`, `neutral` | `default` |
| `-b, --background-color <COLOR>` | Background colour (e.g. `transparent`, `#ffffff`) | — |
| `--quiet` | Suppress progress output | false |
| `--version` | Print version and exit | — |

## mmdc compatibility

`arielc` is command-line compatible with `mmdc` for SVG output. To use it as a drop-in in scripts:

```sh
alias mmdc=arielc
```

Flags not applicable to arielc (no Puppeteer, no browser):
- `-e, --puppeteerConfigFile` — ignored
- `-p, --puppeteerConfig` — ignored
- `-w, --width` / `-H, --height` — SVG is resolution-independent (planned for PNG output)

## Supported diagram types

All 29 diagram types supported by [ariel-rs](https://crates.io/crates/ariel-rs).

## License

MIT © 2026 Rochanglien Infimate
