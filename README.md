# `anno`

[![Build](https://github.com/alexandru-dinu/anno/actions/workflows/build.yml/badge.svg)](https://github.com/alexandru-dinu/anno/actions/workflows/build.yml)

Annotate (binary) files with useful metadata.

## Motivation

When working with lots of heterogeneous (binary) files, it is important to maintain a high-level description of the contents, so that you can immediately understand what's inside, without having to open or explore the data.

There are various solutions for this problem, each with their own advantages and drawbacks, e.g.:

1. Maintaining a `README` file with description of file contents
2. Part of the filetype: [Apache Parquet Metadata](https://parquet.apache.org/docs/file-format/metadata/)
3. [Augmenting serialization protocols](https://github.com/opskrift/opskrift/blob/main/opskrift/augmented_pickle.py) (like [pickle](https://docs.python.org/3/library/pickle.html) in Python) to add a new layer of metadata as follows:
```
<metadata>
<body (actual data)>
```
so you can read only the `<metadata>` bit, without touching actual data. However, this also constrains the output file format and may not be practical for all purposes.

It would be great if we can have a universal solution, maybe with support from the file system, so tools like `stat(2)` can display this metadata. For instance, consider the following hypothetical output:
```console
$ stat foo.bin
  File: foo.bin
  Size: 10485760        Blocks: 20480      IO Block: 4096   regular file
Device: 259,3   Inode: 14585614    Links: 1
Access: (0644/-rw-r--r--)  Uid: ( 1000/    alex)   Gid: ( 1000/    alex)
Access: 2022-12-29 22:06:14.907979734 +0200
Modify: 2022-12-29 22:06:14.927979366 +0200
Change: 2022-12-29 22:06:14.927979366 +0200
 Birth: 2022-12-29 22:06:14.907979734 +0200
  Anno: A neat binary file! # <- something like this
```

There is: [`getfattr(1)`](https://man7.org/linux/man-pages/man1/getfattr.1.html) to get **extended attributes** of filesystem objects However, this feature is not universally supported:
```console
$ getfattr -n description foo.bin
foo.bin: description: Operation not supported
```

## Solution

`anno` tries to offer sensible solution to this problem by maintaining a mapping from a UID (unique ID, e.g. hash / checksum) of the input file to its metadata, so you can easily move / copy the data without having to worry about breaking the metadata.

`anno` uses a root dir (e.g. `~/.local/share/anno-dir`) inside which it stores metadata as JSON files named by the file's UID.
For example, if you have a file with a UID `1234abcd`, its metadata will be stored to `~/.local/share/anno-dir/1234abcd`.

The current UID implementation uses SHA256.

## Usage

```
Usage: anno [OPTIONS]

Options:
  -r, --read <FILE>   Read anno for given file.
  -w, --write <FILE>  Write anno for given file.
  -h, --help          Print help information
  -V, --version       Print version information
```