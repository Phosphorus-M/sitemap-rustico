use serde::{Deserialize, Serialize};
use crate::models::domain_models::article_type::ArticleType;

use super::freq::Freq;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Changefreq {
    #[serde(rename = "$text")]
    pub(crate) field: Freq,
}

/// Adapter to convert ArticleType to Changefreq
impl From<ArticleType> for Changefreq {
    fn from(value: ArticleType) -> Self {
        match value {
            ArticleType::ThisWeekInRust => Changefreq {
                field: Freq::Monthly,
            },
            ArticleType::Article => Changefreq {
                field: Freq::Monthly,
            },
            ArticleType::Tag => Changefreq {
                field: Freq::Monthly,
            },
        }
    }
}
