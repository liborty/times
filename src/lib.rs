#![warn(missing_docs)]
//! Benchmark for timing algorithms

use std::time::Instant;
use core::ops::Range;
use indxvec::{printing::*, Indices, Vecops};
use medians::Medianf64;
use ran::*;

fn report(names: &[&str], meds: &[f64], stderrs: &[f64]) {
    let medsx = meds.isort_indexed(0..meds.len(),|a:&f64,b| a.total_cmp(b));
    let meds_sorted = medsx.unindex(meds, true);
    let names_sorted = medsx.unindex(names, true);
    let stderrs_sorted = medsx.unindex(stderrs, true);
    for i in 0..names.len() {
        println!(
            "{YL}{:<18}{GR}{:>13.0} ±{:>7.0} ~{:>5.2}%{YL} {:>7.4}{UN}",
            names_sorted[i],
            meds_sorted[i],
            stderrs_sorted[i],
            100.0*stderrs_sorted[i]/meds_sorted[i],
            meds_sorted[i]/meds_sorted[0]);
    };
}
fn heading(data:&str,c1:usize,c2:usize,step:usize,rows:usize,repeats:usize) {
    println!(
        "\n{YL}Data:{GR}{data} {YL}lengths:{GR}{c1}-{c2} {YL}step:{GR}{step} {YL}rows:{GR}{rows} {YL}repeats:{GR}{repeats}{UN}"
    );
}

/// Tests of listed `closures` that take no or constant arguments, named in `names`
/// `repeats` runs of each closure.
pub fn bench(repeats: usize, names: &[&str], closures: &[fn()]) {
    let algno = names.len(); 
    println!(
        "\n{YL}Input Data: {GR}none {YL}repeats: {GR}{repeats}{UN}"
    );
    let mut meds = Vec::with_capacity(algno);
    let mut stderrs = Vec::with_capacity(algno);
    let seed = get_seed(); // store the seed, whatever it is
    for closure in closures {
        // reintialise random numbers generator to the same seed for each closure
        set_seeds(seed);
        let mut times: Vec<f64> = Vec::with_capacity(repeats);
        for _ in 0..repeats {
            let now = Instant::now(); // = UNIX_EPOCH.elapsed().unwrap().as_nanos() as u64;timer.start();
            closure();
            times.push(now.elapsed().as_nanos() as f64);
        };
        let med = times.medf_checked().expect("bench Nan detected");
        meds.push(med);
        stderrs.push(times.madf(med));
    }
    report(names, &meds, &stderrs);
}

/// Tests of listed `closures`, named in `names`,
/// on random data vectors of type specified by `rn`
/// and lengths of increasing `magnitudes` in multiples of 10, e.g. 10,100,1000  
/// `repeats` runs of each closure for each magnitude
pub fn mutbenchu8(
    lengths: Range<usize>,
    step: usize,
    repeats: usize,
    names: &[&str],
    closures: &[fn(&mut [u8])],
) {
    let algno = names.len(); 
    heading("&mut[u8]",lengths.start,lengths.end,step,1,repeats); 
    for d in lengths.step_by(step) {
        let mut meds = Vec::with_capacity(algno);
        let mut stderrs = Vec::with_capacity(algno);
        println!("\nLength: {BL}{}{UN}\n", d);
        let seed = get_seed(); // store the seed, whatever it is
        for closure in closures {
            // reintialise random numbers generator to the same seed for each closure
            set_seeds(seed);
            let mut times: Vec<f64> = Vec::with_capacity(repeats);
            for _ in 0..repeats {
                let mut data = ranv_u8(d).expect("ranv_u8 failed"); // different for each repeat
                let now = Instant::now();
                closure(&mut data);
                times.push(now.elapsed().as_nanos() as f64);
            };
            let med = times.medf_checked().expect("mutbenchu8 Nan detected");
            meds.push(med);
            stderrs.push(times.madf(med));
        }
        report(names, &meds, &stderrs);
    }
}

