use std::fs::read_to_string;
use std::io::Write;
use std::{fs::File, process::Command};
use models::domain_models::project::Project;
use models::sitemap_models::url::Url;
use models::sitemap_models::url_set::UrlSet;
/*
 * usage:
 * cargo run
 *
 * @see https://www.sitemaps.org/protocol.html
 * @see https://developers.google.com/search/docs/crawling-indexing/sitemaps/build-sitemap
 * @see https://developers.google.com/search/docs/crawling-indexing/sitemaps/build-sitemap#addsitemap
 * @see https://www.wikiwand.com/en/Sitemaps#Additional_sitemap_types
 */
mod models;

const PATHS_TO_IGNORE: [&str; 7] = [
    "book/src/SUMMARY",
    "home/src/pages/communidad",   /* communidad.rs */
    "home/src/pages/contributors", /* contributors.rs */
    "home/src/pages/mod",          /* mod.rs */
    /* dotnet book*/
    "dotnet/src/es/SUMMARY",
    "dotnet/src/es/license",
    "go-book/src/es/SUMMARY",
];

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let filename = "generated/dates_and_paths.txt";

    let content = read_to_string(filename).expect("Failed to open '{filename}' file");

    let items = content
        .lines()
        .map(|line| {
            let parts: Vec<&str> = line.split(',').collect();
            (parts[0], parts[1])
        })
        .filter(|(_date, path)| !PATHS_TO_IGNORE.contains(path))
        .map(|(date, path)| {
            let project_classification = Project::from(path);

            Url::from((date.to_string(), project_classification))
        })
        .collect::<Vec<_>>();


    /*
     * The Sitemap XML protocol is also extended to provide
     * a way of listing multiple Sitemaps in a 'Sitemap index' file.
     * The maximum Sitemap size of 50 MiB or 50,000 URLs means this is necessary for large sites.
     *
     * @see https://www.wikiwand.com/en/Sitemaps#File_format
     */
    let xml = quick_xml::se::to_string(&UrlSet::new(items))?;

    let mut file = File::create("sitemap.xml").expect("Failed to create file");
    write!(file, "{}", xml).expect("Failed to write sitemap.xml file");

    validate_sitemap();
    Ok(())
}

fn validate_sitemap() {
    let cmd = Command::new("xmllint")
        .arg("--schema")
        .arg("schema.xsd")
        .arg("sitemap.xml")
        .output()
        .expect("- Failed to execute xmllint command\n- Did you install xmllint on windows?\n- Try: scoop install libxml2\n");

    println!("Ok: {}", String::from_utf8_lossy(&cmd.stderr));
}

pub fn iso_8601(system_time: &std::time::SystemTime) -> String {
    use chrono::prelude::{DateTime, Utc};
    // * @see https://www.w3.org/TR/NOTE-datetime
    let datetime: DateTime<Utc> = (*system_time).into();
    //? formats like "2001-07-08T00:34:60.026490+09:30"
    format!("{}", datetime.format("%+"))
}
