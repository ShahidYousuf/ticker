extern crate chrono;
use std::time::Duration;
use std::fs;
use std::thread;
use std::env;
use std::process;
//use std::error::Error;
use std::io;
//use std::fs::DirEntry;
use std::path::Path;
use chrono::prelude::*;

fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("problem parsing arguments: {}", err);
        process::exit(1);
    });
    let local: DateTime<Local> = Local::now();
    println!("ticker started at {}\nWatching for file changes inside {:?} with frequency: {}\n",local.format("%d %B, %Y at %r"), config.path, config.freq);
    loop {
        // let flag = watcher(&Path::new(&config.path)).unwrap_or_else(|err| {
        //     eprintln!("problem reading file info {}", err);
        //     process::exit(1);
        // });
        // if flag == true {
        //     println!("File {:?} was modified.", &config.path);
        // }

        visit_dirs(&Path::new(&config.path),config.freq, &watcher).unwrap_or_else(|err| {
            eprintln!("problem fetching information: {}", err);
            process::exit(1);
        });
        thread::sleep(Duration::from_secs(config.freq.into()));
    }
}

struct Config {
    path: String,
    freq: u32,
}
impl Config {
    fn new(mut args: std::env::Args) -> Result<Config, &'static str> {
        args.next();
        let path = match args.next() {
            Some(arg) => arg,
            None => return Err("didn't get the directory to watch."),
        };
        let freq = match args.next() {
            Some(f) => f.parse::<u32>(),
            None => "1".parse::<u32>(),
        };
        let freq = freq.unwrap_or_else(|err| {
            eprintln!("invalid frequency value: {}", err);
            process::exit(1);
        });
        Ok(Config{path, freq})
    }
}

struct Log {
    path: String,
    ctime: String,
    mtime: String,
}

impl Log {
    fn new(path: String, ctime: String, mtime: String) -> Log {
        Log {
            path,
            ctime,
            mtime,
        }
    }
}
fn watcher(path: &Path, freq: u32){
    let meta_data = fs::metadata(path).unwrap_or_else(|err| {
        eprintln!("Problem parsing metadata for {:?}: {}", path, err);
        process::exit(1)});
    let mtime = meta_data.modified().unwrap_or_else(|err| {
        eprintln!("Problem parsing time info for {:?}: {}", path, err);
        process::exit(1)});
    let ctime = meta_data.created().unwrap_or_else(|err| {
        eprintln!("Problem parsing creation time for {:?}: {}", path, err);
        process::exit(1);
    });
    let melapsed = mtime.elapsed().unwrap_or_else(|err| {
        eprintln!("Problem calulation: {}", err);
        process::exit(1)});
    
    let celapsed = ctime.elapsed().unwrap_or_else(|err| {
        eprintln!("Problem calculation: {}", err);
        process::exit(1);
    });
    //let mut flag = false;
    if melapsed <= Duration::from_secs(freq.into()) || celapsed <= Duration::from_secs(freq.into()) {
        //flag = true;
        let m_ltime: DateTime<Local> = mtime.into();
        println!("File: {:?}, Change log time: {}", path, m_ltime.format("%d %B, %Y at %r"));
    }
}
// &dyn Fn(_) : dynamically dispatch an Fn trait
fn visit_dirs(dir: &Path, freq: u32, cb: &dyn Fn(&Path, u32)) -> Result<(), io::Error> {
   if dir.is_dir() {
       for entry in fs::read_dir(dir)? {
           let entry = entry?;
           let path = entry.path();
           let name = dir.file_name().unwrap();
           let sname = name.to_os_string().into_string().unwrap();
           if sname.starts_with(".") && freq <= 9 {
               continue;
           }
           if path.is_file() {
               cb(&path, freq);
           }else if path.is_dir() {
               visit_dirs(&path,freq, cb)?;
           }else {
               continue;
           }
       }
       Ok(())
   }else {
       Err(io::Error::new(io::ErrorKind::Other, format!("is {:?} a valid directory?", dir)))
   }
}
