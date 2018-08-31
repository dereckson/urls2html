use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::io::Error as IOError;
use std::io::prelude::*;
use std::collections::BTreeMap;

pub static DEFAULT_TITLE: &'static str = "Links";

pub struct Config {
    pub filename: String,
    pub title: String,
    pub full_html_document: bool,
}

impl Config {
    pub fn new() -> Self {
        Config {
            filename: String::new(),
            title: String::new(),
            full_html_document: true,
        }
    }
}

pub struct Link {
    pub id: String,
    pub url: String,
    pub title: String,
}

impl<'a> Link {
    /// Generates a link from a parsed string, with values separated by the specified delimiter.
    ///
    /// # Examples
    ///
    /// ```
    /// let text = "001::http://www.perdu.com::Perdu ? Vous êtes ici.";
    /// let link = Link::from_str(text, "::");
    ///
    /// assert_eq!("001", link.id);
    /// assert_eq!("http://www.perdu.com", link.url);
    /// assert_eq!("Perdu ? Vous êtes ici.", link.title);
    /// ```
    pub fn from_str (text: &'a str, delimiter: &str) -> Result<Self, &'a str> {
        let fragments: Vec<&str> = text.split(delimiter).collect();

        if fragments.len() < 3 {
            return Err("Can't parse link");
        }

        Ok(Link {
            id: String::from(fragments[0]),
            url: String::from(fragments[1]),
            title: String::from(fragments[2]),
        })
    }
}

pub fn run (config: Config) -> Result<(), Box<dyn Error>> {
    let links = extract_link_from_files(&config.filename)?;

    if config.full_html_document {
        println!("<html>");
        println!("<head>");
        println!("  <meta charset=\"utf-8\"/>");
        println!("  <title>{}</title>", config.title);
        println!("</head>");
        println!("<body>");
        println!("  <h1>{}</h1>", config.title);
    }

    println!("  <ul>");
    for (id, link) in links {
        println!("      <li><a href=\"{}\">{} — {}</a></li>", link.url, id, link.title);
    }
    println!("  </ul>");

    if config.full_html_document {
        println!("</body>");
        println!("</html>");
    }

    Ok(())
}

pub fn extract_link_from_files (filename: &str) -> Result<BTreeMap<String, Link>, IOError> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    let mut links = BTreeMap::new();

    for line in reader.lines() {
        match line {
            Ok(text) => {
                if let Ok(link) = Link::from_str(&text, "\t") {
                    links.insert(link.id.clone(), link);
                }
            },
            Err(e) => { return Err(e); },
        }
    }

    Ok(links)
}
