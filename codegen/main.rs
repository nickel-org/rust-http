#![feature(collections,core,env,io,os,path,std_misc)]
use std::env;
use std::old_io::{File, Truncate, Write};
use std::thread::Thread;

pub mod branchify;
pub mod status;
pub mod read_method;

fn main() {
    Thread::spawn(move || {
        let out = env::var_os("OUT_DIR").unwrap().into_string().unwrap();
        let output_dir = Path::new(out);
        read_method::generate(output_dir).unwrap();
    });

    let out = env::var_os("OUT_DIR").unwrap().into_string().unwrap();
    let output_dir = Path::new(out);
    status::generate(output_dir).unwrap();
}

pub fn get_writer(mut output_dir: Path, filename: &str) -> Box<Writer + 'static> {
    output_dir.push(filename);
    match File::open_mode(&output_dir, Truncate, Write) {
        Ok(writer) => Box::new(writer),
        Err(e) => panic!("Unable to write file: {}", e.desc),
    }
}
