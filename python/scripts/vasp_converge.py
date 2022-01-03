#!/bin/bash
"""
Show convergence state of VASP relaxation task.
Columns: Step | E0 | Max force
Usage: vasp_converge.py
"""
try:
    import ase.io
except ImportError:
    raise ImportError("Please install ase first.")

try:
    import numpy as np
except ImportError:
    raise ImportError("Please install numpy first.")

try:
    from rich.console import Console
    from rich.table import Table
except ImportError:
    raise ImportError("Please install rich first.")


def get_max_force(atoms):
    return np.sqrt((atoms.get_forces() ** 2).sum(axis=1).max())


if __name__ == "__main__":
    table = Table(title="Vasp calculation result")
    table.add_column("Step")
    table.add_column("E0")
    table.add_column("Max force")
    traj = ase.io.read("OUTCAR", ":")
    energy_prev = 0
    print(f"{'Step':<7}{'E0':<16}{'force':<16}")
    for step, atoms in enumerate(traj):
        energy = atoms.get_potential_energy()
        energy_diff = energy - energy_prev
        energy_prev = energy
        force = get_max_force(atoms)
        result = f"{step:<7}{energy:<16.6f}{force:<16.6f}"
        table.add_row(*result.split())

    console = Console()
    console.print(table)
