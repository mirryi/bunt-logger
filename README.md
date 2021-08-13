[![Build status](https://github.com/Dophin2009/bunt-logger/workflows/ci/badge.svg)](https://github.com/Dophin2009/bunt-logger/actions)
[![Crates.io](https://img.shields.io/crates/v/bunt-logger.svg)](https://crates.io/crates/bunt-logger)
[![Docs.rs](https://docs.rs/bunt-logger/badge.svg)](https://docs.rs/bunt-logger)

# bunt-logger

`bunt-logger` is a convenience wrapper around [`bunt`](https://github.com/LukasKalbertodt/bunt), a
library for printing colored and formatted text to terminals. It provides logging macros resembling
those of [`log`](https://github.com/rust-lang/log) and a configuration interface similar to
[`stderrlog`](https://github.com/cardoe/stderrlog-rs).

```rust
use bunt_logger::{error, debug, ColorChoice, Level};

bunt_logger::with()
    .level(Level::Debug)
    .stderr(ColorChoice::Always);

let data = vec![0, 2, 4];
debug!("Current value: {[cyan]:?}", data);

let err = ...
error!("{$red}Oh no! Error: {[bold]}{/$}", err);
```

See the [documentation](https://docs.rs/bunt-logger).

## License

Licensed under either of [Apache License, Version 2.0](./LICENSE-APACHE) or [MIT](./LICENSE-MIT)
license at your option. Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in this project by you, as defined in the Apache-2.0 license, shall be dual
licensed as above, without any additional terms or conditions.