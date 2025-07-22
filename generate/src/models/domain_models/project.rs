use super::article_type::ArticleType;

#[derive(Debug)]
pub enum Project {
    Home(String),
    Blog{
        article_type: ArticleType,
        url: String
    },
    Book(String),
    DotnetBook(String),
    GoBook(String)
}


impl From<&str> for Project {
    fn from(value: &str) -> Self {
        let parts: Vec<&str> = value.split('/').collect();
        match (parts[0], parts[1]) {
            ("home", "src") => {
                if let Some(page) = parts.get(3) {
                    let url = format!("https://rustlang-es.org/{}", page)
                        .replace("projects", "proyectos")
                        .replace("communities", "comunidades")
                        .replace("/index", "");

                    Project::Home(url)
                } else {
                    Project::Home("https://rustlang-es.org".to_string())
                }
            }
            ("blog", "articles") => Project::Blog { article_type: ArticleType::Article, url: format!("https://blog.rustlang-es.org/articles/{}", parts[2]) },
            ("blog", "esta_semana_en_rust") => Project::Blog { article_type: ArticleType::ThisWeekInRust, url: format!("https://blog.rustlang-es.org/articles/{}", parts[2]) },
            ("blog", "tags") => Project::Blog { article_type: ArticleType::Tag, url: format!("https://blog.rustlang-es.org/tags/{}", parts[2]) },
            ("book", "src") => Project::Book(format!("https://book.rustlang-es.org/{}", parts[2])),
            ("dotnet", "src") => {
                if let Some(page) = parts.get(4) {
                    let url = format!("https://dotnet-book.rustlang-es.org/{}/{}", parts[3], page).replace("/index", "");
                    Project::DotnetBook(url)
                } else {
                    let url = format!("https://dotnet-book.rustlang-es.org/{}", parts[3]).replace("/index", "");
                    Project::DotnetBook(url)
                }
            },
            ("go-book", "src") => {
                if let Some(page) = parts.get(4) {
                    let url = format!("https://go-book.rustlang-es.org/es/{}/{}", parts[3], page).replace("/index", "");
                    Project::GoBook(url)
                } else {
                    let url = format!("https://go-book.rustlang-es.org/es/{}", parts[3]).replace("/index", "");
                    Project::GoBook(url)
                }
            }
            _ => panic!("invalid namespace❗: {parts:?}"),
        }
    }
}