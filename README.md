<div align="center">

# Inc Dec

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

## Examples - Extension Traits:

The pp and mm methods:

```rust

    use inc_dec::IncDecSelf;

    let mut u32_val: u32 = 0;

    assert_eq!(1, u32_val.pp());

    assert_eq!(0, u32_val.mm());

```

The try_pp and try_mm methods:

```rust

    use inc_dec::IncDecSelf;

    let mut u32_val: u32 = 0;

    assert_eq!(Some(1), u32_val.try_pp());

    assert_eq!(Some(0), u32_val.try_mm());

    assert_eq!(None, u32_val.try_mm());

```

## Examples - Macros:

The pp macro:

```rust

    use inc_dec::pp;

    let mut int_val = 1;

    pp!(int_val);

    assert_eq!(2, int_val);

```

The ppf macro:

```rust

    use inc_dec::ppf;

    let mut f32_val: f32 = 1.0;

    ppf!(f32_val);

    assert_eq!(2.0, f32_val);

    let mut f64_val = 1.0;

    ppf!(f64_val);

    assert_eq!(2.0, f64_val);

```

The mm macro:

```rust

    use inc_dec::mm;

    let mut int_val = 2;

    mm!(int_val);

    assert_eq!(1, int_val);

```

The mmf macro:

```rust

    use inc_dec::mmf;

    let mut f32_val: f32 = 2.0;

    mmf!(f32_val);

    assert_eq!(1.0, f32_val);

    let mut f64_val = 2.0;

    mmf!(f64_val);

    assert_eq!(1.0, f64_val);

```

</br>

Aside from regular incrementation and decrementation, the following core library integer methods are used (With associated trait method names) in the integer implementations of the IncDecSelf and IntIncDecSelf traits:

| Method | IncDecSelf Method |
| ------ | ----------- |
| checked_add | try_pp |
| checked_sub | try_mm |

| Method | IntIncDecSelf Method |
| ------ | ----------- |
| overflowing_add | opp |
| overflowing_sub | omm |
| wrapping_add | wpp |
| wrapping_sub | wmm |

## No-Std

You don't need it.

## Todo:

- Add more documentation
- Add more code examples
- Add more tests
- Clean-up the code
- Add support for non-zero integers (core::num).

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

