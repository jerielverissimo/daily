#[derive(Debug, PartialEq)]
enum InputMode {
    Normal,
    Insert,
    Command,
}

pub struct App<'a> {
    title: &'a str,
    input: String,
    input_mode: InputMode,
    tasks: Vec<String>,
}

impl<'a> App<'a> {
    pub fn new(title: &'a str) -> Self {
        App {
            title,
            input: String::new(),
            input_mode: InputMode::Normal,
            tasks: Vec::new(),
        }
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
