//! These tests compare sort algorithms implemented in crate `indxvec`
//! and demonstrate that our `muthashsort` is generally faster than the best Rust
//! `sort_unstable_by` for more than the order of 10 sort items.
#![allow(unused_imports)]
#![allow(dead_code)]
#[cfg(test)]
use indxvec::{ here, printing::*, Indices, Printing, Vecops, Mutops};
use ran::{*,generators::{ranvvu8,ranvvu64,ranvvf64}};
use std::convert::From;
use times::{bench,mutbenchu8,mutbenchu64,mutbenchf64};

#[test]
fn benchtests() {
    const NAMES:[&str;6] = [ "merge-sort","hash-sort","mergesort_indexed","hashsort_indexed","muthashsort","rust-sort" ];

    const CLOSURESU8:[fn(&mut[u8]);6] = [
    |v:&mut[_]| { v.sortm(true); }, 
    |v:&mut[_]| { v.sorth(true); }, 
    |v:&mut[_]| { v.mergesort_indexed(); },
    |v:&mut[_]| { v.hashsort_indexed(); },
    |v:&mut[_]| { v.muthashsort(); },
    |v:&mut[_]| { v.mutquicksort(); } ];


    const CLOSURESF64:[fn(&mut[f64]);6] = [
    |v:&mut[_]| { v.sortm(true); }, 
    |v:&mut[_]| { v.sorth(true); }, 
    |v:&mut[_]| { v.mergesort_indexed(); },
    |v:&mut[_]| { v.hashsort_indexed(); },
    |v:&mut[_]| { v.muthashsort(); },
    |v:&mut[_]| { v.mutquicksort(); } ];

    set_seeds(7777777777_u64);   // intialise random numbers generator
     // Rnum encapsulates the type of the data items
    mutbenchu8(Rnum::newu8(),5..10000,2000,10,&NAMES,&CLOSURESU8); 
    mutbenchf64(Rnum::newf64(),50..10000,3000,10,&NAMES,&CLOSURESF64); 
}

#[test]
fn rantest() {
    const D:usize = 10000;
    const N:usize = 20;
    println!( "{GR}Generating {} sets of vectors of length {} each{UN}",N, D );

    const NAMES:[&str;3] = [ "ranvvu8","ranvvu64","ranvvf64" ];

    const CLOSURES:[fn();3] = [
        || { ranvvu8(D,N).unwrap(); }, 
        || { ranvvu64(D,N).unwrap(); }, 
        || { ranvvf64(D,N).unwrap(); } ];

    set_seeds(7777777777_u64);   // intialise random numbers generator
    // Rnum encapsulates the type of the data items
    bench(4,&NAMES,&CLOSURES);
}
