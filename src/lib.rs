#![warn(missing_docs)]
//! Benchmark for timing algorithms
use devtimer::{DevTime};
use indxvec::{Indices, printing::*, Vecops};
use medians::{Median};
use ran::{*, generators::get_seed};

fn report(names:&[&str], meds:&[f64], stderrs:&[f64]) {
    let medsx = meds.mergesort_indexed();
    let meds_sorted = medsx.unindex(meds,true);
    let names_sorted = medsx.unindex(names,true); 
    let stderrs_sorted = medsx.unindex(stderrs,true); 
    for i in 0..names.len() { 
        println!("{MG}{:<18}{GR}{:>10.0} ± {:>3.2}%{UN}",
        names_sorted[i],meds_sorted[i],stderrs_sorted[i]);
    }
}

/// Tests of listed `closures`, named in `names`,
/// on random data vectors of type specified by `rn` 
/// and lengths of increasing `magnitudes` in multiples of 10, e.g. 10,100,1000  
/// `repeats` runs of each closure for each magnitude 
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
        let mut meds = Vec::with_capacity(algno);
        let mut stderrs = Vec::with_capacity(algno);
        let d = 10_usize.pow(mag as u32);      
        println!("\nDimensions: {BL}{}{UN}\n",d);
        let seed = get_seed(); // store the seed, whatever it is
        for closure in closures {
            // reintialise random numbers generator to the same seed for each closure
            set_seeds(seed);
            let mut times:Vec<f64> = Vec::with_capacity(repeats);    
            for _ in 0..repeats {
                let mut data = rn.ranv(d).getvu64(); // different for each repeat 
                timer.start();
                closure(&mut data);
                timer.stop();
                let this_time = timer.time_in_nanos().unwrap() as f64;
                times.push(this_time); 
            };
            let medmad = times.medstats();
            meds.push(medmad.centre);
            stderrs.push(100.0*medmad.dispersion/medmad.centre);
        } 
    report(names,&meds,&stderrs);
    }
}

/// Tests of listed `closures`, named in `names`,
/// on random data vectors of type specified by `rn` 
/// and lengths of increasing `magnitudes` in multiples of 10, e.g. 10,100,1000  
/// `repeats` runs of each closure for each magnitude 
pub fn mutbenchf64(
    rn:Rnum,
    magnitudes:usize,
    repeats:usize,
    names:&[&str],
    closures:&[fn(&mut[f64])]) { // concrete type here

    let algno = names.len();
    let mut timer = DevTime::new_simple();
    println!("\n{YL}Nanoseconds of {BL}{}{YL} algorithms, {BL}&mut[f64]{YL} data, {BL}{}{YL} magnitudes, {BL}{}{YL} repeats:{UN}",
        algno,magnitudes,repeats );
    for mag in 1..magnitudes+1 {  
        let mut meds = Vec::with_capacity(algno);
        let mut stderrs = Vec::with_capacity(algno);
        let d = 10_usize.pow(mag as u32);      
        println!("\nDimensions: {BL}{}{UN}\n",d);
        let seed = get_seed(); // store the seed, whatever it is
        for closure in closures {
            // reintialise random numbers generator to the same seed for each closure
            set_seeds(seed);
            let mut times:Vec<f64> = Vec::with_capacity(repeats);    
            for _ in 0..repeats {
                let mut data = rn.ranv(d).getvf64(); // different for each repeat 
                timer.start();
                closure(&mut data);
                timer.stop();
                let this_time = timer.time_in_nanos().unwrap() as f64;
                times.push(this_time); 
            };
            let medmad = times.medstats();
            meds.push(medmad.centre);
            stderrs.push(100.0*medmad.dispersion/medmad.centre);
        } 
    report(names,&meds,&stderrs);
    }
}

/// Tests of listed `closures`, named in `names`,
/// on random data vectors of type specified by `rn` 
/// and lengths of increasing `magnitudes` in multiples of 10, e.g. 10,100,1000  
/// `repeats` runs of each closure for each magnitude 
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
        let mut meds = Vec::with_capacity(algno);
        let mut stderrs = Vec::with_capacity(algno);
        let d = 10_usize.pow(mag as u32);      
        println!("\nDimensions: {BL}{}{UN}\n",d);
        let seed = get_seed(); // store the seed, whatever it is
        for closure in closures {
            // reintialise random numbers generator to the same seed for each closure
            set_seeds(seed);
            let mut times:Vec<f64> = Vec::with_capacity(repeats);    
            for _ in 0..repeats {
                let mut data = rn.ranv(d).getvu8(); // different for each repeat 
                timer.start();
                closure(&mut data);
                timer.stop();
                let this_time = timer.time_in_nanos().unwrap() as f64;
                times.push(this_time); 
            };
            let medmad = times.medstats();
            meds.push(medmad.centre);
            stderrs.push(100.0*medmad.dispersion/medmad.centre);
        } 
    report(names,&meds,&stderrs);
    }
}

