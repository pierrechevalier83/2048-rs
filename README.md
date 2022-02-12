[![Build Status](https://travis-ci.org/pierrechevalier83/2048-rs.svg?branch=master)](https://travis-ci.org/pierrechevalier83/2048-rs)

## 2048-rs - rust implementation of the popular game

*   We use [matrix_display](https://github.com/pierrechevalier83/matrix_display) to pretty print the matrix in all its colours and unicode glory.
*   We use [crossterm](https://github.com/crossterm-rs/crossterm) for handling user input asynchronously.

![alt tag](https://raw.githubusercontent.com/pierrechevalier83/2048-rs/master/screenshots/demo.gif)

## Install

*   On Arch Linux:
    `yaourt 2048-rs`
*   With cargo:
    `cargo install game-2048`

## Download

`git clone git@github.com:pierrechevalier83/2048-rs.git`

## Build

`cargo build --release`

## Run

`cargo run --release`
