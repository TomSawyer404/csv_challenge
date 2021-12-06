mod core;
mod err;
mod opt;

use self::core::{
    read::{load_csv, write_csv},
    write::replace_column,
};
use self::opt::Opt;
use std::path::PathBuf;
use std::process;
use structopt::StructOpt;

fn main() {
    let opt = Opt::from_args();
    let filename = PathBuf::from(opt.input);

    let csv_data = match load_csv(filename) {
        Ok(fname) => fname,
        Err(e) => {
            eprintln!("main error: {:?}", e);
            process::exit(1);
        }
    };

    let modified_data = match replace_column(csv_data, &opt.column_name, &opt.replacement) {
        Ok(data) => data,
        Err(e) => {
            eprintln!("main error: {:?}", e);
            process::exit(1);
        }
    };

    let output_file = &opt.output.unwrap_or("output/output.csv".to_string());
    match write_csv(&modified_data, &output_file) {
        Ok(_) => {
            println!("write success!");
        }
        Err(e) => {
            eprintln!("main error: {:?}", e);
            process::exit(1);
        }
    }
}
