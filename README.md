<div align="center">

# IncDec

[![Crates.io](https://img.shields.io/crates/v/inc_dec)](https://crates.io/crates/inc_dec)
[![License](https://img.shields.io/badge/license-MIT%2FApache-blue)](#license)
[![Downloads](https://img.shields.io/crates/d/inc_dec)](https://crates.io/crates/inc_dec)
[![Docs](https://docs.rs/inc_dec/badge.svg)](https://docs.rs/inc_dec/latest/inc_dec/)
[![Twitch Status](https://img.shields.io/twitch/status/coruscateor)](https://www.twitch.tv/coruscateor)

[X](https://twitter.com/Coruscateor) | 
[Twitch](https://www.twitch.tv/coruscateor) | 
[Youtube](https://www.youtube.com/@coruscateor) | 
[Mastodon](https://mastodon.social/@Coruscateor) | 
[GitHub](https://github.com/coruscateor) | 
[GitHub Sponsors](https://github.com/sponsors/coruscateor)

Incrementation and decrementation in Rust.

</div>

<br/>

<br/>

IncDec provides macros and extension traits which make doing incrementation and decrementation of numeric values a bit easier.

<br/>

## Examples

The pp and mm extension methods:

```rust

    use inc_dec::IncDecExt;

    let mut val: u32 = 0;

    assert_eq!(1, val.pp());

    assert_eq!(0, val.mm());

```

The try_pp and try_mm extension methods:

```rust

    use inc_dec::IncDecExt;

    let mut val: u32 = 0;

    assert_eq!(Some(1), val.try_pp());

    assert_eq!(Some(0), val.try_mm());

    assert_eq!(None, val.try_mm());

```

The opp and omm extension methods:

```rust

    use inc_dec::IntIncDecExt;

    let mut val: u32 = 0;

    assert_eq!((1, false), val.opp());

    assert_eq!((0, false), val.omm());

    assert_eq!((u32::MAX, true), val.omm());

```

The wpp and wmm extension methods:

```rust

    use inc_dec::IntIncDecExt;

    let mut val: u32 = 0;

    assert_eq!(1, val.wpp());

    assert_eq!(0, val.wmm());

    assert_eq!(u32::MAX, val.wmm());

```

</br>

The below tables indicate which trait implementation methods use which core integer methods (where applicable):

| IncDecExt Method | Method Used |
| ---------------- | ----------- |
| try_pp           | checked_add |
| try_mm           | checked_sub |

| IntIncDecExt Method | Method          |
| ------------------- | --------------- |
| opp                 | overflowing_add |
| omm                 | overflowing_sub |
| wpp                 | wrapping_add    |
| wmm                 | wrapping_sub    |

</br>

## Features

| Feature     | Description                                |
| ----------- | -------------------------------------------|
| num         | Enable core::num NonZero extension methods. |

</br>

## Todo

- Add more documentation.
- Add more code examples.
- Add more tests.
- Clean-up the code.
- Support saturating incrementation and decrementation.

## Maybe

- Add support for numeric types in other crates.

</br>

## Coding Style

This project uses a coding style that emphasises the use of white space over keeping the line and column counts as low as possible.

So this:

```rust

# fn bar() {} 

fn foo()
{

    bar();

}

```

Not this:

```rust

# fn bar() {} 

fn foo()
{
    bar();
}

```

<br/>

## License

Licensed under either of:

- Apache License, Version 2.0, ([LICENSE-APACHE](./LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0 (see also: https://www.tldrlegal.com/license/apache-license-2-0-apache-2-0))
- MIT license ([LICENSE-MIT](./LICENSE-MIT) or http://opensource.org/licenses/MIT (see also: https://www.tldrlegal.com/license/mit-license))

at your discretion

<br/>

## Contributing

Please clone the repository and create an issue explaining what feature or features you'd like to add or bug or bugs you'd like to fix and perhaps how you intend to implement these additions or fixes. Try to include details though it doesn't need to be exhaustive and we'll take it from there (dependant on availability).

<br/>

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.

