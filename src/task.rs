mod status;
mod category;
mod tag;
mod priority;
mod builder;

pub use status::TaskStatus;
pub use category::Category;
pub use tag::Tag;
pub use priority::Priority;

use uuid::Uuid;
use chrono::prelude::*;
use std::cmp::PartialEq;

pub use builder::TaskBuilder;

#[derive(PartialEq)]
pub struct Task {
    id: Uuid,

    title: String,
    description: String,

    status: TaskStatus,

    category: Category,
    tags: Vec<Tag>,

    // TP IMPROVE
    // add sub tasks

    priority: Priority,

    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,

    deadline: Option<DateTime<Utc>>
}

impl Task {
    fn prepare_title (title: &str) -> Result<String, &'static str> {
        if title.trim().is_empty() {
            return Err("Title is empty.");
        }

        Ok(title.trim().to_owned())
    }

    pub fn new (title: &str) -> Result<TaskBuilder, &'static str> {
        Ok(TaskBuilder {
            title: Self::prepare_title(title)?,
            description: None,
            status: None,
            category: None,
            tags: None,
            priority: None,
            deadline: None
        })
    }

    pub fn set_title (&mut self, title: &str) -> Result<(), &'static str> {
        self.title = Self::prepare_title(title)?;
        self.update_time();

        Ok(())
    }

    pub fn get_title (&self) -> &str {
        &self.title
    }

    pub fn set_description (&mut self, description: &str) {
        self.update_time();
        self.description = description.to_owned();
    }

    pub fn get_description (&self) -> &str {
        &self.description
    }

    pub fn set_status (&mut self, status: TaskStatus) {
        self.update_time();
        self.status = status;
    }

    pub fn get_status (&self) -> &TaskStatus {
        &self.status
    }

    pub fn set_category (&mut self, category: Category) {
        self.update_time();
        self.category = category;
    }

    pub fn get_category (&self) -> &Category {
        &self.category
    }

    pub fn set_tags (&mut self, tags: Vec<Tag>) {
        self.update_time();
        self.tags = tags
    }

    pub fn get_tags (&self) -> &Vec<Tag> {
        &self.tags
    }

    pub fn add_tag (&mut self, tag: Tag) -> Result<(), &str> {
        for item in self.tags.iter() {
            if item.get_title() == tag.get_title() {
                return Err("Tag already exists");
            }
        }

        self.tags.push(tag);

        self.update_time();

        Ok(())
    }

    pub fn remove_tag (&mut self, tag: Tag) {
        self.tags.retain(|item| item.get_title() != tag.get_title());
        self.update_time();
    }

    pub fn set_priority (&mut self, priority: Priority) {
        self.priority = priority;
        self.update_time();
    }

    pub fn get_priority (&self) -> &Priority {
        &self.priority
    }

    pub fn up_priority (&mut self) {
        self.priority = self.priority.clone().up();
    }

    pub fn down_priority (&mut self) {
        self.priority = self.priority.clone().down();
    }

    pub fn get_created_at (&self) -> &DateTime<Utc> {
        &self.created_at
    }

    pub fn get_updated_at (&self) -> &DateTime<Utc> {
        &self.updated_at
    }

    fn update_time (&mut self) {
        self.updated_at = Utc::now();
    }

    pub fn has_deadline (&self) -> bool {
        self.deadline.is_some()
    }

    pub fn set_deadline (&mut self, deadline: DateTime<Utc>) {
        self.deadline = Some(deadline);
    }

    pub fn is_for_today (&self) -> bool {
        if !self.has_deadline() {
            return false;
        }

        self.deadline.unwrap().signed_duration_since(Utc::now()).num_days() == 0
    }
}

impl std::fmt::Display for Task {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}| {} : {}", self.get_priority(), self.get_title(), self.get_category())
    }
}

#[cfg(test)]
mod tests {
    use chrono::{Days, Duration};

    use super::*;

    #[test]
    fn create() {
        let task = Task::new("First").unwrap().build();
        assert_eq!(task.get_title(), "First");
    }

    #[test]
    fn change_title () {
        let mut task = Task::new("First").unwrap().build();
        task.set_title("Second").unwrap();
        assert_eq!(task.get_title(), "Second");
    }

