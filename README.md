# fsize

`fsize` computes the total disk usage of files or directories and prints it
in a nice format.

## Usage

```ts
fsize [OPTIONS] <PATH>...
```

Options:

```ts
-d, --binary        Use binary (IEC) units (KiB, MiB, etc.)
-r, --raw           Output raw byte count
-b, --byte          Alias for --raw
-u, --unit <UNIT>   Force output in a specific unit (e.g., MiB, KB)
-i, --info          Show extra info: type indicator (d/l/f) and modification time
```

Examples:

```ts
fsize file.txt            24 KB
fsize -b file.txt         20 KiB
fsize -o file.txt         160000
fsize file.txt -u MiB     0.02 MiB
fsize -i file.txt         24 KB f Jun 24 17:32
fsize -i /some/dir        1.2 GB d Jun 24 17:32
```

Aliases:

```ts
`file`, `filesize`, `fs` (symlink the binary).
```

## Installation

```
cargo install --path .
```
