#![warn(missing_docs)]
//! Benchmark for timing algorithms
use core::ops::Range;
use devtimer::DevTime;
use indxvec::{printing::*, Indices, Vecops};
use medians::{Medianf64};
use ran::{generators::get_seed, *};

fn report(names: &[&str], meds: &[f64], stderrs: &[f64]) {
    let medsx = meds.mergesort_indexed();
    let meds_sorted = medsx.unindex(meds, true);
    let names_sorted = medsx.unindex(names, true);
    let stderrs_sorted = medsx.unindex(stderrs, true);
    for i in 0..names.len() {
        println!(
            "{MG}{:<18}{GR}{:>13.0} ± {:>7.0}{UN}",
            names_sorted[i], meds_sorted[i], stderrs_sorted[i]
        );
    }
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
    let mut timer = DevTime::new_simple();
    heading("none",1,1,0,0,repeats);
    let mut meds = Vec::with_capacity(algno);
    let mut stderrs = Vec::with_capacity(algno);
    let seed = get_seed(); // store the seed, whatever it is
    for closure in closures {
        // reintialise random numbers generator to the same seed for each closure
        set_seeds(seed);
        let mut times: Vec<f64> = Vec::with_capacity(repeats);
        for _ in 0..repeats {
            timer.start();
            closure();
            timer.stop();
            let this_time = timer.time_in_nanos().unwrap() as f64;
            times.push(this_time);
        }
        let medmad = times.medstats().expect("bench mestats");
        meds.push(medmad.centre);
        stderrs.push(medmad.dispersion);
    }
    report(names, &meds, &stderrs);
}

/// Tests of listed `closures`, named in `names`,
/// on random data vectors of type specified by `rn`
/// and lengths of increasing `magnitudes` in multiples of 10, e.g. 10,100,1000  
/// `repeats` runs of each closure for each magnitude
pub fn mutbenchu8(
    rn: Rnum,
    lengths: Range<usize>,
    step: usize,
    repeats: usize,
    names: &[&str],
    closures: &[fn(&mut [u8])],
) {
    let algno = names.len();
    let mut timer = DevTime::new_simple();
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
                let mut data = rn.ranv(d).unwrap().getvu8().unwrap(); // different for each repeat
                timer.start();
                closure(&mut data);
                timer.stop();
                let this_time = timer.time_in_nanos().unwrap() as f64;
                times.push(this_time);
            }
            let medmad = times
                .medstats()
                .expect("mutbenchu8 medstats");
            meds.push(medmad.centre);
            stderrs.push(medmad.dispersion);
        }
        report(names, &meds, &stderrs);
    }
}

/// Tests of listed `closures`, named in `names`,
/// on random data vectors of type specified by `rn`
/// and lengths of increasing `magnitudes` in multiples of 10, e.g. 10,100,1000  
/// `repeats` runs of each closure for each magnitude
pub fn mutbenchu16(
    rn: Rnum,
    lengths: Range<usize>,
    step: usize,
    repeats: usize,
    names: &[&str],
    closures: &[fn(&mut [u16])],
) {
    let algno = names.len();
    let mut timer = DevTime::new_simple();
    heading("&mut[u16]",lengths.start,lengths.end,step,1,repeats); 
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
                let mut data = rn.ranv(d).unwrap().getvu16().unwrap(); // different for each repeat
                timer.start();
                closure(&mut data);
                timer.stop();
                let this_time = timer.time_in_nanos().unwrap() as f64;
                times.push(this_time);
            }
            let medmad = times
                .medstats()
                .expect("mutbenchu8 medstats");
            meds.push(medmad.centre);
            stderrs.push(medmad.dispersion);
        }
        report(names, &meds, &stderrs);
    }
}

/// Tests of listed `closures`, named in `names`,
/// on random data vectors of type specified by `rn`.  
/// Lengths range is iterated over by step.  
/// `repeats` runs of each closure for each length of data
pub fn mutbenchu64(
    rn: Rnum,
    lengths: Range<usize>,
    step: usize,
    repeats: usize,
    names: &[&str],
    closures: &[fn(&mut [u64])],
) {
    let algno = names.len();
    let mut timer = DevTime::new_simple();
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
                let mut data = rn.ranv(d).unwrap().getvu64().unwrap(); // different for each repeat
                timer.start();
                closure(&mut data);
                timer.stop();
                let this_time = timer.time_in_nanos().unwrap() as f64;
                times.push(this_time);
            }
            let medmad = times
                .medstats()
                .expect("mutbenchu64 medstats");
            meds.push(medmad.centre);
            stderrs.push(medmad.dispersion);
        }
        report(names, &meds, &stderrs);
    }
}

