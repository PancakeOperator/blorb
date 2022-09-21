
use std::io::Read;
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

    pub fn read_file(p: String) -> io::Result<()> {
        let mut f = std::fs::File::open(p)?;
        let mut buf = String::new();

        f.read_to_string(&mut buf)?;
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
    /*
    let mut fs = String::new();

    std::io::stdin()
        .read_line(&mut fs)
        .expect("fail");
    
    let _dir = fs;
    // remove
    let dir = Files::new().path_name;

    println!("{:?}", dir); */
    println!("what file do you want to read");
    let mut p = String::new();
    io::stdin().read_line(&mut p).expect("fail");
    let fc = PathWay::read_file(p);
    
    
    println!("{:?}", fc);
}