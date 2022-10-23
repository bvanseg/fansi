[discord-shield]: https://img.shields.io/discord/1031520677787865108
[discord-invite]: https://discord.com/invite/xyaKBzarJs

[actions-shield]: https://github.com/bvanseg/fansi/actions/workflows/rust.yml/badge.svg
[actions-yml]: https://github.com/bvanseg/fansi/actions/workflows/rust.yml

[license]: https://img.shields.io/github/license/bvanseg/fansi
[code-size]: https://img.shields.io/github/languages/code-size/bvanseg/fansi

[rust-lang]: https://www.rust-lang.org/
[rust-shield]: https://img.shields.io/badge/Made%20with-Rust-1f425f.svg

# Fansi

![GitHub][license]
[![rust-shield][]][rust-lang]
[![Rust][actions-shield]][actions-yml]
![GitHub code size in bytes][code-size]
[![discord-shield][]][discord-invite]

This crate allows developers to add color to their terminal output.

Here is an example to make text bold:

```rust
use fansi::{style::AnsiStyle, string::AnsiString};

// Create style.
let style = vec![AnsiStyle::Bold];
// Create text with style.
let text = AnsiString::with_styles_vec("world!", style);
// Print text.
println!("Hello, {}", text);
```

Here is another example to make the foreground text green:

```rust
use crate::{color::AnsiColor, style::AnsiStyle, string::AnsiString};

// Create style.
let style = vec![AnsiStyle::ForegroundColor(AnsiColor::Green)];
// Create text with style.
let text = AnsiString::with_styles_vec("world!", style);
// Print text.
println!("Hello, {}", text);
```

## Pre-Computing Style Strings

When writing code such as the above examples, it is important to note that using raw arrays/vectors of styles is sub-optimal due to the styles being converted to strings and joined when creating the `AnsiString`.

If performance is important for your application and the above method is not acceptable, an `AnsiStyleContainer` struct is provided which takes in styles and compiles them into a `String` internally.

The container can then be applied to any `String` and re-used over and over as shown below:

```rust
// Create styles.
let style = vec![AnsiStyle::ForegroundColor(AnsiColor::Green)];
// Create container.
let container = AnsiStyleContainer::from_vec(style);
// Apply container's compiled style string to text.
let text = container.apply("world!");
// Print text.
println!("Hello, {}", text);
```

## Windows Usage

For windows, you will need to do an extra step to enable ANSI support in your Powershell and Command Prompt terminals:

```rust
let result: Result<(), i32> = enable_ansi_support();
```
