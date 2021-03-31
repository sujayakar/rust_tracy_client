#![feature(const_mut_refs, const_type_name)]

use tracy_client::*;
use std::time::Instant;

fn main() {
    let n = 10000000;
    let start = Instant::now();
    for _ in 0..n {
        let _s = static_span!("a great span");
        drop(_s);
    }
    let duration = start.elapsed();
    println!("Logged {} spans in {:?} ({:?} per span)", n, duration, duration / n);
}
