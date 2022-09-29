use std::io::{Write, BufReader, BufRead, Error};
use std::path::Path;
use std::io::Read;
use std::fs::{self};
use std::env;
use std::io;
use std::fs::File;
use std::io::prelude::*;


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


    PathWay::read_file("E:/forps/src/peop.txt").expect("fail");


}