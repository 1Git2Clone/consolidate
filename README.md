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
