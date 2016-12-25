mod page;
extern crate clap;
extern crate hyper;
extern crate scraper;
extern crate url;

use std::str;
use clap::{Arg, App};
use url::{Url};
use page::Page;

fn main() {
    let matches = App::new("Link Checker")
        .version("0.1.0")
        .author("rene paulokat <rene@so36.net>")
        .arg(Arg::with_name("url")
             .short("u")
             .long("url")
             .value_name("URL")
             .help("Processes given URL")
             .takes_value(true)
             .required(true))
        .get_matches();
   
    let url = matches.value_of("url").unwrap();
    let a_url = Url::parse(url).unwrap();
    let my_page = Page::new(a_url);
    println!("title: {}", my_page.title());
    my_page.linked_urls()
}
