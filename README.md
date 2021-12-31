# hanslab_utils

Useful libary and command-line programs. Written in `rust` programming language.
These tools were initially developed with `python`, but migrated to `rust`.

## Why rust?

### For users

- Easy to build and install
- Very fast (comparable to `C`/`C++`)

### For developers

- Good at handling errors
- Easy to deploy (no need to consider all the dependencies...)
- Can write fast programs with less effort than `C`/`C++`

## Installation

### Automatic install

Run following commands to install automatically.

```bash
git clone https://github.com/mjhong0708/hanslab_utils
cd hanslab_utils
bash install.sh
```

### Manual

- Install `rustup`. [See here](https://rustup.rs/).
- Clone this repository.
- Run `cargo build --release` to generate binaries.
- Copy binaries in `target/release/` to directory in `PATH`, such as `~/.local/bin`.
  - Currently `pos2pot` and `ndstat` exist.

## How to use

See [wiki](https://github.com/mjhong0708/hanslab_utils/wiki).

## TODO

### pos2pot

- Currently no todo

### vasp_parser

- `Poscar`: support cartesian coordinates (currently only direct)