/// Tests of listed `closures`, named in `names`,
/// on random data vectors of type specified by `rn`
/// and lengths of increasing `magnitudes` in multiples of 10, e.g. 10,100,1000  
/// `repeats` runs of each closure for each magnitude
pub fn mutbenchu16(
    lengths: Range<usize>,
    step: usize,
    repeats: usize,
    names: &[&str],
    closures: &[fn(&mut [u16])],
) {
    let algno = names.len();
    heading("&mut[u16]",lengths.start,lengths.end,step,1,repeats); 
    for d in lengths.step_by(step) {
        let mut meds = Vec::with_capacity(algno);
        let mut stderrs = Vec::with_capacity(algno);
        println!("\nLength: {BL}{}{UN}\n", d);
        let seed = get_seed(); // store the seed, whatever it is
        for closure in closures {
            let mut times: Vec<f64> = Vec::with_capacity(repeats);
            // reintialise random numbers generator to the same seed for each closure
            set_seeds(seed);
            for _ in 0..repeats {
                let mut data = ranv_u16(d).expect("ranv_u16 failed"); // different for each repeat
                let now = Instant::now();
                closure(&mut data);
                times.push(now.elapsed().as_nanos() as f64);
            };
            let med = times.medf_checked().expect("mutbenchu16 Nan detected");
            meds.push(med);
            stderrs.push(times.madf(med));
        }
        report(names, &meds, &stderrs);
    }
}

/// Tests of listed `closures`, named in `names`,
/// on random data vectors of type specified by `rn`.  
/// Lengths range is iterated over by step.  
/// `repeats` runs of each closure for each length of data
pub fn mutbenchu64(
    lengths: Range<usize>,
    step: usize,
    repeats: usize,
    names: &[&str],
    closures: &[fn(&mut [u64])],
) {
    let algno = names.len();
    heading("&mut[u64]",lengths.start,lengths.end,step,1,repeats); 
    for d in lengths.step_by(step) {
        let mut meds = Vec::with_capacity(algno);
        let mut stderrs = Vec::with_capacity(algno);
        println!("\nLength: {BL}{}{UN}\n", d);
        let seed = get_seed(); // store the seed, whatever it is
        for closure in closures {
            // reintialise random numbers generator to the same seed for each closure
            set_seeds(seed);
            let mut times: Vec<f64> = Vec::with_capacity(repeats);
            for _ in 0..repeats {
                let mut data = ranv_u64(d).expect("ranv_u64 failed"); // different for each repeat
                let now = Instant::now();
                closure(&mut data);
                times.push(now.elapsed().as_nanos() as f64);
            };
            let med = times.medf_checked().expect("mutbenchu64 Nan detected");
            meds.push(med);
            stderrs.push(times.madf(med));
        }
        report(names, &meds, &stderrs);
    }
}

/// Tests of listed `closures`, named in `names`,
/// on random data vectors of type specified by `rn`
/// and lengths of increasing `magnitudes` in multiples of 10, e.g. 10,100,1000  
/// `repeats` runs of each closure for each magnitude
pub fn mutbenchf64( 
    lengths: Range<usize>,
    step: usize,
    repeats: usize,
    names: &[&str],
    closures: &[fn(&mut [f64])],
) {
    let algno = names.len();
    heading("&mut[f64]",lengths.start,lengths.end,step,1,repeats); 
    for d in lengths.step_by(step) {
        let mut meds = Vec::with_capacity(algno);
        let mut stderrs = Vec::with_capacity(algno);
        println!("\nLength: {BL}{}{UN}\n", d);
        let seed = get_seed(); // store the seed, whatever it is
        for closure in closures {
            // reintialise random numbers generator to the same seed for each closure
            set_seeds(seed);
            let mut times: Vec<f64> = Vec::with_capacity(repeats);
            for _ in 0..repeats {
                let mut data = ranv_f64(d).expect("ranv_f64 failed"); // different for each repeat
                let now = Instant::now();
                closure(&mut data);
                times.push(now.elapsed().as_nanos() as f64);
            };
            let med = times.medf_checked().expect("mutbenchf64 Nan detected");
            meds.push(med);
            stderrs.push(times.madf(med));
        }
        report(names, &meds, &stderrs);
    }
}

