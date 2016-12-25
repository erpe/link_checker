extern crate url;
extern crate scraper;
extern crate hyper;

use std::io::Read;
use hyper::{Client};
use scraper::{Html,Selector};

#[derive(Debug)]
pub struct Page {
  pub url: url::Url,
  html: Option<scraper::Html>
}

impl Page {

    pub fn new(url: url::Url) -> Page {
        let mut page = Page{url: url, html: None};
        page.init_html();
        return page
    }

    pub fn title(&self) -> String {
        let title_selector = Selector::parse("title").unwrap();
        
        if let Some(ref html) = self.html {
            let title = html.select(&title_selector).next().unwrap();
            let title_text = title.inner_html();
            return title_text
        } else {
           panic!("not initialized page");
        }
    }

    pub fn linked_urls(&self) {
        if let Some(ref html) = self.html {
            let selector = Selector::parse("a").unwrap();
            for element in html.select(&selector) {
                println!("a: {:?}", element.value().attr("href"));
                assert_eq!("a", element.value().name());
            }
        }
    }


    fn init_html(&mut self) {
        let client = Client::new();
        let ref url = self.url;
        let mut res = client.get(url.as_str()).send().unwrap();
        println!("status: {}", res.status);
        println!("headers: {:?}", res.headers);

        let mut res_string = String::new();
        match res.read_to_string(&mut res_string) {
            Err(why) => panic!("error string conversion failure: {:?}", why),
            Ok(_) => println!("Read page to string")
        }
        self.html = Some(Html::parse_document(&res_string));
    }
}
