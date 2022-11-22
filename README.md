# Auto Const Array

Use this macro to declare a const array without specifing its length.

[![Crates.io][crates-badge]][crates-url]
[![MIT/Apache-2 licensed][license-badge]][license-url]

[crates-badge]: https://img.shields.io/crates/v/auto_const_array.svg
[crates-url]: https://crates.io/crates/auto_const_array
[license-badge]: https://img.shields.io/crates/l/auto_const_array.svg
[license-url]: LICENSE-MIT

```rust
use auto_const_array::auto_const_array;
auto_const_array! {
    // Additional attributes and docs are supported.
    /// Common array with public visibility.
    #[allow(unused)]
    pub const ARRAY_COMMON: [u8; _] = [1, 2, 4];
    /// Special array with cfg conditional compling.
    const ARRAY_WITH_ATTR: [u8; _] = [1, #[cfg(unix)] 2]
}
```