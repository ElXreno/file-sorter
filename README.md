# filesorter 

[![github actions](https://github.com/ElXreno/filesorter/workflows/Rust/badge.svg)](https://github.com/ElXreno/filesorter/actions)
[![dependency status](https://deps.rs/repo/github/elxreno/filesorter/status.svg)](https://deps.rs/repo/github/elxreno/filesorter)

**Simple and ugly file sorter writen in Rust.**

---

```
filesorter 0.2.0
ElXreno <elxreno@gmail.com>
Utility for sorting files in directory writen in Rust

USAGE:
    filesorter [SUBCOMMAND]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    help    Prints this message or the help of the given subcommand(s)
    init    (Re)Initialize configuration file
    sort    Sorting source directory to destination (config file should be initialized first!)
```

## Example:
```bash
filesorter init /home/elxreno/Downloads /home/elxreno/Downloads/Sorted --use-date-pattern
filesorter sort
```