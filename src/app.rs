use crate::list::ListState;

#[derive(Debug, PartialEq)]
pub enum InputMode {
    Normal,
    Insert,
    Command,
}

pub struct App<'a> {
    pub input: String,
    pub input_mode: InputMode,
    title: &'a str,
    tasks: ListState<String>,
}

impl<'a> App<'a> {
    pub fn new(title: &'a str) -> Self {
        App {
            title,
            input: String::new(),
            input_mode: InputMode::Normal,
            tasks: ListState::new(Vec::new()),
        }
    }

    pub fn tasks(&self) -> ListState<String> {
        self.tasks.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_create_a_new_app_with_title() {
        // Arrange
        let title = "Title Test";
        // Act
        let app = App::new(title);
        // Assert
        assert_eq!(app.title, title);
    }

    #[test]
    fn should_create_a_new_app_with_default_input_mode() {
        // Arrange
        let title = "Title Test";
        // Act
        let app = App::new(title);
        // Assert
        assert_eq!(app.input_mode, InputMode::Normal);
    }
}
