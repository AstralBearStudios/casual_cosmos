# Casual Cosmos

[!WARNING]
This project is in early development. Many features mentioned here
have **not** been developed (yet). Also, expect breaking changes!
**A Contributions guide and the Code of Conduct is a WIP, so please wait to submit
pull requests.**

Welcome to Casual Cosmos, a game engine dedicated to custom casual games!

## Frontend

The core config format is written and has four main features:

- **Human-readable**, written in [TOML](https://toml.io/en/) with simple
  key and value names. (Currently, the text encoding is UTF-8).
- **Language agnostic**, so this engine can be implemented with
  nearly any programming language! (Boardgames are also possible!)
- **Extremely customizable**, from levels to UI design!
- **Backwards compatible**, with newer versions **only** adding new options!

These core features mean several things:

- **Easily portable!**
- **Easy to customize!** Use the builtin editor
  or edit the file directly!
- **Easy to maintain!** No need to worry about old
  or proprietary formats!

## Backend

## License

The **frontend language** is licensed under:

- CC0 (LICENSE-CC0 or [https://creativecommons.org/publicdomain/zero/1.0/legalcode.txt](https://creativecommons.org/publicdomain/zero/1.0/legalcode.txt)).

The **backend (engine)** is dual licensed. Except where noted, you may choose either:

- MIT License ([LICENSE-MIT](LICENSE-MIT) or [https://opensource.org/licenses/MIT](https://opensource.org/licenses/MIT))
- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or [https://www.apache.org/licenses/LICENSE-2.0](https://www.apache.org/licenses/LICENSE-2.0))

(The wording above is adapted from the [Bevy engine](https://github.com/bevyengine),
a core dependency in this project.)
