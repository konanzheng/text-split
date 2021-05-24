use std::{
    io::{BufRead, Write,BufReader,BufWriter, Seek,Result},
    fs::{File},
};
use std::time::{SystemTime};
use quicli::prelude::*;
use structopt::StructOpt;


fn split_file(o_name:String,count:u64) -> Result<()> {
    let name = o_name.trim();
    let file = File::open(name)?;
    let f_len = file.metadata()?.len();
    let mut f = BufReader::new(file);
    let split_size = f_len/count;
    let mut idx = 1;
    let mut file_name = String::from(name) + "-"+ &idx.to_string() + ".txt";
    let mut w = BufWriter::new(File::create(file_name)?);
    let mut before = f.stream_position()?;
    loop {
        let mut after = f.stream_position()?;
        if  after >= f_len {
            w.flush()?;
            break
        }
        let mut split_content = String::from("");
        f.read_line(&mut split_content)?;
        w.write(split_content.as_bytes())?;
        after = f.stream_position()?;
        let len = after - before;
        if  len >= split_size {
            w.flush()?;
            // println!("split files   {}  ,size {}, read position{} !",idx,len,after);
            idx = idx +1;
            file_name = String::from(name) + "-" +&idx.to_string() + ".txt";
            w = BufWriter::new(File::create(file_name)?);
            before = after;
        }
    }
    w.flush()?;
    Ok(())
}

#[derive(Debug, StructOpt)]
struct Cli {
    // Add a CLI argument `--count`/-n` that defaults to 3, and has this help text:
    /// How many lines to get
    #[structopt(long = "count", short = "n", default_value = "3")]
    count: u64,
    // Add a positional argument that the user has to supply:
    /// The file to read
    file: String,
    // Quick and easy logging setup you get for free with quicli
    #[structopt(flatten)]
    verbosity: Verbosity,
}

fn main() -> CliResult {
    let args = Cli::from_args();
    args.verbosity.setup_env_logger("head")?;
    // const  SPLIT_SIZE:u64= 5242880;
    // const  SPLIT_SIZE:u64= 10485760;
    let sy_time = SystemTime::now();
    let r = split_file(args.file,args.count)?;
    println!("end in {} seccends !",SystemTime::now().duration_since(sy_time).unwrap().as_secs());
    Ok(r)
}
