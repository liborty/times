#![warn(missing_docs)]
//! Benchmark for timing algorithms
use devtimer::{DevTime};
use indxvec::{Indices, printing::*, Vecops};
use ran::*;

fn report(times:&[u128],timessq:&[u128],names:&[&str],repeats:f64) {
    let timesx = times.mergesort_indexed();
    let times_sorted = timesx.unindex(times,true);
    let names_sorted = timesx.unindex(names,true);
    let timessq_sorted = timesx.unindex(timessq,true);
    for i in 0..names.len() {
        let sx2 = timessq_sorted[i] as f64;
        let sx = times_sorted[i] as f64;
        // standard error as a percentage of the mean
        let ste = 100.0*(repeats*sx2/(sx*sx) - 1.).sqrt();
        println!("{MG}{:<18}{GR}{:>10.0} ± {:>3.2}%{UN}",names_sorted[i],sx/repeats,ste);
    }
}

/// Repeated tests on vectors of magnitudes steps in length, e.g. 10,100,1000  
/// Time n runs of listed closures, identified by names,
/// on random data of type specified by rn, e.g. rn = Rnum::newu64;
/// identified by names listed in names.
pub fn mutbenchu64(
    rn:Rnum,
    magnitudes:usize,
    repeats:usize,
    names:&[&str],
    closures:&[fn(&mut[u64])]) { // concrete type here

    let algno = names.len();
    let mut timer = DevTime::new_simple();

    println!("\n{YL}Nanoseconds of {BL}{}{YL} algorithms, {BL}&mut[u64]{YL} data, {BL}{}{YL} magnitudes, {BL}{}{YL} repeats:{UN}",
        algno,magnitudes,repeats );  

    for mag in 1..magnitudes+1 {  
        let d = 10_usize.pow(mag as u32);      
        println!("\nDimensions: {BL}{}{UN}\n",d);
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
        report(&times,&timessq,names,repeats as f64)
    }
}

/// Repeated tests on vectors of magnitudes steps in length, e.g. 10,100,1000  
/// Time n runs of listed closures, identified by names,
/// on random data of type specified by rn, e.g. rn = Rnum::newf64;
/// identified by names listed in names.
pub fn mutbenchf64(
    rn:Rnum,
    magnitudes:usize,
    repeats:usize,
    names:&[&str],
    closures:&[fn(&mut[f64])]) { // concrete type here

    let algno = names.len();
    let mut timer = DevTime::new_simple();

    println!("\n{YL}Nanoseconds of {BL}{}{YL} algorithms, {BL}&mut[f64]{YL} data, {BL}{}{YL} magnitudes, {BL}{}{YL} repeats:{UN}",
        algno,magnitudes,repeats);
 
    for mag in 1..magnitudes+1 {  
        let d = 10_usize.pow(mag as u32);
        println!("\nDimensions: {BL}{}{UN}\n",d);
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
        report(&times,&timessq,names,repeats as f64)
    }
}


/// Repeated tests on vectors of magnitudes steps in length, e.g. 10,100,1000  
/// Time n runs of listed closures, identified by names,
/// on random data of type specified by rn, e.g. rn = Rnum::newu8;
/// identified by names listed in names.
pub fn mutbenchu8(
    rn:Rnum,
    magnitudes:usize,
    repeats:usize,
    names:&[&str],
    closures:&[fn(&mut[u8])]) { // concrete type here

    let algno = names.len();
    let mut timer = DevTime::new_simple();

    println!("\n{YL}Nanoseconds of {BL}{}{YL} algorithms, {BL}&mut[u8]{YL} data, {BL}{}{YL} magnitudes, {BL}{}{YL} repeats:{UN}",
        algno,magnitudes,repeats);
 
    for mag in 1..magnitudes+1 {  
        let d = 10_usize.pow(mag as u32); 
        println!("\nDimensions: {BL}{}{UN}\n",d);     
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
        report(&times,&timessq,names,repeats as f64)
    }
}

