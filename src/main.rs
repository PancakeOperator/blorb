use std::io::{BufReader, BufRead};
use std::fs::{self};
use std::env;
use std::io;

struct PathWay {
    file_path:  Vec<String>,
    file_name:  Vec<String>,
    err:    Option<String>
}

impl PathWay {
    #![allow(dead_code)]
    pub fn new() -> Self {
        let pathway = Self {
            file_path: vec!["./".to_string()],
            file_name: vec![],
            err: None,
        };
        pathway
    }

    pub fn read_file(name: &str) -> io::Result<()> {
       let filename = fs::File::open(&name)?;

       let read = BufReader::new(filename);

       for line in read.lines() {
            let line = line?;
            println!("{}", line);
       }
       Ok(())
    }
    pub fn curren_dir() -> io::Result<()> {
        let cur_dir = env::current_dir()?;
        
       for entry in fs::read_dir(cur_dir)? {
        let entry = entry?;
            let path = entry.path();

            let metadata = fs::metadata(&path)?;
            let last_modified = metadata.modified()?;
            
            println!("Last modified: {:?}, is read only: {:?}, size: {:?} bytes, filename: {:?}",
                last_modified,
                metadata.permissions().readonly(),
                metadata.len(),
                path.file_name().ok_or("no filename")
            );
            
        }
        Ok(())
    }
    
    #[allow(dead_code)]
    pub fn find_file() {
        //let lookup = 
    }
    #[allow(dead_code)]
    pub fn reload_path_list(&mut self) {
        let cur_path = self.file_path.last().unwrap();

        let paths = match std::fs::read_dir(cur_path) {
            Ok(e) => e,
            Err(err) => {
                let err = format!("An error has occured: {:?}", err);
                self.err = Some(err);
                self.file_path.pop();
                return;
            }
        };

        let collected = paths.collect::<Vec<_>>();

        self.clear_err();
        self.file_name.clear();

        for path in collected {
            self.file_name
                .push(path.unwrap().path().display().to_string());
        }
    }
    pub fn clear_err(&mut self) {
        self.err = None;
    }



}
pub struct Matchway {
    flags: env::Args,
    err: Option<String>,
}
impl Matchway {
    pub fn new() -> Self {
        let matchway = Self {
            flags: vec![""],
            err: None
        };
        matchway
    }
    pub fn about()  {
        print!(concat!("Usage: ", env!("CARGO_PKG_NAME"), " [OPTION]... [FILE]...\n", 
           "  -f, --filenames          display filenames\n",
           "  -r, --read               display verbose output\n",
           "  -?, --help               display this help and exit\n",
        ));
    }
    pub fn matching() {
        let _args = (env::args()).skip(1);
        let mut _names = Vec::new();

        for _arg in _args {
            if _arg.starts_with("-") && _arg.len() > 1 {
                if _arg == "-d" || _arg == "--filenames" || _arg == "--filename" {
                    PathWay::curren_dir();
                } else if _arg == "-r" || _arg == "--read" {
                    PathWay::read_file("E:/forps/src/peop.txt").expect("failed to read files");
                }
            }
            else {
                _names.push(_arg)
            }
        }



    }
}
fn main() {
    
}