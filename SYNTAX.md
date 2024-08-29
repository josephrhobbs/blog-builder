# Guide to Blog Builder Syntax

The Blog Builder uses a modified form of Markdown to format webpages.  Use this guide to inform your web design and stylesheet development.

## Headers

HTML headers `h1` through `h6` are supported.

```
# Lorem Ipsum       => <h1>Lorem Ipsum</h1>

## Dolor Sit Amet   => <h2>Dolor Sit Amet</h2>

### Consectetur     => <h3>Consectetur</h3>

#### Lorem Ipssum   => <h4>Lorem Ipsum</h4>

##### Dolor         => <h5>Dolor</h5>

###### Sit Amet     => <h6>Sit Amet</h6>
```

## Text Emphasis

Bold and italics are supported.

```
*Adipiscing*        => <em>Adipiscing</em>

_Adipiscing_        => <em>Adipiscing</em>

**Elit**            => <strong>Elit</strong>

__Elit__            => <strong>Elit</strong>
```

Note that, unlike some Markdown programs, you must break out emphasized text with spaces as paragraphs are delimited by spaces.  That is, you may not emphasize the middle of a word.

## Hyperlinks

Hyperlinks are supported in the Markdown style.

```
[Google](https://google.com/)
```

## Images

Images are supported using the following syntax.

```
::image[alternate text][/path/to/image]
```

The `image` control sequence must be placed at the beginning of a line.

## Notice Banner

Notice banners are supported using the following syntax.

```
::notice[Information]  => <div class="notice">Information</div>
```

The `notice` control sequence must be placed at the beginning of a line.

## Tile Hyperlinks

Tile hyperlinks are supported using the following syntax.

```
::tile[Title][Short Description][/path/to/link][/path/to/image]
```

This generates an HTML sequence similar to the one below (note the two URIs are omitted).

```
<div class="tile">
    <div>
        Title
    </div>
    <br>
    <div class="desc">
        Short Description
    </div>
</div>
```

The `tile` control sequence must be placed at the beginning of a line.