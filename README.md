# Flatten

A simple rust tool for moving all files from directories and subdirectories
down to a base directory via globbing.

## The benefits

1. Handles duplicate file renaming for you.

2. You can define the renaming pattern you desire using simple: `{name}`, `{n}`
   and `{ext}` variables for the `--duplicate-format` flag.

3. Verbose logging for the whole process as well as the results.

4. Convenient to use - Instead of writing a sequence of shell commands - you
   can use **one**.

5. A dry run mode to test what will happen before committing to the changes.

## Usage

Flattening a tree with files is somewhat of a common task. Say - you've used my
[recordings-mover](https://github.com/1Git2Clone/recordings-mover) and now you
want to revert the changes from moving the recordings from a flat structure to
a chronological structure of directories back to a flat structure again. With
`flatten` you can do this by just running:

```sh
./flatten <Path to year directory>/**/*
```

> [!IMPORTANT] Please make sure you're fully aware of the consequences of
> flattening directories as running this script with the input as: `/**/*` or
> `C:\**\*` with sudo/administrator permission for example would cause
> permanent (and incredibly tedious to recover) damage to your system.

## Implications

Due to the limitations of [`glob`](https://docs.rs/glob/latest/glob/), this
program only works with **local paths** or **mounted network paths**, such as:

- UNC shares on Windows;
- Network-mounted filesystems (`NFS`, `CIFS/SMB`, `SSHFS`, etc.) on Linux.

Remote paths using protocols like `sftp://`, `http://`, or `ftp://` are **not supported**.