/// Tests of listed `closures`, named in `names`,
/// on random data vectors of type specified by `rn` 
/// and lengths of increasing `magnitudes` in multiples of 10, e.g. 10,100,1000  
/// `repeats` runs of each closure for each magnitude 
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
        let mut meds = Vec::with_capacity(algno);
        let mut stderrs = Vec::with_capacity(algno);
        let d = 10_usize.pow(mag as u32);      
        println!("\nDimensions: {BL}{}{UN}\n",d);
        let seed = get_seed(); // store the seed, whatever it is
        for closure in closures {
            // reintialise random numbers generator to the same seed for each closure
            set_seeds(seed);
            let mut times:Vec<f64> = Vec::with_capacity(repeats);    
            for _ in 0..repeats {
                let data = rn.ranv(d).getvu64(); // different for each repeat 
                timer.start();
                closure(&data);
                timer.stop();
                let this_time = timer.time_in_nanos().unwrap() as f64;
                times.push(this_time); 
            };
            let medmad = times.medstats();
            meds.push(medmad.centre);
            stderrs.push(100.0*medmad.dispersion/medmad.centre);
        } 
    report(names,&meds,&stderrs);
    }
}

/// Tests of listed `closures`, named in `names`,
/// on random data vectors of type specified by `rn` 
/// and lengths of increasing `magnitudes` in multiples of 10, e.g. 10,100,1000  
/// `repeats` runs of each closure for each magnitude 
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
        let mut meds = Vec::with_capacity(algno);
        let mut stderrs = Vec::with_capacity(algno);
        let d = 10_usize.pow(mag as u32);      
        println!("\nDimensions: {BL}{}{UN}\n",d);
        let seed = get_seed(); // store the seed, whatever it is
        for closure in closures {
            // reintialise random numbers generator to the same seed for each closure
            set_seeds(seed);
            let mut times:Vec<f64> = Vec::with_capacity(repeats);    
            for _ in 0..repeats {
                let data = rn.ranv(d).getvf64(); // different for each repeat 
                timer.start();
                closure(&data);
                timer.stop();
                let this_time = timer.time_in_nanos().unwrap() as f64;
                times.push(this_time); 
            };
            let medmad = times.medstats();
            meds.push(medmad.centre);
            stderrs.push(100.0*medmad.dispersion/medmad.centre);
        } 
    report(names,&meds,&stderrs);
    }
}

/// Tests of listed `closures`, named in `names`,
/// on random data vectors of type specified by `rn` 
/// and lengths of increasing `magnitudes` in multiples of 10, e.g. 10,100,1000  
/// `repeats` runs of each closure for each magnitude 
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
        let mut meds = Vec::with_capacity(algno);
        let mut stderrs = Vec::with_capacity(algno);
        let d = 10_usize.pow(mag as u32);      
        println!("\nDimensions: {BL}{}{UN}\n",d);
        let seed = get_seed(); // store the seed, whatever it is
        for closure in closures {
            // reintialise random numbers generator to the same seed for each closure
            set_seeds(seed);
            let mut times:Vec<f64> = Vec::with_capacity(repeats);    
            for _ in 0..repeats {
                let data = rn.ranv(d).getvu8(); // different for each repeat 
                timer.start();
                closure(&data);
                timer.stop();
                let this_time = timer.time_in_nanos().unwrap() as f64;
                times.push(this_time); 
            };
            let medmad = times.medstats();
            meds.push(medmad.centre);
            stderrs.push(100.0*medmad.dispersion/medmad.centre);
        } 
    report(names,&meds,&stderrs);
    }
}

