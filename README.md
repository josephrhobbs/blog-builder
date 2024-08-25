# Blog Builder

A simple static web framework, powered by the Rust programming language.

## Usage

Create a new website by typing `blog new <name>`.  This will create a directory with the following structure.

```
<name>
|   blog.toml
|   source
    |   index.txt
```

The directory `source/` is referred to as the "source directory", and it is used to construct the directory
`html/` which is referred to as the "HTML directory" or the "output directory". 