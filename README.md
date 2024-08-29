# Blog Builder

![Rust](https://github.com/josephrhobbs/blog-builder/actions/workflows/rust.yml/badge.svg)

A simple static web framework, powered by the Rust programming language.

## Installation

The Blog Builder may be installed with an up-to-date installation of stable Rust.

Minimum Supported Rust Version (MSRV): **Rust 1.80**.

Clone this repository to your computer and, using `cargo install`, build and install the package.

```bash
$ git clone https://github.com/josephrhobbs/blog-builder.git
$ cargo install --path .
```

## Usage

Create a new website by typing `blog new <name>`.  This will create a directory with the following structure.

```
<name>
|   blog.toml
|   source
    |   index.md
```

The directory `source/` is referred to as the "source directory", and it is used to construct the directory
`html/` which is referred to as the "HTML directory" or the "output directory".  The output directory acts as
the root for the published website.  That is, `<name>/html/` will map to `/` on the web server.

## Adding Pages

Create a new page by creating a Markdown file anywhere in `source/`.

**Note**: all source files must have `md` extension or the Blog Builder will not recognize them.

```
<name>
|   blog.toml
|   source
    |   index.md
    |   something
        | new-page.md
```

Execute `blog build`.  This new page will be accessible at `html/something/new-page.html` on your machine and `/something/new-page.html` on your website.

## Adding a Menu

Create a menu by updating `blog.toml`.

```toml
[menu]
names = []
links = []
```

Each entry in `names` will correspond to an entry in `links`.  Entries in `names` are visible to the user, whereas entries in `links` represent URIs.

## Adding an Analytics Tag

Create an analytics tag in the `source/` directory (for example, `source/analytics-tag.html`).

**Note**: you may use any extension except `md` for this file, but the `html` extension is recommended.

Update `blog.toml`.

```toml
[analytics]
tag = "analytics-tag.html"
```

The file path is interpreted to be relative to `source/` and it will be linked immediately after `<head>` in all HTML output files.

## Adding an Icon

You may add a favicon for a website by updating `blog.toml`.

```toml
[site]
name = "My Website"
icon = "media/icon.ico"
```

The path associated with `site.icon` is interpreted to be relative to `source/`.

## Adding a Style

You may add a built-in style by setting `site.style` in `blog.toml`.

```toml
[site]
name = "My Website"
style = "tech"
```

## Adding Media

Create the `media/` subdirectory inside of `source/`.

```
<name>
|   blog.toml
|   source
    |   index.md
    |   media
        | example.png
```

You may create any number of media files (for example: images, video, audio, etc.) in this directory.

Ensure to update your `blog.toml` file to reflect the new media.  Note that you do not include the `media/`
directory in the file path because all media is assumed to be stored in the `media/` directory.

```toml
[media]
include = ["example.png"]
```

Execute `blog build`.  You will see that `html/media/` will include all media indicated in `blog.toml`.

```
<name>
|   blog.toml
|   source
    |   index.md
    |   media
        | example.png
|   html
    |   index.html
    |   media
        | example.png
```