    #[test]
    fn failed_change_title_to_empty () {
        let mut task = Task::new("First").unwrap().build();
        assert!(task.set_title("").is_err());
        assert_eq!(task.get_title(), "First");
    }

    #[test]
    fn change_description () {
        let mut task = Task::new("First").unwrap().build();
        task.set_description("description ...");
        assert_eq!(task.get_description(), "description ...");
    }

    #[test]
    fn change_description_to_empty () {
        let mut task = Task::new("First").unwrap().description("description ...").build();
        task.set_description("");
        assert_eq!(task.get_description(), "");
    }

    #[test]
    fn change_status () {
        let mut task = Task::new("First").unwrap().build();
        task.set_status(TaskStatus::Archived);
        assert_eq!(task.get_status(), &TaskStatus::Archived);
    }

    #[test]
    fn change_category () {
        let mut task = Task::new("First").unwrap().build();
        task.set_category(Category::new("Important").unwrap());
        assert_eq!(task.get_category(), &Category::new("Important").unwrap());
    }

    #[test]
    fn set_tags () {
        let mut task = Task::new("First").unwrap().build();
        task.set_tags(vec![Tag::new("code").unwrap(), Tag::new("Rust").unwrap()]);
        assert_eq!(task.get_tags(), &vec![Tag::new("code").unwrap(), Tag::new("Rust").unwrap()]);
    }

    #[test]
    fn add_tag_to_empty () {
        let mut task = Task::new("First").unwrap().build();
        task.add_tag(Tag::new("Rust").unwrap()).unwrap();
        assert_eq!(task.get_tags(), &vec![Tag::new("Rust").unwrap()]);
    }

    #[test]
    fn add_tag () {
        let mut task = Task::new("First").unwrap()
                                .tags(&vec![Tag::new("Rust").unwrap()])
                                .build();

        task.add_tag(Tag::new("code").unwrap()).unwrap();

        assert_eq!(task.get_tags(), &vec![Tag::new("Rust").unwrap(), Tag::new("code").unwrap()]);
    }

    #[test]
    fn remove_tag () {
        let mut task = Task::new("First").unwrap()
                                .tags(&vec![
                                    Tag::new("code").unwrap(), 
                                    Tag::new("Rust").unwrap()]
                                )
                                .build();

        task.remove_tag(Tag::new("code").unwrap());

        assert_eq!(task.get_tags(), &vec![Tag::new("Rust").unwrap()]);
    }

    #[test]
    fn remove_tag_from_empty () {
        let mut task = Task::new("First").unwrap().build();
        task.remove_tag(Tag::new("code").unwrap());
        assert_eq!(task.get_tags(), &vec![]);
    }

    // priority
    #[test]
    fn set_priority () {
        let mut task = Task::new("First").unwrap().build();
        task.set_priority(Priority::Yellow);
        assert_eq!(task.get_priority(), &Priority::Yellow);
    }

    #[test]
    fn down_and_up_priority () {
        let mut task = Task::new("First").unwrap().build();
        task.set_priority(Priority::Yellow);

        task.down_priority();
        assert_eq!(task.get_priority(), &Priority::Green);

        task.up_priority();
        assert_eq!(task.get_priority(), &Priority::Yellow);
    }

    #[test]
    fn update_time () {
        let mut task = Task::new("First").unwrap().build();
        task.set_priority(Priority::Red);
        assert_ne!(task.get_updated_at(), task.get_created_at());
    }

    #[test]
    fn deadline_has_not_for_init () {
        let task = Task::new("First").unwrap().build();
        assert!(!task.has_deadline());
    }

    #[test]
    fn deadline_has_deadline () {
        let task = Task::new("First").unwrap()
                                .deadline(Utc::now().checked_add_days(Days::new(2)).unwrap())
                                .build();

        assert!(task.has_deadline());
    }

    #[test]
    fn deadline_set_for_today () {
        let task = Task::new("First").unwrap()
                                .deadline(Utc::now().checked_add_signed(Duration::hours(2)).unwrap())
                                .build();

        assert!(task.is_for_today());
    }

    #[test]
    fn deadline_set_for_not_today () {
        let task = Task::new("First").unwrap()
                                .deadline(Utc::now().checked_add_days(Days::new(2)).unwrap())
                                .build();

        assert!(! task.is_for_today());
    }
}