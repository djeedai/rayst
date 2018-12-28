# rayst

[![LICENSE](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

Rust port of the [postcard-sized raytracer](https://twitter.com/lexfrench/status/1049196936161415169) from Andrew Kensler distributed by Pixar at GHC 2018.

The initial code written in C/C++ is described by Fabien Sanglard in [a detailed blog post](http://fabiensanglard.net/postcard_pathtracer/).

As with the original code, this generates a PPM image on the standard output, best redirected to a file:
```
rayst > pixar.ppm
```

**Disclaimer:** This Rust port produces the same image as the C/C++ original code, which means is optimized for source code size and is therefore highly de-optimized in terms of performance. On top of that, this is a naive Rust port and my first ever Rust program, so it likely contains extra inefficiencies. I have no intention of even trying to optimize the raytracer, but feedback on the Rust code itself is encouraged!
