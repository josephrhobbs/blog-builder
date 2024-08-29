# Blog Builder Error Handling

This crate provides the `BlogError` enumeration and the `BlogReuslt` structure to the Blog Builder, enabling tolerant error handling.

## Application

The `BlogResult` structure is an error-tolerant result structure that may contain zero or more errors.  If it contains at least one error, it discards its resultant value, which may be malformed, and propagates its list of errors up to the end user.