/// Tests of listed `closures`, named in `names`,
/// on random data vectors of type specified by `rn` 
/// and lengths of increasing `magnitudes` in multiples of 10, e.g. 10,100,1000  
/// `repeats` runs of each closure for each magnitude 
pub fn benchvvu8(
    rn:Rnum,
    points: usize, // number of Vecs in each Vec<Vec<u8>>
    magnitudes:usize,
    repeats:usize,
    names:&[&str],
    closures:&[fn(&[Vec<u8>])]) { // concrete type here

    let algno = names.len();
    let mut timer = DevTime::new_simple();
    println!("\n{YL}Nanoseconds of {BL}{}{YL} algorithms, {BL}&[Vec<u8>]{YL} data, {BL}{}{YL} magnitudes, {BL}{}{YL} repeats:{UN}",
        algno,magnitudes,repeats );
    for mag in 1..magnitudes+1 {  
        let mut meds = Vec::with_capacity(algno);
        let mut stderrs = Vec::with_capacity(algno);
        let d = 10_usize.pow(mag as u32);      
        println!("\nDimensions: {BL}{}{UN}\n",d);
        let seed = get_seed(); // store the seed, whatever it is
        for closure in closures {
            // reintialise random numbers generator to the same seed for each closure
            set_seeds(seed);
            let mut times:Vec<f64> = Vec::with_capacity(repeats);    
            for _ in 0..repeats {
                let data = rn.ranvv(d,points).getvvu8(); // different for each repeat 
                timer.start();
                closure(&data);
                timer.stop();
                let this_time = timer.time_in_nanos().unwrap() as f64;
                times.push(this_time); 
            };
            let medmad = times.medstats();
            meds.push(medmad.centre);
            stderrs.push(100.0*medmad.dispersion/medmad.centre);
        } 
    report(names,&meds,&stderrs);
    }
}

/// Tests of listed `closures`, named in `names`,
/// on random data vectors of type specified by `rn` 
/// and lengths of increasing `magnitudes` in multiples of 10, e.g. 10,100,1000  
/// `repeats` runs of each closure for each magnitude 
pub fn benchvvf64(
    rn:Rnum,
    points: usize, // number of Vecs in each Vec<Vec<u8>>
    magnitudes:usize,
    repeats:usize,
    names:&[&str],
    closures:&[fn(&[Vec<f64>])]) { // concrete type here

    let algno = names.len();
    let mut timer = DevTime::new_simple();
    println!("\n{YL}Nanoseconds of {BL}{}{YL} algorithms, {BL}&[Vec<f64>]{YL} data, {BL}{}{YL} magnitudes, {BL}{}{YL} repeats:{UN}",
        algno,magnitudes,repeats );
    for mag in 1..magnitudes+1 {  
        let mut meds = Vec::with_capacity(algno);
        let mut stderrs = Vec::with_capacity(algno);
        let d = 10_usize.pow(mag as u32);      
        println!("\nDimensions: {BL}{}{UN}\n",d);
        let seed = get_seed(); // store the seed, whatever it is
        for closure in closures {
            // reintialise random numbers generator to the same seed for each closure
            set_seeds(seed);
            let mut times:Vec<f64> = Vec::with_capacity(repeats);    
            for _ in 0..repeats {
                let data = rn.ranvv(d,points).getvvf64(); // different for each repeat 
                timer.start();
                closure(&data);
                timer.stop();
                let this_time = timer.time_in_nanos().unwrap() as f64;
                times.push(this_time); 
            };
            let medmad = times.medstats();
            meds.push(medmad.centre);
            stderrs.push(100.0*medmad.dispersion/medmad.centre);
        } 
    report(names,&meds,&stderrs);
    }
}

/// Tests of listed `closures`, named in `names`,
/// on random data vectors of type specified by `rn` 
/// and lengths of increasing `magnitudes` in multiples of 10, e.g. 10,100,1000  
/// `repeats` runs of each closure for each magnitude 
pub fn benchvvu64(
    rn:Rnum,
    points: usize, // number of Vecs in each Vec<Vec<u8>>
    magnitudes:usize,
    repeats:usize,
    names:&[&str],
    closures:&[fn(&[Vec<u64>])]) { // concrete type here

    let algno = names.len();
    let mut timer = DevTime::new_simple();
    println!("\n{YL}Nanoseconds of {BL}{}{YL} algorithms, {BL}&[Vec<u64>]{YL} data, {BL}{}{YL} magnitudes, {BL}{}{YL} repeats:{UN}",
        algno,magnitudes,repeats );
    for mag in 1..magnitudes+1 {  
        let mut meds = Vec::with_capacity(algno);
        let mut stderrs = Vec::with_capacity(algno);
        let d = 10_usize.pow(mag as u32);      
        println!("\nDimensions: {BL}{}{UN}\n",d);
        let seed = get_seed(); // store the seed, whatever it is
        for closure in closures {
            // reintialise random numbers generator to the same seed for each closure
            set_seeds(seed);
            let mut times:Vec<f64> = Vec::with_capacity(repeats);    
            for _ in 0..repeats {
                let data = rn.ranvv(d,points).getvvu64(); // different for each repeat 
                timer.start();
                closure(&data);
                timer.stop();
                let this_time = timer.time_in_nanos().unwrap() as f64;
                times.push(this_time); 
            };
            let medmad = times.medstats();
            meds.push(medmad.centre);
            stderrs.push(100.0*medmad.dispersion/medmad.centre);
        } 
    report(names,&meds,&stderrs);
    }
}
