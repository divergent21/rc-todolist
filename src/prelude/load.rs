use super::{Category, Task, Tag};

pub trait TodoListLoad {
    fn get_todolist_categories (&self) -> Vec<Category>;
    fn get_todolist_tasks (&self) -> Vec<Task>;
    fn get_todolist_tags (&self) -> Vec<Tag>;
}