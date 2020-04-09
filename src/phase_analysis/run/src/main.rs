use std::process;
use algorithm;

fn main() {
    //let root = String::from("/work/ku0598/k202107/git/mistral-job-evaluation/data/eval_20200117/");
    let root = String::from("/home/joobog/dkrz/git/job_io_datasets");
    let dset_fn = String::from("coding_job_abs_mode_False.csv");
    //let dset_fn = String::from("coding_metric_hex_False.csv");
    let dset_fn = format!("{}/{}", root, dset_fn);
    let output_fn = format!("{}/coding_job_abs_mode_False_similarity.csv", root);

    let cfg = run::Config::new(
        dset_fn,
        output_fn,
        100000,
        0.7,
        );

    if let Err(e) = run::run(cfg) {
       eprintln!("Error occured in run: {}", e);
       process::exit(1);
    }
}
