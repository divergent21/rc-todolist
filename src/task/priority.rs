use std::cmp::{PartialEq, PartialOrd};

#[derive(PartialEq, PartialOrd, Debug, Clone, Copy)]
pub enum Priority {
    Common,
    Green,
    Yellow,
    Red
}


impl Priority {
    pub fn up (self) -> Self {
        match self {
            Self::Common => Self::Green,
            Self::Green => Self::Yellow,
            _ => Self::Red,
        }
    }

    pub fn down (self) -> Self {
        match self {
            Self::Red => Self::Yellow,
            Self::Yellow => Self::Green,
            _ => Self::Common
        }
    }
}

impl Default for Priority {
    fn default() -> Self {
        Priority::Common
    }
}

impl std::fmt::Display for Priority {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Self::Common => "Common",
            Self::Green => "Green",
            Self::Yellow => "Yellow",
            Self::Red => "Red"
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn default_priority () {
        let priority = Priority::default();
        assert_eq!(priority, Priority::Common);
    }

    #[test]
    fn order () {
        assert!(Priority::Common < Priority::Green);
        assert!(Priority::Green < Priority::Yellow);
        assert!(Priority::Yellow < Priority::Red);
    }

    #[test]
    fn up_common () {
        let priority = Priority::Common;
        assert_eq!(priority.up(), Priority::Green);
    }

    #[test]
    fn up_green () {
        let priority = Priority::Green;
        assert_eq!(priority.up(), Priority::Yellow);
    }

    #[test]
    fn up_yellow () {
        let priority = Priority::Yellow;
        assert_eq!(priority.up(), Priority::Red);
    }

    #[test]
    fn up_red () {
        let priority = Priority::Red;
        assert_eq!(priority.up(), Priority::Red);
    }

    #[test]
    fn down_common () {
        let priority = Priority::Common;
        assert_eq!(priority.down(), Priority::Common);
    }

    #[test]
    fn down_green () {
        let priority = Priority::Green;
        assert_eq!(priority.down(), Priority::Common);
    }

    #[test]
    fn down_yellow () {
        let priority = Priority::Yellow;
        assert_eq!(priority.down(), Priority::Green);
    }

    #[test]
    fn down_red () {
        let priority = Priority::Red;
        assert_eq!(priority.down(), Priority::Yellow);
    }
}