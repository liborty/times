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
        |v:&mut [u8]| { v.sortm(true); }, 
        |v:&mut [u8]| { v.sorth(true); }, 
        |v:&mut [u8]| { v.mergesort_indexed(); },
        |v:&mut [u8]| { v.hashsort_indexed(); },
        |v:&mut [u8]| { v.muthashsort(); },
        |v:&mut [u8]| { v.mutsort(); } ];

    set_seeds(7777777777_u64);   // intialise random numbers generator
    let rn = Rnum::newu8(); // specifies the type of data items
    bench(rn,5,10,&NAMES,&closures); 
}
