
// pdflc-rs - simple PDF hyperlink reference checker

    // error!("{}", "And error occurred");
// warn!("{:#?}", "This is important");
// info!("{:?}", "Take note");
// debug!("Something weird occurred: {}", "Error");

extern crate pdf_extract;

use std::io::{self, Write, BufWriter, Error};
use std::io::stdout;
use std::fs::{File};
use std::env;
use std::path::PathBuf;
use pdf_extract::extract_text;
use linkify::{LinkFinder, LinkKind};
// use log::{ info, error, debug, warn };
use reqwest::{StatusCode,Url};

use clap::{arg, Arg, ArgMatches, command, value_parser};
// use clap::{ArgAction, Command};

// ///////////////////////////



#[derive(Debug)]
struct LinkStatus {
    link: Url,
    status: StatusCode
}

impl LinkStatus {
    fn new(link: Url, status: StatusCode) -> Self {
        LinkStatus{link, status}
    }
}

fn test_links_in_pdf (input_path: &PathBuf) -> Result<Vec<LinkStatus>, String> {
    
    let mut tested_links: Vec<LinkStatus> = Vec::new();
    
    let pdf_text = extract_text(&input_path).unwrap();
    
    let mut finder = LinkFinder::new();
    finder.kinds(&[LinkKind::Url]);
    
    let links: Vec<_> = finder.links(&pdf_text).collect();
    
    for link in links {
        let slink = link.as_str();
        let res = reqwest::blocking::get(slink).unwrap();
        tested_links.push(LinkStatus::new(Url::parse(slink).unwrap(),res.status())) 
    }
    
    Ok(tested_links)

}

fn get_args() -> ArgMatches {
    command!() 
    .arg(arg!([input_path] "Path to PDF file")
        .required(true)
        .value_parser(value_parser!(PathBuf))
    )
    .arg(
        arg!(
            -o --output <FILE> "Output file"
        )
        .required(false)
        .value_parser(value_parser!(PathBuf)),
    )
    .get_matches()
}

fn main() {

    env_logger::init();
    
    let matches = get_args();

    let input_path = matches.get_one::<PathBuf>("input_path").unwrap();

    let stdout = io::stdout();
    let mut write_handle = io::BufWriter::new(stdout);

    let write_handle_result: Result<Box<dyn Write>, Error> = match matches.get_one::<PathBuf>("output") {
        Some(ref path) => File::open(path).map(|f| Box::new(f) as Box<dyn Write>),
        None => Ok(Box::new(io::stdout())),
    };

    let tested_links = test_links_in_pdf(input_path).unwrap();

    if let Ok(mut write_handle) = write_handle_result {
       for ls in tested_links.iter() {
           writeln!(write_handle, "Link: {} - Status: {}", ls.link.as_str(), ls.status).expect("Unable to write.");
       }
       write_handle.flush().unwrap();  
    };

    

   
      
}
