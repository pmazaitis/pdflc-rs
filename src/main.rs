
// pdflc-rs - simple PDF hyperlink reference checker

extern crate pdf_extract;

use std::io::{self, Write, BufWriter};
use std::io::stdout;
use std::fs::{File};
use std::env;
use std::path::PathBuf;
use pdf_extract::extract_text;
use linkify::{LinkFinder, LinkKind};
use log::{ info, error, debug, warn };
use reqwest::{StatusCode,Url};

use clap::{arg, command, value_parser};
// use clap::{ArgAction, Command};

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

fn main() {

    env_logger::init();
    
    // error!("{}", "And error occurred");
    // warn!("{:#?}", "This is important");
    // info!("{:?}", "Take note");
    // debug!("Something weird occurred: {}", "Error");
    
    let matches = command!() 
        .arg(arg!([input_path] "Path of PDF file to use")
            .required(true)
            .value_parser(value_parser!(PathBuf))
        )
        .get_matches();

    let input_path = matches.get_one::<PathBuf>("input_path").unwrap();


    let stdout = io::stdout();
    let mut write_handle = io::BufWriter::new(stdout);

    let tested_links = test_links_in_pdf(input_path).unwrap();

    for ls in tested_links.iter() {
        writeln!(write_handle, "Link: {} - Status: {}", ls.link.as_str(), ls.status);
    }    
      
}
