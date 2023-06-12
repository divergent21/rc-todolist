pub mod prelude;
mod task;

use prelude::*;
use prelude::load::TodoListLoad;

pub enum By<'a> {
    Category(&'a Category),
    Tag(&'a Tag),
    Priority(Priority),
    Status(Status)
}

pub struct TodoList {
    tasks: Vec<Task>,
    categories: Vec<Category>,
    tags: Vec<Tag>
}

impl TodoList {
    pub fn init (loader: impl TodoListLoad) -> Self {
        Self {
            tasks: loader.get_todolist_tasks(),
            categories: loader.get_todolist_categories(),
            tags: loader.get_todolist_tags()
        }
    }

    pub fn get_tasks (&self) -> &Vec<Task> {
        &self.tasks
    }

    pub fn new_task (&mut self, title: &str, description: &str) -> Result<(), &str> {
        for item in self.tasks.iter() {
            if item.get_title() == title {
                return Err("A task with same title is already exists");
            }
        }

        self.tasks.push(Task::new(title, description)?);

        Ok(())
    }

    pub fn remove_task (&mut self, task: Task) -> Result<(), &str> {
        if ! self.tasks.contains(&task) {
            return Err("THe task is not exists");
        }

        self.tasks.retain(|item| item.get_title() != item.get_title());

        Ok(())
    }

    pub fn get_categories (&self) -> &Vec<Category> {
        &self.categories
    }

    pub fn new_category (&mut self, title: &str) -> Result<(), &str> {
        for item in self.categories.iter() {
            if item.get_title() == title {
                return Err("The category with same title is already exists");
            }
        }

        self.categories.push(Category::new(title)?);

        Ok(())
    }

    pub fn get_tags (&self) -> &Vec<Tag> {
        &self.tags
    }

    pub fn new_tag (&mut self, title: &str) -> Result<(), &str> {
        let tag = Tag::new(title)?;

        if self.tags.contains(&tag) {
            return Err("The tag is already exists");
        }

        self.tags.push(tag);

        Ok(())
    }

    pub fn get_tasks_by (&self, by: By) -> Vec<&Task> {
        match by {
            By::Category(category) => self.get_tasks_by_category(category),
            By::Priority(priority) => self.get_tasks_by_priority(priority),
            By::Status(status) => self.get_tasks_by_status(status),
            By::Tag(tag) => self.get_tasks_by_tag(tag)
        }
    }

    fn get_tasks_by_category (&self, category: &Category) -> Vec<&Task> {
        self.tasks.iter().filter(|task| task.get_category().get_title() == category.get_title()).collect()
    }

    fn get_tasks_by_priority (&self, priority: Priority) -> Vec<&Task> {
        self.tasks.iter().filter(|task| task.get_priority() == &priority).collect()
    }

    fn get_tasks_by_tag (&self, tag: &Tag) -> Vec<&Task> {
        self.tasks.iter().filter(|task| task.get_tags().contains(tag)).collect()
    }

    fn get_tasks_by_status (&self, status: Status) -> Vec<&Task> {
        self.tasks.iter().filter(|task| task.get_status() == &status).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestTodoListLoader;
    impl TodoListLoad for TestTodoListLoader {
        fn get_todolist_categories (&self) -> Vec<Category> {
            vec![]
        }

        fn get_todolist_tags (&self) -> Vec<Tag> {
            vec![]
        }

        fn get_todolist_tasks (&self) -> Vec<Task> {
            vec![]
        }
    }

    #[test]
    fn empty_tasks_list_after_init () {
        let todolist = TodoList::init(TestTodoListLoader);
        assert!(todolist.get_tasks().is_empty());
    }

    #[test]
    fn empty_categories_after_init () {
        let todolist = TodoList::init(TestTodoListLoader);
        assert!(todolist.get_categories().is_empty());
    }
}