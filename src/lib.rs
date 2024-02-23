// This is my webassembly part of my project , It is suppose to connect to my other project , Which is the front end part , Which is named rustwebserverfrontend
use wasm_bindgen::prelude::*;
use std::collections::VecDeque;

#[wasm_bindgen]
pub fn add_and_fetch_first(values: Vec<i32>) -> i32 {
    let mut deque: VecDeque<i32> = VecDeque::new();
    
    for value in values {
        deque.push_back(value);
    }
    
    match deque.pop_front() {
        Some(val) => val,
        None => 0, // Return a default value or handle this case as needed
    }
}

// // Something I threw together while high 
// asyncify.start_unwind {
//     asyncify.stop_unwind 
//     // call in the button that when its clicked or loaded when it starts, It plays
//     // then when 
//     Asyncify.start_rewind then asyncify.stop_unwind when button.onclick(showbyid("rewind")) 

//     Onclick.(showelementbyid("load").loadapplication){
//         asyncify.stop_unwind 

//         Asyncify.start_rewind then asyncify.stop_unwind when button.onclick(showbyid("rewind")) 
//     }
// }