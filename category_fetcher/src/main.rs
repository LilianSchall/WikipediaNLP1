use rayon::iter::{IntoParallelIterator, ParallelIterator};
use reqwest;

use csv;
use serde_json::json;
use std::{
    collections::HashMap,
    fs::File,
    io::{BufReader, Write},
    path::Path
};

mod wikipedia;

use wikipedia::{Article, Category, WikipediaResponse};

fn process_article(a: Article) -> (i32, Option<Vec<Category>>) {
    let params = [
        ("action", "query"),
        ("format", "json"),
        ("prop", "categories"),
        ("cllimit", "max"),
        ("clshow", "!hidden"),
        ("titles", &a.title),
    ];
    let uri = "https://en.wikipedia.org/w/api.php";

    let url = reqwest::Url::parse_with_params(uri, params).unwrap();
    let url_string = url.to_string();

    let mut server_response = reqwest::blocking::get(&url_string);

    while (server_response.is_err()) {
        println!("Got timedout for title: {}", a.title);
        std::thread::sleep(std::time::Duration::from_millis(100));
        server_response = reqwest::blocking::get(&url_string);
    }

    let body = server_response.unwrap().text().unwrap();

    let response: Result<WikipediaResponse, serde_json::Error> = serde_json::from_str(&body);

    if response.as_ref().err().is_some() {
        return (-1, None);
    }
    println!("Fetched categories for {}", a.title);

    let mut categories: Vec<Category> = Vec::new();

    let wiki_response: WikipediaResponse = response.unwrap();

    for page_key in wiki_response.query.pages.keys() {
        let page = wiki_response.query.pages[page_key].clone();
        categories = [categories, page.categories].concat();
    }

    (a.id, Some(categories))
}

fn load_articles(file_path: &str) -> Vec<Article> {
    let file = File::open(file_path).unwrap();
    let file_reader = BufReader::new(file);
    let mut rdr = csv::Reader::from_reader(file_reader);

    let articles: Vec<Article> = rdr
        .deserialize::<Article>()
        .into_iter()
        .map(|r| r.unwrap())
        .collect();

    articles
}

fn fetch_category(file_path: &str) -> HashMap<i32, Vec<String>> {
    let articles = load_articles(file_path);
    println!("nb articles: {}", articles.len());

    let mapping = articles
        .into_par_iter()
        .map(process_article)
        .filter(|item| item.1.is_some())
        .fold(
            || HashMap::new(),
            |mut hashmap: HashMap<i32, Vec<String>>,
             item: (i32, std::option::Option<Vec<Category>>)| {
                let mut categories: Vec<String> = Vec::new();
                for cat in item.1.unwrap() {
                    categories.push(cat.title);
                }
                hashmap.insert(item.0, categories);
                hashmap
            },
        )
        .reduce(
            || HashMap::new(),
            |m1, m2| {
                m2.iter().fold(m1, |mut acc, (k, vs)| {
                    acc.insert(k.clone(), vs.clone());
                    acc
                })
            },
        );
    mapping
}

fn get_shard_number(name: String) -> String {
    let tokens: Vec<&str> = name.split("_").last().unwrap().split(".").collect();

    tokens.first().unwrap().to_string()
}

fn process_shards(input_path: String, output_path: String){
    let shards = std::fs::read_dir(input_path).unwrap();

    for shard in shards {
        // build output path
        let entry = shard.expect("Unknown error on shard process");
        let entry_path = entry.path();
        let shard_name = entry_path.file_name().unwrap().to_str().unwrap();
        let shard_number = get_shard_number(shard_name.to_string());
        let mut output_path = Path::new(&output_path).join("shard_".to_string() + &shard_number + ".json");

        let categories = fetch_category(entry.path().to_str().unwrap());
        let json = json!(categories);
        let mut file = File::create(output_path).unwrap();
        file.write_all(json.to_string().as_bytes());

        println!("Processed shard {}", shard_number);
    }

}

fn main() {
    let output_path: String = std::env::var("OUTPUT_DIR").expect("Please set OUTPUT_DIR environment variable.");
    let input_path: String = std::env::var("INPUT_DIR").expect("Please set INPUT_DIR environment variable.");

    process_shards(input_path, output_path);
}
