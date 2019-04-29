#![feature(test)]

extern crate core;
extern crate test;

use core::hazard;

use test::Bencher;

#[bench]
fn bench_hazard(b: &mut Bencher) {
    b.iter(|| {
        hazard::generate_hazard();
    })
}
