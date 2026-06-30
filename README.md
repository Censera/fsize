# fsize

`fsize` computes the total disk usage of files or directories and prints it
in a human‑friendly format.

## Usage

fsize [OPTIONS] <PATH>...

Options:
-d, --binary      Use binary (IEC) units (KiB, MiB, etc.)
-r, --raw         Output raw byte count
-b, --byte        Alias for --raw
--in <UNIT>       Force output in a specific unit (e.g., MiB, KB)
-i, --info        Show extra info: type indicator (d/l/-) and modification time

Examples:
fsize file.txt          → 24 KB
fsize -d file.txt       → 20 KiB
fsize -raw file.txt     → 160000
fsize file.txt --in MiB → 0.02 MiB
fsize -i file.txt       → 24 KB - Jun 24 17:32
fsize -i /some/dir      → 1.2 GB d Jun 24 17:32

Aliases: `file`, `filesize`, `fs` (symlink the binary).

## Installation

cargo install --path .
