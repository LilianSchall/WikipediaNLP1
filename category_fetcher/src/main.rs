use rayon::iter::{IntoParallelIterator, ParallelIterator};
use reqwest;
use serde;

use csv;
use serde_json::json;
use std::{fs::File, error::Error, io::{BufReader, Write}, process, collections::HashMap};

#[derive(Debug, serde::Deserialize)]
struct Article {
    #[serde(rename = "")]
    line_number: i32,
    id: i32,
    title: String,
}

#[derive(Debug, Clone, serde::Deserialize)]
struct Category {
    ns: i32,
    title: String,
}

#[derive(Debug, Clone, serde::Deserialize)]
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

fn print_title() -> Result<HashMap<i32, Vec<String>>, Box<dyn Error>> {
    let file = File::open("shard_example.csv")?;
    let file_reader = BufReader::new(file);
    let mut rdr = csv::Reader::from_reader(file_reader);

    let uri = "https://en.wikipedia.org/w/api.php";

    let articles : Vec<Article> = rdr.deserialize::<Article>().into_iter().map(|r| r.unwrap()).collect();
    println!("nb articles: {}", articles.len());

    // TODO: replace the for_each by a map
    let mapping = articles
        .into_par_iter()
        .map(|a| {
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

        let response: Result<WikipediaResponse, serde_json::Error> = serde_json::from_str(&body);

        if response.as_ref().err().is_some() {
            return (-1, None);
        }

        let mut categories : Vec<Category> = Vec::new();

        let wiki_response: WikipediaResponse = response.unwrap();

        for page_key in wiki_response.query.pages.keys() {
            let page = wiki_response.query.pages[page_key].clone();
            categories = [categories, page.categories].concat();
        }
        
        println!("Processed Title: {}", a.title);

        (a.id, Some(categories))
    })
    .filter(|item| item.1.is_some())
    .fold(|| HashMap::new(), |mut hashmap: HashMap<i32, Vec<String>>, item:(i32, std::option::Option<Vec<Category>>)| {
        let mut categories: Vec<String> = Vec::new();
        for cat in item.1.unwrap() {
            categories.push(cat.title);
        }
        hashmap.insert(item.0, categories);
        hashmap
    })
    .reduce(|| HashMap::new(),
        |m1, m2| {
            m2.iter().fold(m1, |mut acc, (k, vs)| {
                acc.insert(k.clone(), vs.clone());
                acc
            })
        },);
    // fold then reduce into one single hashmap.
    Ok(mapping)
}

fn main() {


    match print_title() {
        Err(err) => {
            println!("error running print_title: {}", err);
            process::exit(1);
        }
        Ok(val) => {
            let json = json!(val);

            let mut file = File::create("category_example.json").unwrap();
            file.write_all(json.to_string().as_bytes());
        }
    }
}
