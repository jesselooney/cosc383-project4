[![Tests](https://github.com/jesselooney/cosc383-project4/actions/workflows/ci.yml/badge.svg)](https://github.com/jesselooney/cosc383-project4/actions/workflows/ci.yml)

# Installation/Usage Instructions

To compile this project you'll need to install [Rust](https://www.rust-lang.org/learn/get-started). Once you do that, clone this repository, then run `cargo test` to verify that all of the functionality is working as intended.

# Project Architecture

- `./src/`
  - `main.rs`: The entry point for the program
  - `decode.rs`: Stores all of the solutions and notes we've found so far
  - `extract.rs`: Functions to iterate over individual bits of an image and grab data
  - `iteration_order.rs`: Helpers for iterating over pixels in different directions
  - `automatic.rs`: Functions that attempt to automatically parse data out of images
  - `extensions.rs`: Extensions to code from existing libraries we're using
  - `helpers.rs`: Misc utility functions for use in solving images

All of the files have module and function level documentation that should be read in their respective files.