/// Repeated tests on vectors of magnitudes steps in length, e.g. 10,100,1000  
/// Time n runs of listed closures, identified by names,
/// on random data of type specified by rn, e.g. rn = Rnum::newu64;
/// identified by names listed in names.
pub fn benchu64(
    rn:Rnum,
    magnitudes:usize,
    repeats:usize,
    names:&[&str],
    closures:&[fn(&[u64])]) { // concrete type here

    let algno = names.len();
    let mut timer = DevTime::new_simple();

    println!("\n{YL}Nanoseconds of {BL}{}{YL} algorithms, {BL}&[u64]{YL} data, {BL}{}{YL} magnitudes, {BL}{}{YL} repeats:{UN}",
        algno,magnitudes,repeats );
 
    for mag in 1..magnitudes+1 {  
        let d = 10_usize.pow(mag as u32);
        println!("\nDimensions: {BL}{}{UN}\n",d);
        let mut times = vec![0_u128;algno];
        let mut timessq = vec![0_u128;algno];
        for _ in 0..repeats {
            let data = rn.ranv(d).getvu64(); // same concrete type here
            for (i,closure) in closures.iter().enumerate() {
                timer.start();
                closure(&data);
                timer.stop();
                let this_time = timer.time_in_nanos().unwrap();
                times[i] += this_time; 
                timessq[i] += this_time*this_time; 
            }
        } 
        report(&times,&timessq,names,repeats as f64)
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
    closures:&[fn(&[f64])]) { // concrete type here

    let algno = names.len();
    let mut timer = DevTime::new_simple();

    println!("\n{YL}Nanoseconds of {BL}{}{YL} algorithms, {BL}&[f64]{YL} data, {BL}{}{YL} magnitudes, {BL}{}{YL} repeats:{UN}",
        algno,magnitudes,repeats );
 
    for mag in 1..magnitudes+1 {  
        let d = 10_usize.pow(mag as u32);
        println!("\nDimensions: {BL}{}{UN}\n",d);
        let mut times = vec![0_u128;algno];
        let mut timessq = vec![0_u128;algno];
        for _ in 0..repeats {
            let data = rn.ranv(d).getvf64(); // same concrete type here
            for (i,closure) in closures.iter().enumerate() {
                timer.start();
                closure(&data);
                timer.stop();
                let this_time = timer.time_in_nanos().unwrap();
                times[i] += this_time; 
                timessq[i] += this_time*this_time; 
            }
        }   
        report(&times,&timessq,names,repeats as f64)
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
    closures:&[fn(&[u8])]) { // concrete type here

    let algno = names.len();
    let mut timer = DevTime::new_simple();

    println!("\n{YL}Nanoseconds of {BL}{}{YL} algorithms, {BL}&[u8]{YL} data, {BL}{}{YL} magnitudes, {BL}{}{YL} repeats:{UN}",
        algno,magnitudes,repeats );  

    for mag in 1..magnitudes+1 {  
        let d = 10_usize.pow(mag as u32);
        println!("\nDimensions: {BL}{}{UN}\n",d);
        let mut times = vec![0_u128;algno];
        let mut timessq = vec![0_u128;algno];
        for _ in 0..repeats {
            let data = rn.ranv(d).getvu8(); // same concrete type here
            for (i,closure) in closures.iter().enumerate() {
                timer.start();
                closure(&data);
                timer.stop();
                let this_time = timer.time_in_nanos().unwrap();
                times[i] += this_time; 
                timessq[i] += this_time*this_time; 
            }
        }   
        report(&times,&timessq,names,repeats as f64)
    }
}

/// Repeated tests on vectors of vectors of magnitudes steps in length, e.g. 10,100,1000  
/// Time n runs of listed closures, identified by names,
/// on random data of type specified by rn, e.g. rn = Rnum::newu8;
/// identified by names listed in names.
pub fn benchvvu8(
    rn:Rnum,
    points: usize, // number of Vecs in each Vec<Vec<u8>>
    magnitudes:usize,
    repeats:usize,
    names:&[&str],
    closures:&[fn(&[Vec<u8>])]) { // concrete type here

    let algno = names.len();
    let mut timer = DevTime::new_simple();

    println!("\n{YL}Nanoseconds of {BL}{}{YL} algorithms, {BL}&[Vec<u8>]{YL} data, {BL}{}{YL} magnitudes, {BL}{}{YL} points, {BL}{}{YL} repeats:{UN}",
        algno,points,magnitudes,repeats );  
 
    for mag in 1..magnitudes+1 {  
        let d = 10_usize.pow(mag as u32);
        println!("\nDimensions: {BL}{}{UN}\n",d);
        let mut times = vec![0_u128;algno];
        let mut timessq = vec![0_u128;algno];
        for _ in 0..repeats {
            let data = rn.ranvv(d,points).getvvu8(); // closures concrete type 
            for (i,closure) in closures.iter().enumerate() {
                timer.start();
                closure(&data);
                timer.stop();
                let this_time = timer.time_in_nanos().unwrap();
                times[i] += this_time; 
                timessq[i] += this_time*this_time; 
            }
        }   
        report(&times,&timessq,names,repeats as f64)
    }
}

/// Repeated tests on vectors of vectors of magnitudes steps in length, e.g. 10,100,1000  
/// Time n runs of listed closures, identified by names,
/// on random data of type &[Vec<f64>] and rn = Rnum::newf64;
/// identified by names listed in names.
pub fn benchvvf64(
    rn:Rnum,
    points: usize, // number of Vecs in each Vec<Vec<u8>>
    magnitudes:usize,
    repeats:usize,
    names:&[&str],
    closures:&[fn(&[Vec<f64>])]) { // concrete type here

    let algno = names.len();
    let mut timer = DevTime::new_simple();

    println!("\n{YL}Nanoseconds of {BL}{}{YL} algorithms, {BL}&[Vec<f64>]{YL} data, {BL}{}{YL} magnitudes, {BL}{}{YL} points, {BL}{}{YL} repeats:{UN}",
        algno,points,magnitudes,repeats ); 

    for mag in 1..magnitudes+1 {  
        let d = 10_usize.pow(mag as u32);
        println!("\nDimensions: {BL}{}{UN}\n",d); 
        let mut times = vec![0_u128;algno];
        let mut timessq = vec![0_u128;algno];
        for _ in 0..repeats {
            let data = rn.ranvv(d,points).getvvf64(); // closures concrete type 
            for (i,closure) in closures.iter().enumerate() {
                timer.start();
                closure(&data);
                timer.stop();
                let this_time = timer.time_in_nanos().unwrap();
                times[i] += this_time; 
                timessq[i] += this_time*this_time; 
            }
        }   
        report(&times,&timessq,names,repeats as f64)
    }
}

/// Repeated tests on vectors of vectors of magnitudes steps in length, e.g. 10,100,1000  
/// Time n runs of listed closures, identified by names,
/// on random data of type &[Vec<u64>] and rn = Rnum::newu64;
/// identified by names listed in names.
pub fn benchvvu64(
    rn:Rnum,
    points: usize, // number of Vecs in each Vec<Vec<u8>>
    magnitudes:usize,
    repeats:usize,
    names:&[&str],
    closures:&[fn(&[Vec<u64>])]) { // concrete type here

    let algno = names.len();
    let mut timer = DevTime::new_simple();

    println!("\n{YL}Nanoseconds of {BL}{}{YL} algorithms, {BL}&[Vec<f64>]{YL} data, {BL}{}{YL} magnitudes, {BL}{}{YL} points, {BL}{}{YL} repeats:{UN}",
        algno,points,magnitudes,repeats ); 

    for mag in 1..magnitudes+1 {  
        let d = 10_usize.pow(mag as u32);
        println!("\nDimensions: {BL}{}{UN}\n",d); 
        let mut times = vec![0_u128;algno];
        let mut timessq = vec![0_u128;algno];
        for _ in 0..repeats {
            let data = rn.ranvv(d,points).getvvu64(); // closures concrete type 
            for (i,closure) in closures.iter().enumerate() {
                timer.start();
                closure(&data);
                timer.stop();
                let this_time = timer.time_in_nanos().unwrap();
                times[i] += this_time; 
                timessq[i] += this_time*this_time; 
            }
        }   
        report(&times,&timessq,names,repeats as f64)
    }
}