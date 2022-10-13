
use std::path::PathBuf;
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
    
    pub fn rm_file(name: &str) -> io::Result<()> {
        fs::remove_file(name)?;
        Ok(())
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
    pub fn modified_dir() -> io::Result<()> {
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
    pub fn current_dirs() -> io::Result<()> {
        let _curr_dir = env::current_dir()?;

        for look_at in fs::read_dir(_curr_dir)? {
            let look_at = look_at?;
            let path = look_at.path();

            println!("{:?}", path);
        }
        Ok(())
    }

    //attempt to find files / might not work yet
    pub fn seek(name: &str) {
        let _names = std::path::Path::new(name);

        let name = _names.parent().unwrap();
        
        println!("{:?}", name  )
        
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

fn main() {
    matching();
}

fn about()  {
    print!(concat!("Usage: ", env!("CARGO_PKG_NAME"), " [OPTION]...\n", 
       "  -m, --modir       display modified directories\n",
       "  -r, --read        reads files\n",
       "  -d, --dir         displays directories\n",
       "  -?, --help        display this help and exit\n",
       "\nExample\n",
       "   ", env!("CARGO_PKG_NAME"), " -d\toutputs all files in the current directory"
    ));
}

fn matching() {
    let _args = (env::args()).skip(1);
    let mut _names = Vec::new();

    for _arg in _args {
        if _arg.starts_with("-") && _arg.len() > 1 {
            if _arg == "-m" || _arg == "--modified_dir" || _arg == "--modir" {
                PathWay::modified_dir().expect("fail");
            } else if _arg == "-r" || _arg == "--read" {
                PathWay::read_file("E:/emil/src/peop.txt").expect("failed to read files");
            } else if _arg == "-?" || _arg == "--help" {
                about();
                std::process::exit(0);
            } else if _arg == "-d" || _arg == "--dir" {
                PathWay::current_dirs().expect("failed to find ");
            } else if _arg == "--seek" {
                println!("What file do you want to look for");
                let mut name = String::new();
                io::stdin()
                    .read_line(&mut name)
                    .expect("fail");
                PathWay::seek(&name);
            } else if _arg == "-rmf" || _arg == "--rm_file" {
                println!("What file do you want to delete?");
                let mut file = String::new();
                io::stdin()
                    .read_line(&mut file)
                    .expect("failure to find file");
                PathWay::rm_file(&file);  
            }
        }
        else {
            _names.push(_arg)
        }
    }
}
