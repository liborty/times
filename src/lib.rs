#![warn(missing_docs)]
//! Benchmark for timing algorithms
use devtimer::{DevTime};
use indxvec::{Indices, printing::*, Vecops};
use ran::*;

/// Repeated tests on vectors of magnitudes steps in length, e.g. 10,100,1000  
/// Time n runs of listed closures, identified by names,
/// on random data of type specified by rn, e.g. rn = Rnum::newu64;
/// identified by names listed in names.
pub fn benchu64(
    rn:Rnum,
    magnitudes:usize,
    repeats:usize,
    names:&[&str],
    closures:&[fn(&mut[u64])]) { // concrete type here

    let algno = names.len();
    let rint = repeats as u128;
    let mut timer = DevTime::new_simple();

    println!("\n{YL}Timing sort algorithms on u64 data (in nanoseconds){UN}");  
 
    for mag in 1..magnitudes+1 {  
        let d = 10_usize.pow(mag as u32);      
        println!("\nTesting sorts on a set of {GR}{}{UN} random vectors\nof length {GR}{}{UN} each:\n",
            repeats,d);
        let mut times = vec![0_u128;algno];
        let mut timessq = vec![0_u128;algno];
        for _ in 0..repeats {
            let gd = rn.ranv(d).getvu64(); // same concrete type here
            for (i,closure) in closures.iter().enumerate() {
                let mut data = gd.clone(); // make a fresh copy, in case mutated
                timer.start();
                closure(&mut data);
                timer.stop();
                let this_time = timer.time_in_nanos().unwrap();
                times[i] += this_time; 
                timessq[i] += this_time*this_time; 
            }
        }   
        let timesx = times.mergesort_indexed();
        let times_sorted = timesx.unindex(&times,true);
        let names_sorted = timesx.unindex(names,true);
        let timessq_sorted = timesx.unindex(&timessq,true);
        
        for i in 0..names.len() {
            println!("{MG}{:<18}{GR}{:>10.0} ± {:>8.0}{UN}",names_sorted[i],times_sorted[i]/rint,
            (((timessq_sorted[i]-times_sorted[i]*times_sorted[i]/rint)/rint) as f64).sqrt()); 
        } 
    }
}

/// Repeated tests on vectors of magnitudes steps in length, e.g. 10,100,1000  
/// Time n runs of listed closures, identified by names,
/// on random data of type specified by rn, e.g. rn = Rnum::newf64;
/// identified by names listed in names.
pub fn benchf64(
    rn:Rnum,
    magnitudes:usize,
    repeats:usize,
    names:&[&str],
    closures:&[fn(&mut[f64])]) { // concrete type here

    let algno = names.len();
    let rint = repeats as u128;
    let mut timer = DevTime::new_simple();

    println!("\n{YL}Timing sort algorithms on f64 data (in nanoseconds){UN}");  
 
    for mag in 1..magnitudes+1 {  
        let d = 10_usize.pow(mag as u32);      
        println!("\nTesting sorts on a set of {GR}{}{UN} random vectors\nof length {GR}{}{UN} each:\n",
            repeats,d);
        let mut times = vec![0_u128;algno];
        let mut timessq = vec![0_u128;algno];
        for _ in 0..repeats {
            let gd = rn.ranv(d).getvf64(); // same concrete type here
            for (i,closure) in closures.iter().enumerate() {
                let mut data = gd.clone(); // make a fresh copy, in case mutated
                timer.start();
                closure(&mut data);
                timer.stop();
                let this_time = timer.time_in_nanos().unwrap();
                times[i] += this_time; 
                timessq[i] += this_time*this_time; 
            }
        }   
        let timesx = times.mergesort_indexed();
        let times_sorted = timesx.unindex(&times,true);
        let names_sorted = timesx.unindex(names,true);
        let timessq_sorted = timesx.unindex(&timessq,true);
        
        for i in 0..names.len() {
            println!("{MG}{:<18}{GR}{:>10.0} ± {:>8.0}{UN}",names_sorted[i],times_sorted[i]/rint,
            (((timessq_sorted[i]-times_sorted[i]*times_sorted[i]/rint)/rint) as f64).sqrt()); 
        } 
    }
}

/// Repeated tests on vectors of magnitudes steps in length, e.g. 10,100,1000  
/// Time n runs of listed closures, identified by names,
/// on random data of type specified by rn, e.g. rn = Rnum::newu8;
/// identified by names listed in names.
pub fn benchu8(
    rn:Rnum,
    magnitudes:usize,
    repeats:usize,
    names:&[&str],
    closures:&[fn(&mut[u8])]) { // concrete type here

    let algno = names.len();
    let rint = repeats as u128;
    let mut timer = DevTime::new_simple();

    println!("\n{YL}Timing sort algorithms on u8 data (in nanoseconds){UN}");  
 
    for mag in 1..magnitudes+1 {  
        let d = 10_usize.pow(mag as u32);      
        println!("\nTesting sorts on a set of {GR}{}{UN} random vectors\nof length {GR}{}{UN} each:\n",
            repeats,d);
        let mut times = vec![0_u128;algno];
        let mut timessq = vec![0_u128;algno];
        for _ in 0..repeats {
            let gd = rn.ranv(d).getvu8(); // same concrete type here
            for (i,closure) in closures.iter().enumerate() {
                let mut data = gd.clone(); // make a fresh copy, in case mutated
                timer.start();
                closure(&mut data);
                timer.stop();
                let this_time = timer.time_in_nanos().unwrap();
                times[i] += this_time; 
                timessq[i] += this_time*this_time; 
            }
        }   
        let timesx = times.mergesort_indexed();
        let times_sorted = timesx.unindex(&times,true);
        let names_sorted = timesx.unindex(names,true);
        let timessq_sorted = timesx.unindex(&timessq,true);
        
        for i in 0..names.len() {
            println!("{MG}{:<18}{GR}{:>10.0} ± {:>8.0}{UN}",names_sorted[i],times_sorted[i]/rint,
            (((timessq_sorted[i]-times_sorted[i]*times_sorted[i]/rint)/rint) as f64).sqrt()); 
        } 
    }
}