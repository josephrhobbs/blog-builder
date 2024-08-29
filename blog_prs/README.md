# Blog Builder Parser

This crate provides the `Parser` structure to the Blog Builder.

## Application

The `Parser` structure accepts a list of tokens and outputs a list of (possibly malformed) expressions.  These expressions are validated by the Parser Check Utility (crate `blog_chk`) before they are converted into output HTML.