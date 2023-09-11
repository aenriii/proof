#![feature(int_roundings)]

mod lib;
use std::fmt::format;

use lib::{*, ttablegen::generateTruthTable, notation::LOGICAL_AND};

fn main() {
    assignments::assignment1();
}
