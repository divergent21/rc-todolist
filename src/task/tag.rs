use uuid::Uuid;
use std::cmp::PartialEq;

#[derive(Debug)]
pub struct Tag {
    #[allow(unused)]
    id: Uuid,
    title: String
}

impl Tag {    
    pub fn new(title: &str) -> Result<Self, &'static str> {
        Ok(Self {
            id: Uuid::new_v4(),
            title: Self::prepare_title(title)?
        })
    }

    pub fn get_title(&self) -> &str {
        &self.title
    }

    pub fn set_title(&mut self, title: &str) -> Result<&str, &'static str> {
        self.title = Self::prepare_title(title)?;

        Ok(&self.title)
    }

    fn prepare_title (title: &str) -> Result<String, &'static str> {
        if title.trim().is_empty() {
            return Err("the title is empty");
        }

        Ok(title.trim().replace(" ", "_"))
    }
}

impl std::fmt::Display for Tag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "#{}", self.title)
    }
}

impl PartialEq for Tag {
    fn eq(&self, other: &Self) -> bool {
        self.title == other.title
    }

    fn ne(&self, other: &Self) -> bool {
        self.title != other.title
    }
}

impl Default for Tag {
    fn default() -> Self {
        Self {
            id: Uuid::new_v4(),
            title: String::from("default")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cant_change_title_to_empty () {
        let mut tag = Tag::new("start title").unwrap();
        assert!(tag.set_title("").is_err());
    }

    #[test]
    fn change_title () {
        let mut tag = Tag::new("start title").unwrap();
        tag.set_title("new title").unwrap();
        assert_eq!("new_title", tag.get_title());
    }

    #[test]
    fn default_tag () {
        let tag = Tag::default();
        assert_eq!(tag.get_title(), "default");
    }
}