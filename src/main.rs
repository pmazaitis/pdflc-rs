
// pdflc-rs - simple PDF hyperlink reference checker



extern crate pdf_extract;
extern crate lopdf;
// extern crate linkify;

use std::env;
use std::path::PathBuf;
use pdf_extract::extract_text;
use linkify::{LinkFinder, LinkKind};

use clap::{arg, command, value_parser, ArgAction, Command};



fn main() {
    // println!("Hello, world!");
    
    let matches = command!() 
        .arg(arg!([input_path] "Path of PDF file to use")
            .required(true)
            .value_parser(value_parser!(PathBuf))
        )
        .get_matches();

    let input_path = matches.get_one::<PathBuf>("input_path").unwrap();

    let pdf_text = extract_text(&input_path).unwrap();  
    
    let mut finder = LinkFinder::new();
    
    finder.kinds(&[LinkKind::Url]);
    
    let links: Vec<_> = finder.links(&pdf_text).collect();
 
    for link in links {
        
        let slink = link.as_str();
        println!("Link: {:?}", &slink);
        let res = reqwest::blocking::get(slink).unwrap();
        println!("Status: {}", res.status());
        
    }
 
    // println!("{}",pdf_text);    
}
