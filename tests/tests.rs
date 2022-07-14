#![allow(unused_imports)]
#![allow(dead_code)]
#[cfg(test)]
use indxvec::{ here, F64, printing::*, Indices, Printing, Vecops, Mutops};
use ran::*;
use std::convert::From;
use times::{benchu8,benchu64,benchf64};

const NAMES:[&str;6] = [ "sortm","sorth","mergesort_indexed","hashsort_indexed","muthashsort","mutsort" ];

#[test]
fn benchtests() {
    set_seeds(7777777777_u64);   // intialise random numbers generator

    let closuresu8 = [
        |v:&mut[_]| { v.sortm(true); }, 
        |v:&mut[_]| { v.sorth(true); }, 
        |v:&mut[_]| { v.mergesort_indexed(); },
        |v:&mut[_]| { v.hashsort_indexed(); },
        |v:&mut[_]| { v.muthashsort(); },
        |v:&mut[_]| { v.mutsort(); } ];
    benchu8(Rnum::newu8(),5,10,&NAMES,&closuresu8); 

    let closuresu64 = [
        |v:&mut[_]| { v.sortm(true); }, 
        |v:&mut[_]| { v.sorth(true); }, 
        |v:&mut[_]| { v.mergesort_indexed(); },
        |v:&mut[_]| { v.hashsort_indexed(); },
        |v:&mut[_]| { v.muthashsort(); },
        |v:&mut[_]| { v.mutsort(); } ];
    // Rnum encapsulates the type of the data items
    benchu64(Rnum::newu64(),5,10,&NAMES,&closuresu64); 

    let closuresf64 = [
        |v:&mut[_]| { v.sortm(true); }, 
        |v:&mut[_]| { v.sorth(true); }, 
        |v:&mut[_]| { v.mergesort_indexed(); },
        |v:&mut[_]| { v.hashsort_indexed(); },
        |v:&mut[_]| { v.muthashsort(); },
        |v:&mut[_]| { v.mutsort(); } ];

    set_seeds(7777777777_u64);   // intialise random numbers generator
    // Rnum encapsulates the type of the data items
    benchf64(Rnum::newf64(),5,10,&NAMES,&closuresf64); 
}
