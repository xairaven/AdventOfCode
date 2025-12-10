# 🎄 Advent of Code

[![Rust](https://img.shields.io/badge/Rust-fde7d5?style=for-the-badge&logo=rust&logoColor=black)](#)

This repository contains my solutions for the [Advent of Code](https://adventofcode.com/) programming puzzles.

The project is structured as a **Cargo Workspace**, allowing for shared logic between years while keeping dependencies and compilation artifacts isolated per year.

## 📂 Project Structure

The repository is organized by year, with a shared `common` library for reusable algorithms (grids, pathfinding, math helpers).

```text
AdventOfCode/
├── common/             # Shared library
├── 2025/               # Solutions for 2025
│   ├── inputs/         # Puzzle inputs (ignored by git)
│   └── src/
│       ├── main.rs     # CLI runner
│       └── days        # Daily solution modules
└── Cargo.toml          # Workspace configuration
```

## 🚀 Usage

You can run solutions through shell script, located in year folder.

## Progress

| Year | Stars | Languages | Notes       |
|------|-------|-----------|-------------|
| 2025 | 20/24 | Rust      | In Progress |

## ⚖️ Disclaimer

- **Spoilers**: The source code obviously contains spoilers.

*Happy Hacking!* 🦀