/// Tests of listed `closures`, named in `names`,
/// on random data vectors of type specified by `rn`
/// and lengths of increasing `magnitudes` in multiples of 10, e.g. 10,100,1000  
/// `repeats` runs of each closure for each magnitude
pub fn benchu8(
    lengths: Range<usize>,
    step: usize,
    repeats: usize,
    names: &[&str],
    closures: &[fn(&[u8])],
) {
    let algno = names.len();
    heading("&[u8]",lengths.start,lengths.end,step,1,repeats); 
    for d in lengths.step_by(step) {
        let mut meds = Vec::with_capacity(algno);
        let mut stderrs = Vec::with_capacity(algno);
        println!("\nLength: {BL}{}{UN}\n", d);
        let seed = get_seed(); // store the seed, whatever it is
        for closure in closures {
            // reintialise random numbers generator to the same seed for each closure
            set_seeds(seed);
            let mut times: Vec<f64> = Vec::with_capacity(repeats);
            for _ in 0..repeats {
                let data = ranv_u8(d).expect("ranv_u8 failed"); // different for each repeat
                let now = Instant::now();
                closure(&data);
                times.push(now.elapsed().as_nanos() as f64); 
            };
            let med = times.medf_checked().expect("benchu8 Nan detected");
            meds.push(med);
            stderrs.push(times.madf(med)); 
        }
        report(names, &meds, &stderrs);
    }
}

/// Tests of listed `closures`, named in `names`,
/// on random data vectors of type specified by `rn`
/// and lengths of increasing `magnitudes` in multiples of 10, e.g. 10,100,1000  
/// `repeats` runs of each closure for each magnitude
pub fn benchu16( 
    lengths: Range<usize>,
    step: usize,
    repeats: usize,
    names: &[&str],
    closures: &[fn(&[u16])],
) {
    let algno = names.len();
    heading("&[u16]",lengths.start,lengths.end,step,1,repeats); 
    for d in lengths.step_by(step) {
        let mut meds = Vec::with_capacity(algno);
        let mut stderrs = Vec::with_capacity(algno);
        println!("\nLength: {BL}{}{UN}\n", d);
        let seed = get_seed(); // store the seed, whatever it is
        for closure in closures {
            // reintialise random numbers generator to the same seed for each closure
            set_seeds(seed);
            let mut times: Vec<f64> = Vec::with_capacity(repeats);
            for _ in 0..repeats {
                let data = ranv_u16(d).expect("ranv_u16 failed"); // different for each repeat
                let now = Instant::now();
                closure(&data);
                times.push(now.elapsed().as_nanos() as f64);
            };
            let med = times.medf_checked().expect("benchu16 Nan detected");
            meds.push(med);
            stderrs.push(times.madf(med));
        }
        report(names, &meds, &stderrs);
    }
}

/// Tests of listed `closures`, named in `names`,
/// on random data vectors of type specified by `rn`
/// and lengths of increasing `magnitudes` in multiples of 10, e.g. 10,100,1000  
/// `repeats` runs of each closure for each magnitude
pub fn benchu64(
    lengths: Range<usize>,
    step: usize,
    repeats: usize,
    names: &[&str],
    closures: &[fn(&[u64])],
) {
    let algno = names.len();
    heading("&[u64]",lengths.start,lengths.end,step,1,repeats);
    for d in lengths.step_by(step) {
        let mut meds = Vec::with_capacity(algno);
        let mut stderrs = Vec::with_capacity(algno);
        println!("\nLength: {BL}{}{UN}\n", d);
        let seed = get_seed(); // store the seed, whatever it is
        for closure in closures {
            // reintialise random numbers generator to the same seed for each closure
            set_seeds(seed);
            let mut times: Vec<f64> = Vec::with_capacity(repeats);
            for _ in 0..repeats {
                let data = ranv_u64(d).expect("ranv_u64 failed"); // different for each repeat
                let now = Instant::now();
                closure(&data);
                times.push(now.elapsed().as_nanos() as f64);
            };
            let med = times.medf_checked().expect("benchu64 Nan detected");
            meds.push(med);
            stderrs.push(times.madf(med)); 
        };
        report(names, &meds, &stderrs);
    }
}

