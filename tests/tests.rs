//! These tests compare sort algorithms implemented in crate `indxvec`
//! and demonstrate that our `muthashsort` is generally faster than the best Rust
//! `sort_unstable_by` for more than the order of 10 sort items.
#![allow(unused_imports)]
#![allow(dead_code)]
#[cfg(test)]
use indxvec::{ here, printing::*, Indices, Printing, Vecops, Mutops};
use ran::*;
use std::convert::From;
use times::{bench,mutbenchu8,mutbenchu16,mutbenchu64,mutbenchf64};

#[test]
fn benchtests() {
    const NAMES:[&str;3] = [ "muthashsort","rust-sort","mutisort" ];

    const CLOSURESU8:[fn(&mut[u8]);3] = [
    |v:&mut[_]| { v.muthashsort(|t:&u8| *t as f64); },
    |v:&mut[_]| { v.sort_unstable(); },
    |v:&mut[_]| { v.mutisort(0..v.len(),|a,b| a.cmp(b)); } ];

    const CLOSURESU16:[fn(&mut[u16]);3] = [
        |v:&mut[_]| { v.muthashsort(|t:&u16| *t as f64); },
        |v:&mut[_]| { v.sort_unstable(); },
        |v:&mut[_]| { v.mutisort(0..v.len(),|a,b| a.cmp(b)); } ];

    const CLOSURESF64:[fn(&mut[f64]);3] = [
    |v:&mut[_]| { v.muthashsort(|t:&f64| *t); },
    |v:&mut[_]| { v.sort_unstable_by(|a,b| a.total_cmp(b)); },
    |v:&mut[_]| { v.mutisort(0..v.len(),|a,b| a.total_cmp(b)); } ];

    set_seeds(0);   // intialise random numbers generator
     // Rnum encapsulates the type of the data items
    mutbenchu8(5..10000,2000,10,&NAMES,&CLOSURESU8); 
    mutbenchu16(5..10000,2000,10,&NAMES,&CLOSURESU16); 
    mutbenchf64(50..10000,3000,20,&NAMES,&CLOSURESF64); 
}

#[test]
fn rantest() {
    const D:usize = 10000;
    const N:usize = 20;
    println!("\n{GR}Generating {} sets of vectors of length {} each{UN}",N, D );

    const NAMES:[&str;4] = [ "ranvv_u8","ranvv_u16","ranvv_u64","ranvv_f64" ];

    const CLOSURES:[fn();4] = [
        || { ranvv_u8(N,D).unwrap(); }, 
        || { ranvv_u16(N,D).unwrap(); }, 
        || { ranvv_u64(N,D).unwrap(); }, 
        || { ranvv_f64(N,D).unwrap(); } ];

    set_seeds(0);   // intialise random numbers generator
    // Rnum encapsulates the type of the data items
    bench(8,&NAMES,&CLOSURES);
}