/// Tests of listed `closures`, named in `names`,
/// on random data vectors of type specified by `rn`
/// and lengths of increasing `magnitudes` in multiples of 10, e.g. 10,100,1000  
/// `repeats` runs of each closure for each magnitude
pub fn mutbenchf64(
    rn: Rnum,
    lengths: Range<usize>,
    step: usize,
    repeats: usize,
    names: &[&str],
    closures: &[fn(&mut [f64])],
) {
    let algno = names.len();
    let mut timer = DevTime::new_simple();
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
                let mut data = rn.ranv(d).unwrap().getvf64().unwrap(); // different for each repeat
                timer.start();
                closure(&mut data);
                timer.stop();
                let this_time = timer.time_in_nanos().unwrap() as f64;
                times.push(this_time);
            }
            let medmad = times
                .medstats()
                .expect("mutbenchf64 medstats");
            meds.push(medmad.centre);
            stderrs.push(medmad.dispersion);
        }
        report(names, &meds, &stderrs);
    }
}

/// Tests of listed `closures`, named in `names`,
/// on random data vectors of type specified by `rn`
/// and lengths of increasing `magnitudes` in multiples of 10, e.g. 10,100,1000  
/// `repeats` runs of each closure for each magnitude
pub fn benchu8(
    rn: Rnum,
    lengths: Range<usize>,
    step: usize,
    repeats: usize,
    names: &[&str],
    closures: &[fn(&[u8])],
) {
    let algno = names.len();
    let mut timer = DevTime::new_simple();
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
                let data = rn.ranv(d).unwrap().getvu8().unwrap(); // different for each repeat
                timer.start();
                closure(&data);
                timer.stop();
                let this_time = timer.time_in_nanos().unwrap() as f64;
                times.push(this_time);
            }
            let medmad = times.medstats().expect("benchu8 medstats");
            meds.push(medmad.centre);
            stderrs.push(medmad.dispersion);
        }
        report(names, &meds, &stderrs);
    }
}

/// Tests of listed `closures`, named in `names`,
/// on random data vectors of type specified by `rn`
/// and lengths of increasing `magnitudes` in multiples of 10, e.g. 10,100,1000  
/// `repeats` runs of each closure for each magnitude
pub fn benchu16(
    rn: Rnum,
    lengths: Range<usize>,
    step: usize,
    repeats: usize,
    names: &[&str],
    closures: &[fn(&[u16])],
) {
    let algno = names.len();
    let mut timer = DevTime::new_simple();
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
                let data = rn.ranv(d).unwrap().getvu16().unwrap(); // different for each repeat
                timer.start();
                closure(&data);
                timer.stop();
                let this_time = timer.time_in_nanos().unwrap() as f64;
                times.push(this_time);
            }
            let medmad = times.medstats().expect("benchu8 medstats");
            meds.push(medmad.centre);
            stderrs.push(medmad.dispersion);
        }
        report(names, &meds, &stderrs);
    }
}

/// Tests of listed `closures`, named in `names`,
/// on random data vectors of type specified by `rn`
/// and lengths of increasing `magnitudes` in multiples of 10, e.g. 10,100,1000  
/// `repeats` runs of each closure for each magnitude
pub fn benchu64(
    rn: Rnum,
    lengths: Range<usize>,
    step: usize,
    repeats: usize,
    names: &[&str],
    closures: &[fn(&[u64])],
) {
    let algno = names.len();
    let mut timer = DevTime::new_simple();
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
                let data = rn.ranv(d).unwrap().getvu64().unwrap(); // different for each repeat
                timer.start();
                closure(&data);
                timer.stop();
                let this_time = timer.time_in_nanos().unwrap() as f64;
                times.push(this_time);
            }
            let medmad = times
                .medstats()
                .expect("benchu64 medstats");
            meds.push(medmad.centre);
            stderrs.push(medmad.dispersion);
        }
        report(names, &meds, &stderrs);
    }
}

/// Tests of listed `closures`, named in `names`,
/// on random data vectors of type specified by `rn`
/// and lengths of increasing `magnitudes` in multiples of 10, e.g. 10,100,1000  
/// `repeats` runs of each closure for each magnitude
pub fn benchf64(
    rn: Rnum,
    lengths: Range<usize>,
    step: usize,
    repeats: usize,
    names: &[&str],
    closures: &[fn(&[f64])],
) {
    let algno = names.len();
    let mut timer = DevTime::new_simple();
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
                let data = rn.ranv(d).unwrap().getvf64().unwrap(); // different for each repeat
                timer.start();
                closure(&data);
                timer.stop();
                let this_time = timer.time_in_nanos().unwrap() as f64;
                times.push(this_time);
            }
            let medmad = times
                .medstats()
                .expect("benchf64 medstats");
            meds.push(medmad.centre);
            stderrs.push(medmad.dispersion);
        }
        report(names, &meds, &stderrs);
    }
}

