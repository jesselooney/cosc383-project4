[![Tests](https://github.com/jesselooney/cosc383-project4/actions/workflows/ci.yml/badge.svg)](https://github.com/jesselooney/cosc383-project4/actions/workflows/ci.yml)

# Installation/Usage Instructions

To compile this project you'll need to install [Rust](https://www.rust-lang.org/learn/get-started). Once you do that, clone this repository, then run `cargo test` to verify that all of the functionality is working as intended.

# Project Architecture

- `src/`
  - `./main.rs`: The entry point for the program
  - `./bit_pattersn.rs`: Contains the core API used to manipulate images
  - `./detect.rs`: Contains functions used to detect modified images
  - `./transorm.rs`: Contains useful transformations to perform on images during analysis

All of the files have module and function level documentation that should be read in their respective files.
