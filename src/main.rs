
// pdflc-rs - simple PDF hyperlink reference checker

    // error!("{}", "And error occurred");
// warn!("{:#?}", "This is important");
// info!("{:?}", "Take note");
// debug!("Something weird occurred: {}", "Error");

extern crate pdf_extract;

use std::env;
use std::path::PathBuf;
use pdf_extract::extract_text;
use linkify::{LinkFinder, LinkKind};
// use log::{ info, error, debug, warn };
use reqwest::{StatusCode,Url};
use csv::Writer;

use clap::{arg, ArgMatches, command, value_parser};
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

    let tested_links = test_links_in_pdf(input_path).unwrap();

    // If there is an output specified, dispatch the data to the appropriate writer
    // Else, just send to stdout
    match matches.get_one::<PathBuf>("output")  {
        Some(outpath) => {
            // We'll just do CSV for now
            let mut csv_writer = Writer::from_path(outpath.as_os_str()).unwrap();
            csv_writer.write_record(&["Link","Status"]).unwrap();
            for ls in tested_links.iter() {
                csv_writer.write_record(&[ls.link.as_str(),ls.status.as_str()]).unwrap();
            }
            csv_writer.flush().unwrap();
        },
        None => {
            for ls in tested_links.iter() {
                println!("Link: {} - Status: {}", ls.link.as_str(), ls.status);
            }  
        }
    }

    

   
      
}
