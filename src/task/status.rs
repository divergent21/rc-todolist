use std::cmp::{PartialEq, Eq};

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum TaskStatus {
    Created,
    Progress,
    Completed,
    Archived
}

impl Default for TaskStatus {
    fn default() -> Self {
        TaskStatus::Created
    }
}

impl std::fmt::Display for TaskStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Self::Created => "Created",
            Self::Progress => "Progress",
            Self::Completed => "Completed",
            Self::Archived => "Archived"
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_status () {
        let status = TaskStatus::default();
        assert_eq!(status, TaskStatus::Created);
    }

    #[test]
    fn equals_between_status () {
        let created = TaskStatus::Created;
        let created2 = TaskStatus::Created;

        assert_eq!(created, created2);
    }

    #[test]
    fn not_eq_between_status () {
        let created = TaskStatus::Created;
        let archived = TaskStatus::Archived;

        assert_ne!(created, archived);
    }
}