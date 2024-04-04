use rayon::iter::{IntoParallelIterator, ParallelIterator};
use reqwest;
use serde;

use csv;
use std::{fs::File, error::Error, io::BufReader, process, collections::HashMap};

#[derive(Debug, serde::Deserialize)]
struct Article {
    #[serde(rename = "")]
    line_number: i32,
    id: i32,
    title: String,
}

#[derive(Debug, serde::Deserialize)]
struct Category {
    ns: i32,
    title: String,
}

#[derive(Debug, serde::Deserialize)]
struct Page {
    pageid: i32,
    ns: i32,
    title: String,
    categories: Vec<Category>
}

#[derive(Debug, serde::Deserialize)]
struct Query {
    pages: HashMap<String, Page>
}

#[derive(Debug, serde::Deserialize)]
struct WikipediaResponse {
    batchcomplete: String,
    query: Query,
    limits: HashMap<String, i32>
}

fn print_title() -> Result<(), Box<dyn Error>> {
    let file = File::open("shard_example.csv")?;
    let file_reader = BufReader::new(file);
    let mut rdr = csv::Reader::from_reader(file_reader);

    let uri = "https://en.wikipedia.org/w/api.php";

    let articles : Vec<Article> = rdr.deserialize::<Article>().into_iter().map(|r| r.unwrap()).collect();

    let mut hashmap : HashMap<i32, Vec<String>>;

    // TODO: replace the for_each by a map
    articles.into_par_iter().for_each(|a| {
        let params = [
            ("action", "query"),
	    ("format", "json"),
	    ("prop", "categories"),
	    ("cllimit", "max"),
	    ("clshow", "!hidden"),
            ("titles", &a.title),
        ];

        let url = reqwest::Url::parse_with_params(uri, params).unwrap();
        let body = reqwest::blocking::get(url).unwrap()
            .text().unwrap();

        let mut hashmap: HashMap<i32, Vec<String>>= HashMap::new();

        println!("Processed {}", a.title);
    });
    // fold then reduce into one single hashmap.

    Ok(())
}

fn main() {

    if let Err(err) = print_title() {
        println!("error running print_title: {}", err);
        process::exit(1);
    }
}
