# RDNS

RDNS is a small CLI utility for performing single and bulk reverse DNS (PTR) lookups.

## Usage

```
RDNS 0.1.0
Joe Banks <joe@jb3.dev>
Utilities for working with reverse DNS.

USAGE:
    rdns [FLAGS] [ADDRESS] [SUBCOMMAND]

FLAGS:
    -h, --help       Prints help information
    -v               Sets the level of verbosity
    -V, --version    Prints version information

ARGS:
    <ADDRESS>    Address (v4 or v6) to perform an RDNS lookup on

SUBCOMMANDS:
    bulk    Bulk RDNS lookups
    help    Prints this message or the help of the given subcommand(s)
```
