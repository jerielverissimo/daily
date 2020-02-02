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