/// Tests of listed `closures`, named in `names`,
/// on random data vectors of type specified by `rn`
/// and lengths of increasing `magnitudes` in multiples of 10, e.g. 10,100,1000  
/// `repeats` runs of each closure for each magnitude
pub fn benchf64(
    lengths: Range<usize>,
    step: usize,
    repeats: usize,
    names: &[&str],
    closures: &[fn(&[f64])],
) {
    let algno = names.len();
    heading("&[f64]",lengths.start,lengths.end,step,1,repeats);  
    for d in lengths.step_by(step) {
        let mut meds = Vec::with_capacity(algno);
        let mut stderrs = Vec::with_capacity(algno);
        println!("\nLength: {BL}{}{UN}\n", d);
        let seed = get_seed(); // store the seed, whatever it is
        for closure in closures {
            // reintialise random numbers generator to the same seed for each closure
            set_seeds(seed);
            let mut times: Vec<f64> = Vec::with_capacity(repeats);
            for _ in 0..repeats {
                let data = ranv_f64(d).expect("ranv_f64 failed"); // different for each repeat
                let now = Instant::now();
                closure(&data);
                times.push(now.elapsed().as_nanos() as f64);
            };
            let med = times.medf_checked().expect("benchf64 Nan detected");
            meds.push(med);
            stderrs.push(times.madf(med));
        };
        report(names, &meds, &stderrs);
    }
}

/// Tests of listed `closures`, named in `names`,
/// on random data vectors of type specified by `rn`
/// and lengths of increasing `magnitudes` in multiples of 10, e.g. 10,100,1000  
/// `repeats` runs of each closure for each magnitude
pub fn benchvvu8(
    points: usize, // number of Vecs in each Vec<Vec<u8>>
    lengths: Range<usize>,
    step: usize,
    repeats: usize,
    names: &[&str],
    closures: &[fn(&[Vec<u8>])],
) {
    let algno = names.len();
    heading("&[Vec<u8>]",lengths.start,lengths.end,step,points,repeats); 
    for d in lengths.step_by(step) {
        let mut meds = Vec::with_capacity(algno);
        let mut stderrs = Vec::with_capacity(algno);
        println!("\nLength: {BL}{}{UN}\n", d);
        let seed = get_seed(); // store the seed, whatever it is
        for closure in closures {
            // reintialise random numbers generator to the same seed for each closure
            set_seeds(seed);
            let mut times: Vec<f64> = Vec::with_capacity(repeats);
            for _ in 0..repeats {
                let data = ranvv_u8(points,d).expect("ranvv_u8 failed"); // different for each repeat
                let now = Instant::now();
                closure(&data);
                times.push(now.elapsed().as_nanos() as f64);
            };
            let med = times.medf_checked().expect("benchvvu8 Nan detected");
            meds.push(med);
            stderrs.push(times.madf(med));
        };
        report(names, &meds, &stderrs);
    }
}

