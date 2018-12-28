# rayst

[![LICENSE](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

Rust port of the [postcard-sized raytracer](https://twitter.com/lexfrench/status/1049196936161415169) from Andrew Kensler distributed by Pixar at GHC 2018. The initial code written in C/C++ is described by Fabien Sanglard in [a detailed blog post](http://fabiensanglard.net/postcard_pathtracer/).

![Result image, 256 samples per pixel](https://raw.githubusercontent.com/djeedai/rayst/master/assets/pixar_256spp.png)

**Disclaimer:** This Rust port produces the same image as the original C/C++ code, which has been optimized for source code size and is therefore highly de-optimized in terms of performance. On top of that, this is a naive Rust port and my first ever Rust program, so it likely contains extra inefficiencies. I have no intention of even trying to optimize the raytracer, but feedback on the Rust code itself is encouraged!

## Usage

As with the original code, this generates a PPM image on the standard output, best redirected to a file:
```
rayst > pixar.ppm
```

After that the image can be opened with any tool supporting the PPM format, like [Gimp](https://www.gimp.org/) for example.

## Earlier version

Andrew Kensler wrote an earlier postcard-sized raytracer before the one ported here, and presents some details about it in [a blog post](http://eastfarthing.com/blog/2016-01-12-card/) along with some more explanations and a commented code version. Several ports of that earlier code are mentioned, including [a Rust port by Huon Wilson](https://github.com/huonw/card-trace).
