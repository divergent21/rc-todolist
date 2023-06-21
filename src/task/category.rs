use uuid::Uuid;
use std::cmp::{PartialEq, Eq};
use std::sync::OnceLock;

#[derive(Clone, Eq, Debug)]
pub struct Category {
    // TO IMPROVE
    // add sub categoryes
    // add a parent category

    #[allow(unused)]
    id: Uuid,
    title: String,
}

static DEFAULT: OnceLock<Category> = OnceLock::new();

impl Category {
    pub fn new (title: &str) -> Result<Self, &'static str> {
        Ok(Self {
            id: Uuid::new_v4(),
            title: Self::prepare_title(title)?
        })
    }

    pub fn get_title (&self) -> &str {
        &self.title
    }

    pub fn set_title (&mut self, title: &str) -> Result<&str, &'static str> {
        self.title = Self::prepare_title(title)?;

        Ok(&self.title)
    }

    fn prepare_title (title: &str) -> Result<String, &'static str> {
        if title.trim().is_empty() {
            return Err("the title is empty");
        }

        Ok(title.trim().to_owned())
    }
}

impl PartialEq for Category {
    fn eq(&self, other: &Self) -> bool {
        self.title == other.title
    }

    fn ne(&self, other: &Self) -> bool {
        self.title != other.title
    }
}

impl std::fmt::Display for Category {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.title)
    }
}

impl Default for Category {
    fn default() -> Self {
        DEFAULT.get_or_init(|| Self {
            id: Uuid::new_v4(),
            title: String::from("Default")
        }).clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_category () {
        let category = Category::default();

        assert_eq!(category.get_title(), "Default");
    }

    #[test]
    fn update_title () {
        let mut category = Category::new("First").unwrap();
        category.set_title("Second").unwrap();

        assert_eq!(category.get_title(), "Second");
    }

    #[test]
    fn failed_update_title_to_emoty () {
        let mut category = Category::new("First").unwrap();
        assert!(category.set_title("").is_err());
    }

    #[test]
    fn equals_category () {
        let category = Category::new("First").unwrap();
        let category2 = Category::new("First").unwrap();

        assert_eq!(category, category2);
    }
}