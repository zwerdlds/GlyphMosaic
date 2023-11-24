# GlyphMosaic
## What is GlyphMosaic?
GlyphMosaic is a graphic design program which re-creates images using glyphs.
In essence, it is supplied a bitmap (source image) and a body of text.
With additional parameters, it uses the color values in the source image to place glyphs from the text.

This project will eventually be a usage demonstrator for the [Roopes](https://docs.rs/roopes/) crate.
It is currently in design, with the `latest` release being the current state of the specification.
Implementation and design iterations are planned to start once initial design is complete.

# Getting Started
Developers are suggested to use `nix-shell` to employ the accompanying `shell.nix` which should include all necessary prerequisites.
Convenience commands (scripts?) are included in the accompanying `justfile`:

## `just build-documentation`
Produces `documenation/Specification.pdf`.


## Current Status
The project is currently in a pre-development stage.
Essentially, tooling and process will be iterated on in small ways while the project design specification document is complete.
Once the project design has a first version, it will transition to a more agile style development pathway.