/// Tests of listed `closures`, named in `names`,
/// on random data vectors of type specified by `rn`
/// and lengths of increasing `magnitudes` in multiples of 10, e.g. 10,100,1000  
/// `repeats` runs of each closure for each magnitude
pub fn benchvvu16(
    points: usize, // number of Vecs in each Vec<Vec<u8>>
    lengths: Range<usize>,
    step: usize,
    repeats: usize,
    names: &[&str],
    closures: &[fn(&[Vec<u16>])],
) {
    let algno = names.len();
    heading("&[Vec<u8>]",lengths.start,lengths.end,step,points,repeats); 
    for d in lengths.step_by(step) {
        let mut meds = Vec::with_capacity(algno);
        let mut stderrs = Vec::with_capacity(algno);
        println!("\nLength: {BL}{}{UN}\n", d);
        let seed = get_seed(); // store the seed, whatever it is
        for closure in closures {
            // reintialise random numbers generator to the same seed for each closure
            set_seeds(seed);
            let mut times: Vec<f64> = Vec::with_capacity(repeats);
            for _ in 0..repeats {
                let data = ranvv_u16(points,d).expect("ranvv_u16 failed"); // different for each repeat
                let now = Instant::now();
                closure(&data);
                times.push(now.elapsed().as_nanos() as f64);
            };
            let med = times.medf_checked().expect("benchvvu16 Nan detected");
            meds.push(med);
            stderrs.push(times.madf(med));
        };
        report(names, &meds, &stderrs);
    }
}

/// Tests of listed `closures`, named in `names`,
/// on random data vectors of type specified by `rn`
/// and lengths of increasing `magnitudes` in multiples of 10, e.g. 10,100,1000  
/// `repeats` runs of each closure for each magnitude
pub fn benchvvf64( 
    points: usize, // number of Vecs in each Vec<Vec<u8>>
    lengths: Range<usize>,
    step: usize,
    repeats: usize,
    names: &[&str],
    closures: &[fn(&[Vec<f64>])],
) {
    let algno = names.len();
    heading("&[Vec<f64>]",lengths.start,lengths.end,step,points,repeats);  
    for d in lengths.step_by(step) {
        let mut meds = Vec::with_capacity(algno);
        let mut stderrs = Vec::with_capacity(algno);
        println!("\nLength: {BL}{}{UN}\n", d);
        let seed = get_seed(); // store the seed, whatever it is
        for closure in closures {
            // reintialise random numbers generator to the same seed for each closure
            set_seeds(seed);
            let mut times: Vec<f64> = Vec::with_capacity(repeats);
            for _ in 0..repeats {
                let data = ranvv_f64(points,d).expect("ranvv_f64 failed"); // different for each repeat
                let now = Instant::now();
                closure(&data);
                times.push(now.elapsed().as_nanos() as f64);
            };
            let med = times.medf_checked().expect("benchvvf64 Nan detected");
            meds.push(med);
            stderrs.push(times.madf(med));
        };
        report(names, &meds, &stderrs);
    }
}

/// Tests of listed `closures`, named in `names`,
/// on random data vectors of type specified by `rn`
/// and lengths of increasing `magnitudes` in multiples of 10, e.g. 10,100,1000  
/// `repeats` runs of each closure for each magnitude
pub fn benchvvu64( 
    points: usize, // number of Vecs in each Vec<Vec<u64>>
    lengths: Range<usize>,
    step: usize,
    repeats: usize,
    names: &[&str],
    closures: &[fn(&[Vec<u64>])],
) {
    let algno = names.len();
    heading("&[Vec<u64>]",lengths.start,lengths.end,step,points,repeats); 
    for d in lengths.step_by(step) {
        let mut meds = Vec::with_capacity(algno);
        let mut stderrs = Vec::with_capacity(algno);
        println!("\nLength: {BL}{}{UN}\n", d);
        let seed = get_seed(); // store the seed, whatever it is
        for closure in closures {
            // reintialise random numbers generator to the same seed for each closure
            set_seeds(seed);
            let mut times: Vec<f64> = Vec::with_capacity(repeats);
            for _ in 0..repeats {
                let data = ranvv_u64(points,d).expect("ranvv_u64 failed"); // different for each repeat
                let now = Instant::now();
                closure(&data);
                times.push(now.elapsed().as_nanos() as f64);
            };
            let med = times.medf_checked().expect("benchvvu64 Nan detected");
            meds.push(med);
            stderrs.push(times.madf(med));
        };
        report(names, &meds, &stderrs);
    }
}
