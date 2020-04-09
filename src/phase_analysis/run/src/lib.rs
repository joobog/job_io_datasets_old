extern crate csv;
extern crate serde;

use algorithm;

use std::error::Error;
//use glob::glob;
//use std::io;
//use std::env;
//use std::error::Error;
//use std::ffi::OsString;
use std::fs::File;
//use std::process;
//use std::path::Path;
use serde::Deserialize;
use serde::Serialize;

pub struct Config {
    dataset_fn: String,
    output_fn: String,
    take: usize,
    min_similarity: algorithm::SimType,
}

impl Config {
    pub fn new(dataset_fn: String, output_fn: String, take: usize, min_similarity: algorithm::SimType) -> Config {
        Config{
            dataset_fn,
            output_fn,
            take,
            min_similarity,
        }
    }
}


#[derive(Debug, Deserialize)]
pub struct Record {
   jobid: u32,
   coding: String,
}

pub fn convert_to_coding(coding: String) -> Vec<u16> {
    let split = coding.split(":");
    let vec: Vec<u16> = split
        .filter(|s| !s.is_empty())
        .map(|s| s.parse().unwrap()) 
        .collect();
    vec
}

#[derive(Debug, Serialize)]
pub struct OutputRow {
    pub jobid_1: u32,
    pub jobid_2: u32,
    pub num_phases_1: u8,
    pub num_phases_2: u8,
    pub sim: algorithm::SimType,
}

pub fn run(cfg: Config) -> Result<(), Box<dyn Error>> {
    let file = File::open(&cfg.dataset_fn).expect("Unable to open");
    let mut rdr = csv::Reader::from_reader(file);

    let mut phases_set: Vec<(u32, Vec<Vec<u16>>)> = Vec::new();

    for result in rdr.deserialize() {
        let record: Record = result.expect("bla bla");
        //println!("{:?}", record);
        let coding = convert_to_coding(record.coding);
        //println!("{:?}", coding);
        let phases = algorithm::detect_phases(coding);
        //println!("{:?}", phases);
        phases_set.push((record.jobid, phases));
    }

    let mut matrix: Vec<OutputRow> = Vec::new();
    let mut counter = 0;
    for p1 in phases_set.iter().take(cfg.take) {
        println!("{} {}", counter, p1.0);
        counter += 1;
        for p2 in phases_set.iter().take(cfg.take).skip(counter) {
            if (p1.1.len() > 0) && (p2.1.len() > 0) {
                let sim = algorithm::job_similarity(&p1.1, &p2.1);
                if sim > cfg.min_similarity {
                    matrix.push(OutputRow{
                        jobid_1: p1.0, 
                        jobid_2: p2.0,  
                        num_phases_1: p1.1.len() as u8, 
                        num_phases_2: p2.1.len() as u8,
                        sim: sim,
                    }); 
                    //println!("{}, {}, {}, {}, {}", p1.0, p2.0, sim, p1.1.len(), p2.1.len());
                }
            }
        }

    }

    
    //let file = File::open(&cfg.output_fn).expect("Unable to open");
    let file = File::create(&cfg.output_fn).expect("Unable to open");
    let mut wtr = csv::Writer::from_writer(file);
    for row in matrix.into_iter() {
        wtr.serialize(row)?;
    }
    wtr.flush()?;
    //println!("{:?}", matrix);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_convert_to_coding() {
        let coding = String::from("256:256:0:0:38");
        let c = convert_to_coding(coding);
        let expected_c: Vec<u16> = vec![256, 256, 0, 0, 38];
        assert_eq!(expected_c, c);
    }
}


//#[derive(Debug, Deserialize)]
//pub struct Record {
//    jobid: u32,
//    name: String,
//    host: String,
//    metric: String,
//    idx: u32,
//    value: f32,
//    duration: f32,
//    start: u32,
//    end: u32,
//    runtime: f32,
//    nvalue: f32,
//    cat: f32,
//}

//pub fn run(cfg: Config) -> Result<(), Box<dyn Error>> {
//    let file_names = glob(&cfg.wildcard).expect("Failed to read glob pattern");

//    for entry in file_names {
//        let path = entry.unwrap();
//        //let path = match entry {
//        //    Ok(path) => path,
//        //    Err(error) => panic!("{:?}", error),
//        //};

//        println!("{:?}", path.display());
//        let file = File::open(path).expect("Unable to open");
//        let mut rdr = csv::Reader::from_reader(file);


//        for result in rdr.deserialize() {
//           let record: Record = result.expect("bla bla");
//           println!("{:?}", record);
//        }
//    }

//    Ok(())
//}
