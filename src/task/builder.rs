use super::{TaskStatus, Category, Priority, Tag, Task, DateTime, Utc, Uuid};

pub struct TaskBuilder {
    title: String,
    description: Option<String>,

    status: Option<TaskStatus>,

    category: Option<Category>,
    tags: Option<Vec<Tag>>,

    priority: Option<Priority>,

    deadline: Option<DateTime<Utc>>
}

impl TaskBuilder {
    pub fn description(&mut self, description: &str) -> &mut Self {
        self.description = Some(description.to_owned());
        self
    }

    pub fn status(&mut self, status: TaskStatus) -> &mut Self {
        self.status = Some(status);
        self
    }

    pub fn category(&mut self, category: Category) -> &mut Self {
        self.category = Some(category);
        self
    }

    pub fn tags(&mut self, tags: &[Tag]) -> &mut Self {
        self.tags = Some(tags.to_vec());
        self
    }

    pub fn priority(&mut self, priority: Priority) -> &mut Self {
        self.priority = Some(priority);
        self
    }

    pub fn deadline(&mut self, deadline: DateTime<Utc>) -> &mut Self {
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