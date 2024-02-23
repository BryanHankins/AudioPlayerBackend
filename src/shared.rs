use std::collections::VecDeque;

/// Processes the given input string, splitting it into words and returning
/// a VecDeque containing each word.
///
/// # Arguments
///
/// * `input` - A string slice that holds the input string to be processed.
///
/// # Returns
///
/// A VecDeque containing each word from the input string.
pub fn process_data_with_vecdeque(input: &str) -> VecDeque<String> {
    let mut deque = VecDeque::new();

    // Split the input string on whitespace and enqueue each word
    input.split_whitespace().for_each(|word| {
        deque.push_back(word.to_string());
    });

    deque
}
