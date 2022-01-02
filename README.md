# hanslab_utils

Useful libary and command-line programs.

## List of programs

Grouped by used programming languages.

### Rust

- `pos2pot`: POSCAR to POTCAR
- `ndstat`: Show slurm node state
- `qst`: Alternative of `qstat -u $USER`

### Python

Nothing yet, but planned to be added.

## Installation

### Rust programs

- If not installed, first install rust toolkit `rustup`. This installs `cargo` command which enables installing packages from online repository. [See here for detail](https://rustup.rs/).

  ```bash
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
  source $HOME/.cargo/env
  ```

- Install programs:

  ```bash
  cargo install --git https://github.com/mjhong0708/hanslab_utils <program_name>
  ```

- See [wiki](https://github.com/mjhong0708/hanslab_utils/wiki) to configure `pos2pot`.

## How to use

See [wiki](https://github.com/mjhong0708/hanslab_utils/wiki) for detail.

## Future plan

> **Note**: Name of the programs may change.

- `postool`: Manipulate `POSCAR`. ex) make supercell, set constraints, etc.
- `vasp_check`: Quick check of VASP calculation results & statistics
- `vaspmanager`: Manage VASP calculations with command line interface

## Contribution

Of course, any kind of contributions are welcome!

- If you are willing to contribute code directly, create a pull request.
- If you want some new cool features, post on [feature request section](https://github.com/mjhong0708/hanslab_utils/discussions/categories/feature-request) in discussion.
- IF you found a bug, please share on [bug report section](https://github.com/mjhong0708/hanslab_utils/discussions/categories/bug-report). Optional, but recommended template for bug reporting can be found [here](https://github.com/mjhong0708/hanslab_utils/discussions/3).
