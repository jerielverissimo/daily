#[derive(Debug, Clone)]
pub struct ListState<I> {
    pub items: Vec<I>,
    pub selected: usize,
}

impl<I> ListState<I> {
    pub fn new(items: Vec<I>) -> ListState<I> {
        ListState { items, selected: 0 }
    }

    pub fn select_next(&mut self) {
        if self.selected < self.items.len() - 1 {
            self.selected += 1;
        }
    }

    pub fn select_previous(&mut self) {
        if self.selected > 0 {
            self.selected -= 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_create_a_new_list_state_with_selected_zero() {
        // Act
        let t_list_state = ListState::new(vec!["first", "second", "third"]);
        // Assert
        assert_eq!(t_list_state.selected, 0);
    }

    #[test]
    fn should_change_selected_to_one_when_select_previous_is_called() {
        // Arrange
        let mut t_list_state = ListState::new(vec!["first", "second", "third"]);
        // Act
        t_list_state.select_next();
        t_list_state.select_next();
        t_list_state.select_previous();
        // Assert
        assert_eq!(t_list_state.selected, 1);
    }

    #[test]
    fn should_change_selected_when_select_next_is_called() {
        // Arrange
        let mut t_list_state = ListState::new(vec!["first", "second", "third"]);
        // Act
        t_list_state.select_next();
        // Assert
        assert_eq!(t_list_state.selected, 1);
    }
}
