use super::Flag;

impl<'a> Flag<'a> {
    pub fn help_length(&self) -> usize {
        self.long_name.to_string().len()
            + 1
            + if let Some(value) = &self.value {
                value.to_string().len()
            } else {
                5 // Length of "VALUE"
            }
    }
}
