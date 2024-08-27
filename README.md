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
`html/` which is referred to as the "HTML directory" or the "output directory".  The output directory acts as
the root for the published website.  That is, `<name>/html/` will map to `/` on the web server.

## Adding Pages

Create a new page by creating a text file anywhere in `source/`.

```
<name>
|   blog.toml
|   source
    |   index.txt
    |   something
        | new-page.txt
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

*Note*: this built-in style will be overridden if `site.stylesheet` is set.

## Adding Media

Create the `media/` subdirectory inside of `source/`.

```
<name>
|   blog.toml
|   source
    |   index.txt
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
    |   index.txt
    |   media
        | example.png
|   html
    |   index.html
    |   media
        | example.png
```