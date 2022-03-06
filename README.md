[![crates.io](https://img.shields.io/crates/v/pwn.svg)](https://crates.io/crates/pwn)
[![Documentation](https://img.shields.io/docsrs/pwn/latest)](https://docs.rs/pwn/latest/pwn)
[![CI](https://github.com/clubby789/pwntools/actions/workflows/rust.yml/badge.svg)](https://github.com/clubby789/pwntools/actions/workflows/rust.yml)

## What is this?
A minimal implementation of [Pwntools](https://github.com/gallopsled/pwntools) written in Rust.

## Why is this?
Why not?

## Features
 - A [logging](https://docs.rs/pwn/latest/pwn/logging/index.html) framework
 - [TCP](https://docs.rs/pwn/latest/pwn/tubes/index.html) client/server
 - [Packing/Unpacking](https://docs.rs/pwn/latest/pwn/util/packing/index.html) of integers/bytes
 - High level [ELF](https://docs.rs/pwn/latest/pwn/elf/struct.Elf.html) parsing using [goblin](https://crates.io/crates/goblin)
 - Dynamic runtime behaviour with a configurable global [context](https://docs.rs/pwn/latest/pwn/context/index.html)

## Examples
Usage examples can be found in the [examples](examples) directory, including solutions for retired pwn challenges from Hack THe Box.