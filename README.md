# Replicate

## Overview
replicate is a tool to generate a more general version of its
own command line interface.

## Table of Contents


- [Replicate](#replicate)
  - [Overview](#overview)
  - [Table of Contents](#table-of-contents)
  - [Requirements](#requirements)
  - [Installation](#installation)
  - [Example](#example)
- [Contributing](#contributing)
  - [Prerequisites](#prerequisites)
  - [Clone](#clone)
  - [Build](#build)
  - [Test](#test)
  - [Read](#read)

## Requirements

## Installation


```
 $ comming soon!
```

## Example

Running `$ replicate cli app` will generate a rust crate with
the structure depicted below:

```text
app/
├── Cargo.toml
└── src
    ├── app
    │   ├── lib.rs
    │   └── util
    │       ├── command_prelude.rs
    │       ├── errors.rs
    │       └── mod.rs
    └── bin
        └── app
            ├── cli.rs
            ├── commands
            │   ├── init.rs
            │   └── mod.rs
            └── main.rs
```

# Contributing

Contributions are welcome! No contribution is too small––bug fix, a new feature,
or a typo fix––all are welcome.

* contribution [template]()
* issue [template]()

## Prerequisites

replicate is written in Rust so make sure you have [Rust installed](https://www.rust-lang.org/tools/install).


## Clone

Clone the repository:

```bash
$ git clone https://github.com/ericdeansanchez/replicate.git
```

## Build

cd into the repository and run:

```bash
$ cargo build
```

## Test

Ensure the tests pass on your system (please open an issue if they do not):

```bash
$ cargo test
```

## Read 

These are the docs you really want. Copy & paste the command below in your 
terminal. Make sure `--no-deps` is passed otherwise you'll build documentation 
for all/any dependencies.

```bash
$ cargo doc --no-deps --open
```