# nqar
NQAR is Not Quite A Rougelike, written in Rust

## Introduction

The purpose of this project is three-fold:
1. Learn Rust
2. Write a video game
3. Write an *amusing* video game

Since the standard of *amusing* is the author's, this is not intended to be a video game for everyone, but a video game that I actually would want to play, hence "Not Quite A Rougelike". This is not to say that certain features may (or may) not be added if requested by others, but that the primary driver will be the author's amusement.

## Development Environment
Presuming Windows-Subsystem for Linux and VS Code, [rustup](https://rustup.rs/) will install most dependencies although `cmake`, `pkg-config`, and `fontconfig` may still be needed:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
sudo install cmake
sudo apt install pkg-config
sudo apt install fontconfig libfontconfig-dev
sudo apt install libgl-dev libxrandr2 libxi6
cargo check
```

**NOTE** that restarting may be needed after installing the dependencies before `cargo check` will be successful.

## References
Wolverson, H (2019). *Roguelike Tutorial - In Rust*. https://bfnightly.bracketproductions.com/

Wolverson, H. (2021). *Hands-on Rust: Effective Learning through 2D Game Development and Play*. The Pragmatic Programmers.