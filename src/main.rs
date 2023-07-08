#![allow(unused)]
#![allow(dead_code)]

use metaballs::run;

fn main() {
    pollster::block_on(run());
}
