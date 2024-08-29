# Blog Builder Parser Check Utility

This crate provides the `validate` function to the Blog Builder.

## Application

The `validate` function accepts a list of parsed `Expressions` and checks for errors in those expressions.  If it finds any, it returns a list of errors.  Otherwise, it returns a `BlogResult::Ok` message.  This is used by the Converter Utility (crate `blog_cvt`) to validate parsed expressions before attempting to emit them as HTML code.