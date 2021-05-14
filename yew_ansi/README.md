# Yew ANSI

[![Crate Info](https://img.shields.io/crates/v/yew-ansi.svg)][crate-info]
[![API Documentation](https://docs.rs/yew-ansi/badge.svg)][api-documentation]

ANSI escape code rendering for the web.

Currently this library only covers SGR parameters (i.e. the character appearance part).

Supported SGR parameters:

- bold
- italic
- underline
- foreground and background colours:
  - 3/4 bit (named colours with "bright" modifier)
  - 8-bit (256-color palette)
  - 24-bit (full RGB)

## Features

- "yew" (default feature) - Activate the Yew components. Without this feature this crate is just an ANSI escape code parser.

## Examples

See the [examples](examples) directory.

[crate-info]: https://crates.io/crates/yew-ansi
[api-documentation]: https://docs.rs/yew-ansi