/// Tests of listed `closures`, named in `names`,
/// on random data vectors of type specified by `rn`
/// and lengths of increasing `magnitudes` in multiples of 10, e.g. 10,100,1000  
/// `repeats` runs of each closure for each magnitude
pub fn benchvvu8(
    rn: Rnum,
    points: usize, // number of Vecs in each Vec<Vec<u8>>
    lengths: Range<usize>,
    step: usize,
    repeats: usize,
    names: &[&str],
    closures: &[fn(&[Vec<u8>])],
) {
    let algno = names.len();
    let mut timer = DevTime::new_simple();
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
                let data = rn.ranvv(d, points).unwrap().getvvu8().unwrap(); // different for each repeat
                timer.start();
                closure(&data);
                timer.stop();
                let this_time = timer.time_in_nanos().unwrap() as f64;
                times.push(this_time);
            }
            let medmad = times
                .medstats()
                .expect("benchvvu8 medstats");
            meds.push(medmad.centre);
            stderrs.push(medmad.dispersion);
        }
        report(names, &meds, &stderrs);
    }
}

/// Tests of listed `closures`, named in `names`,
/// on random data vectors of type specified by `rn`
/// and lengths of increasing `magnitudes` in multiples of 10, e.g. 10,100,1000  
/// `repeats` runs of each closure for each magnitude
pub fn benchvvu16(
    rn: Rnum,
    points: usize, // number of Vecs in each Vec<Vec<u8>>
    lengths: Range<usize>,
    step: usize,
    repeats: usize,
    names: &[&str],
    closures: &[fn(&[Vec<u16>])],
) {
    let algno = names.len();
    let mut timer = DevTime::new_simple();
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
                let data = rn.ranvv(d, points).unwrap().getvvu16().unwrap(); // different for each repeat
                timer.start();
                closure(&data);
                timer.stop();
                let this_time = timer.time_in_nanos().unwrap() as f64;
                times.push(this_time);
            }
            let medmad = times
                .medstats()
                .expect("benchvvu8 medstats");
            meds.push(medmad.centre);
            stderrs.push(medmad.dispersion);
        }
        report(names, &meds, &stderrs);
    }
}

/// Tests of listed `closures`, named in `names`,
/// on random data vectors of type specified by `rn`
/// and lengths of increasing `magnitudes` in multiples of 10, e.g. 10,100,1000  
/// `repeats` runs of each closure for each magnitude
pub fn benchvvf64(
    rn: Rnum,
    points: usize, // number of Vecs in each Vec<Vec<u8>>
    lengths: Range<usize>,
    step: usize,
    repeats: usize,
    names: &[&str],
    closures: &[fn(&[Vec<f64>])],
) {
    let algno = names.len();
    let mut timer = DevTime::new_simple();
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
                let data = rn.ranvv(d, points).unwrap().getvvf64().unwrap(); // different for each repeat
                timer.start();
                closure(&data);
                timer.stop();
                let this_time = timer.time_in_nanos().unwrap() as f64;
                times.push(this_time);
            }
            let medmad = times
                .medstats()
                .expect("benchvvf64 medstats");
            meds.push(medmad.centre);
            stderrs.push(medmad.dispersion);
        }
        report(names, &meds, &stderrs);
    }
}

/// Tests of listed `closures`, named in `names`,
/// on random data vectors of type specified by `rn`
/// and lengths of increasing `magnitudes` in multiples of 10, e.g. 10,100,1000  
/// `repeats` runs of each closure for each magnitude
pub fn benchvvu64(
    rn: Rnum,
    points: usize, // number of Vecs in each Vec<Vec<u64>>
    lengths: Range<usize>,
    step: usize,
    repeats: usize,
    names: &[&str],
    closures: &[fn(&[Vec<u64>])],
) {
    let algno = names.len();
    let mut timer = DevTime::new_simple();
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
                let data = rn.ranvv(d, points).unwrap().getvvu64().unwrap(); // different for each repeat
                timer.start();
                closure(&data);
                timer.stop();
                let this_time = timer.time_in_nanos().unwrap() as f64;
                times.push(this_time);
            }
            let medmad = times
                .medstats()
                .expect("benchvvu64 medstats");
            meds.push(medmad.centre);
            stderrs.push(medmad.dispersion);
        }
        report(names, &meds, &stderrs);
    }
}
