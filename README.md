# calc.rs

This is a small project of me, mainly to learn Rust, but also to have a basic but working commandline calculator on Windows, as we don't have bc to my knowledge. (I'll switch to Arch soon anyways, oh well)

## Usage

calc.rs supports really basic arithmetic operations, such as addition, subtraction, multiplication and division.  
Exponentiation and square rooting(?) is also supported.

It's important to note that as of right now, all arguments need to be sperated by spaces. Furthermore, only two numbers are supported. Additional statements are truncated.

Addition:

```
calc.rs $> 2 + 2
4
```

Subtraction:

```
calc.rs $> 31 - 2
29
```

Multiplication:

```
calc.rs $> 2 * 2
4
```

Division:

```
calc.rs $> 5.5 / 2
2.75
```

Exponentiation:

```
calc.rs $> 2 ^ 2
4
```

Square rooting:

```
calc.rs $> 4 # <any number, doesn't matter>
2
```

## Building

calc.rs can be build as any other Rust project.

```
$ git clone https://github.com/miiiiiYT/calc.rs.git
$ cargo build
resulting binary will be in ./target/debug/
```

## Installation

Install calc.rs by cloning the repo and running `cargo install`.

```
$ git clone https://github.com/miiiiiYT/calc.rs.git
$ cargo install --path=.
```

After this (and, depending on your system, restarting your shell), run `calc-rs`

```
$ calc-rs
Welcome to calc.rs
calc.rs $>
```

## Flags

I have implemented a really important feature.  
Run the program with either the `--serious` or `--silly` flag for some changed messages. `--serious` is applied by default.  
This is mostly intended for aliases.

You can also specify the `--suppress-notice` flag or it's shortform `-n` to disable the copyright notice at the beginning.
