# Bridging Autotools with Cargo

This is a quite simple example of hacking [autotools][1] (a guide for it [here][1a]), so it can link
[rust][2] components built using [cargo][3]

[1]: https://en.wikipedia.org/wiki/GNU_Build_System
[1a]: http://autotools.io
[2]: https://www.rust-lang.org
[3]: http://doc.crates.io/

## Why

Cargo works great but is quite rust specific. Autotools is what a good number of projects use.
There is the idea of leveraging the fact rust can generate native object code C-compatible to
replace some (stale) C/C++ codebase with something more maintainable chunk by chunk.

