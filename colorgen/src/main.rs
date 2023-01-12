use std::fs::{self, File};
use std::io::Write;
use std::sync::Arc;

use directories::ProjectDirs;
use raytracer::color::Color;
use regex::Regex;
use reqwest::{ClientBuilder, Url};
use scraper::{Html, Selector};

const COLORS: [&'static str; 3] = ["#000000", "#9f2172", "#e32636"];

const USER_AGENT: &'static str =
    "Mozilla/5.0 (X11; Ubuntu; Linux x86_64; rv:15.0) Gecko/20100101 Firefox/15.0.1";

struct ColorConst {
    name: String,
    color: Color,
}

impl std::fmt::Display for ColorConst {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "const {}: Color = Color {{
\tred: {}
\tgreen: {}
\tblue: {}
}};",
            self.name, self.color.red, self.color.green, self.color.blue
        )
    }
}

#[tokio::main]
async fn main() {
    let project_dirs =
        Arc::new(ProjectDirs::from("regexPattern", "raytracer", "colorgen").unwrap());

    for color in COLORS {
        // TODO: I could throw multiple color formats to the COLORS array, to maybe I could
        // implement a function to parse all those formats and convert them to hex, just the append
        // them to the base URL as Encycolorpedia uses them.
        let color_id = color.replace("#", "");

        let cache_dir_path = project_dirs.cache_dir();
        let cached_file_path = cache_dir_path.join(&color_id);

        let html = match fs::read_to_string(&cached_file_path) {
            Ok(content) => {
                // println!("[READING]: {}", cached_file_path.to_str().unwrap());
                content
            }
            Err(_) => {
                let url = Url::parse("https://encycolorpedia.com/")
                    .unwrap()
                    .join(&color_id)
                    .unwrap();

                // println!("[FETCHING]: {}", url);

                let cached_file_path = cached_file_path.clone();

                (tokio::spawn(async move {
                    let client = ClientBuilder::new().user_agent(USER_AGENT).build().unwrap();
                    let response = client.get(url).send().await.unwrap();
                    let content = response.text().await.unwrap();

                    let mut file = File::create(&cached_file_path).unwrap();
                    file.write(&content.as_bytes()).unwrap();

                    content
                }))
                .await
                .unwrap()
            }
        };

        let document = Html::parse_document(&html);
        let title = document
            .select(&Selector::parse("#information>h1").unwrap())
            .next()
            .unwrap()
            .inner_html();
        let desc = document
            .select(&Selector::parse("#information>p").unwrap())
            .next()
            .unwrap()
            .inner_html();

        let name_re = Regex::new(r"^[a-zA-Z\s]+").unwrap();
        let hexcode_re = Regex::new(r"#([a-zA-Z0-9]{6})").unwrap();
        let desc_re = Regex::new(r"(\d+)% (?:red|green|blue)").unwrap();

        let color_hexcode = hexcode_re
            .captures(&title)
            .unwrap()
            .get(1)
            .unwrap()
            .as_str();
        let color_name = name_re.captures(&title).map_or_else(
            || "HEX_".to_string() + &color_hexcode.to_uppercase(),
            |c| {
                c.get(0)
                    .unwrap()
                    .as_str()
                    .trim()
                    .replace(" ", "_")
                    .to_uppercase()
            },
        );

        let mut color_percentages = desc_re.captures_iter(&desc);
        let red_percentage = color_percentages
            .next()
            .map(|perc| perc.get(1).unwrap().as_str().parse::<f64>().unwrap())
            .unwrap();

        let green_percentage = color_percentages
            .next()
            .map(|perc| perc.get(1).unwrap().as_str().parse::<f64>().unwrap())
            .unwrap();

        let blue_percentage = color_percentages
            .next()
            .map(|perc| perc.get(1).unwrap().as_str().parse::<f64>().unwrap())
            .unwrap();

        let color = ColorConst {
            name: color_name,
            color: Color {
                red: red_percentage / 100.0,
                green: green_percentage / 100.0,
                blue: blue_percentage / 100.0,
            },
        };

        println!("{color}");
    }
}
