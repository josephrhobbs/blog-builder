# Guide to Blog Builder Syntax

The Blog Builder uses a modified form of Markdown to format webpages.  Use this guide to inform your web design and stylesheet development.

## Headers

HTML headers `h1` through `h3` are currently supported.

```
# Lorem Ipsum       => <h1>Lorem Ipsum</h1>

## Dolor Sit Amet   => <h2>Dolor Sit Amet</h2>

### Consectetur     => <h3>Consectetur</h3>
```

## Text Emphasis

Bold and italics are supported.

```
*Adipiscing*        => <em>Adipiscing</em>

_Adipiscing_        => <em>Adipiscing</em>

**Elit**            => <strong>Elit</strong>

__Elit__            => <strong>Elit</strong>
```

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

## Work In Progress Banner

WIP banners are supported using the following syntax.

```
::wip[Information]  => <div class="wip">Information</div>
```

The `wip` control sequence must be placed at the beginning of a line.