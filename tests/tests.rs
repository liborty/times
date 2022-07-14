#![allow(unused_imports)]
#![allow(dead_code)]
#[cfg(test)]
use indxvec::{ here, F64, printing::*, Indices, Printing, Vecops, Mutops};
use ran::*;
use std::convert::From;
use times::{mutbenchu8,mutbenchu64,mutbenchf64};

const NAMES:[&str;6] = [ "sortm","sorth","mergesort_indexed","hashsort_indexed","muthashsort","mutsort" ];

const CLOSURESU8:[fn(&mut[u8]);6] = [
    |v:&mut[_]| { v.sortm(true); }, 
    |v:&mut[_]| { v.sorth(true); }, 
    |v:&mut[_]| { v.mergesort_indexed(); },
    |v:&mut[_]| { v.hashsort_indexed(); },
    |v:&mut[_]| { v.muthashsort(); },
    |v:&mut[_]| { v.mutsort(); } ];

const CLOSUREU64:[fn(&mut[u64]);6] = [
    |v:&mut[_]| { v.sortm(true); }, 
    |v:&mut[_]| { v.sorth(true); }, 
    |v:&mut[_]| { v.mergesort_indexed(); },
    |v:&mut[_]| { v.hashsort_indexed(); },
    |v:&mut[_]| { v.muthashsort(); },
    |v:&mut[_]| { v.mutsort(); } ];

const CLOSURESF64:[fn(&mut[f64]);6] = [
    |v:&mut[_]| { v.sortm(true); }, 
    |v:&mut[_]| { v.sorth(true); }, 
    |v:&mut[_]| { v.mergesort_indexed(); },
    |v:&mut[_]| { v.hashsort_indexed(); },
    |v:&mut[_]| { v.muthashsort(); },
    |v:&mut[_]| { v.mutsort(); } ];

#[test]
fn benchtests() {
    set_seeds(7777777777_u64);   // intialise random numbers generator
     // Rnum encapsulates the type of the data items
    mutbenchu8(Rnum::newu8(),5,10,&NAMES,&CLOSURESU8); 
    mutbenchu64(Rnum::newu64(),5,10,&NAMES,&CLOSUREU64); 
    mutbenchf64(Rnum::newf64(),5,10,&NAMES,&CLOSURESF64); 
}
