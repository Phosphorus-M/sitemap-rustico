use serde::{Deserialize, Serialize};
use crate::{iso_8601, models::domain_models::project::Project};

use super::{changefreq::Changefreq, freq::Freq};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Url {
    #[serde(rename = "loc")]
    url: String,
    #[serde(rename = "lastmod")]
    updated_at: Option<String>,
    #[serde(rename = "changefreq")]
    freq: Option<Changefreq>,
    priority: Option<String>,
}

/// Adapter to convert a tuple of date and Project to Url
impl From<(String, Project)> for Url {
    fn from((date, project): (String, Project)) -> Self {
        match project {
            Project::Home(url) => Url {
                url,
                updated_at: if date.is_empty() {
                    Some(iso_8601(&std::time::SystemTime::now()))
                } else {
                    Some(date.to_string())
                },
                freq: Some(Changefreq {
                    field: Freq::Monthly,
                }),
                priority: Some("0.75".to_string()),
            },
            Project::Blog { article_type, url } => Url {
                url,
                updated_at: if date.is_empty() {
                    Some(iso_8601(&std::time::SystemTime::now()))
                } else {
                    Some(date.to_string())
                },
                freq: Some(Changefreq::from(article_type)),
                priority: Some("0.75".to_string()),
            },
            Project::Book(url) | Project::DotnetBook(url) | Project::GoBook(url) => Url {
                url,
                updated_at: if date.is_empty() {
                    Some(iso_8601(&std::time::SystemTime::now()))
                } else {
                    Some(date.to_string())
                },
                freq: Some(Changefreq {
                    field: Freq::Yearly,
                }),
                priority: Some("1.0".to_string()),
            },
        }
    }
}
