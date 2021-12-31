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

### Manual

- Install `rustup`. [See here](https://rustup.rs/).
- Install `pos2pot`:

  ```bash
  cargo install --git https://github.com/mjhong0708/hanslab_utils pos2pot
  ```

- Install `ndstat`:

  ```bash
    cargo install --git https://github.com/mjhong0708/hanslab_utils ndstat
  ```

- See [wiki](https://github.com/mjhong0708/hanslab_utils/wiki) to configure `pos2pot`.

## How to use

See [wiki](https://github.com/mjhong0708/hanslab_utils/wiki) for detail.

## Future plan

> **Note**: Name of the programs may change.

- `postool`: Manipulate `POSCAR`. ex) make supercell, set constraints, etc.
- `vasp_check`: Quick check of VASP calculation results & statistics
- `vaspmanager`: Manage VASP calculations with command line interface
