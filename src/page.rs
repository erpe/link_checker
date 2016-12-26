extern crate url;
extern crate scraper;
extern crate hyper;

use std::io::Read;
use hyper::Client;
use scraper::{Html, Selector};
use url::Url;




#[derive(Debug)]
pub struct Page {
    pub url: Url,
    html: Option<Html>,
    pub reports: Vec<String>,
}

impl Page {
    pub fn new(url: Url) -> Page {
        let mut page = Page {
            url: url,
            html: None,
            reports: vec![],
        };
        page.init_html();
        return page;
    }

    pub fn title(&self) -> String {
        let title_selector = Selector::parse("title").unwrap();

        if let Some(ref html) = self.html {
            let title = html.select(&title_selector).next().unwrap();
            return title.inner_html();
        } else {
            panic!("not initialized page");
        }
    }

    pub fn print_report(&self) {
        println!("report: {:?}", self.reports);
    }

    fn linked_urls(&self) -> Vec<String> {
        let mut url_vec = vec![];
        if let Some(ref html) = self.html {
            let selector = Selector::parse("a").unwrap();
            for element in html.select(&selector) {
                match element.value().attr("href") {
                    Some(url) => url_vec.push(url.to_string()),
                    None => self.print_out("no href value"),
                }
            }
        }
        return url_vec;
    }

    fn print_out(&self, msg: &str) {
        println!("msg: {}", msg)
    }

    fn init_html(&mut self) {
        let client = Client::new();
        let ref url = self.url;
        let mut res = client.get(url.as_str()).send().unwrap();
        println!("status: {}", res.status);
        //println!("headers: {:?}", res.headers);

        let mut res_string = String::new();
        match res.read_to_string(&mut res_string) {
            Err(why) => panic!("error string conversion failure: {:?}", why),
            Ok(_) => println!("Read page to string"),
        }
        self.html = Some(Html::parse_document(&res_string));
        for urlstring in self.linked_urls() {
            self.reports.push(urlstring);
        }
    }
}
