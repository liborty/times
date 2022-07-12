#![allow(unused_imports)]
#![allow(dead_code)]
#[cfg(test)]
use indxvec::{ here, F64, printing::*, Indices, Printing, Vecops, Mutops};
use ran::*;
use std::convert::From;
use times::bench;

#[test]
fn benchtest()
{ 
    const NAMES:[&str;6] = [ "sortm","sorth","mergesort_indexed","hashsort_indexed","muthashsort","mutsort" ];
    // Here we found it necessary to declare the data argument v as mutable in all closures,
    // even though only the last two require it.
    // The Rust compiler would throw a fit otherwise.
    let closures = [
        |v:&mut [f64]| { v.sortm(true); }, 
        |v:&mut [f64]| { v.sorth(true); }, 
        |v:&mut [f64]| { v.mergesort_indexed(); },
        |v:&mut [f64]| { v.hashsort_indexed(); },
        |v:&mut [f64]| { v.muthashsort(); },
        |v:&mut [f64]| { v.mutsort(); } ];

    set_seeds(7777777777_u64);   // intialise random numbers generator
    let rn = Rnum::newf64(); // specifies the type of data items
    bench(rn,5,10,&NAMES,&closures); 
}
