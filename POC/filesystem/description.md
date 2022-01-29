# Overview

As of now this is a simple shell used to interact with the front end portion of a file system - in this case NTFS, although a replacment pseudo-filesystem will eventually be implemented.

# Language details

This is written in Rust as it generates smaller binaries than C++ does and has some useful features that C lacks such as vectors.

# Errors

All mechanisms which might be implemented to clear the `userInstructCache` vector after it has been processed do not work - at least not in the obvious and expected manner.

Methods which have been tried include:
```rust
userInstructCache.clear()

let mut userIntstructCache = Vec::new() //yes its bad practice but I didn't know what else to do

userInstructCache.drain(..)
```
