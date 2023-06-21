use super::{TaskStatus, Category, Priority, Tag, Task, DateTime, Utc, Uuid};

pub struct TaskBuilder {
    pub(super) title: String,
    pub(super) description: Option<String>,

    pub(super) status: Option<TaskStatus>,

    pub(super) category: Option<Category>,
    pub(super) tags: Option<Vec<Tag>>,

    pub(super) priority: Option<Priority>,

    pub(super) deadline: Option<DateTime<Utc>>
}

impl TaskBuilder {
    pub fn description(mut self, description: &str) -> Self {
        self.description = Some(description.to_owned());
        self
    }

    pub fn status(mut self, status: TaskStatus) -> Self {
        self.status = Some(status);
        self
    }

    pub fn category(mut self, category: Category) -> Self {
        self.category = Some(category);
        self
    }

    pub fn tags(mut self, tags: &[Tag]) -> Self {
        self.tags = Some(tags.to_vec());
        self
    }

    pub fn priority(mut self, priority: Priority) -> Self {
        self.priority = Some(priority);
        self
    }

    pub fn deadline(mut self, deadline: DateTime<Utc>) -> Self {
        self.deadline = Some(deadline);
        self
    }

    pub fn build (self) -> Task {
        Task {
            id: Uuid::new_v4(),
            title: self.title,
            description: self.description.unwrap_or_default(),
            status: self.status.unwrap_or_default(),
            category: self.category.unwrap_or_default(),
            tags: self.tags.unwrap_or_default(),
            priority: self.priority.unwrap_or_default(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
            deadline: self.deadline
        }
    }
}