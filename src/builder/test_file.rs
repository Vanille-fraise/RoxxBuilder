/*
use std::cmp::min;

struct TestStruct {
    data: Vec<String>,
}

impl TestStruct {
    pub fn print_first(&self, before: usize) {
        for i in 0..min(before, self.data.len()) {
            println!("{}", self.data[i]);
        }
    }
}

fn multi_thread() {
    let data = vec!["a".to_string(), "b".to_string(), "c".to_string(), "d".to_string(), "e".to_string()];
    let test_struct = TestStruct { data };
    for i in 0..5 {
        std::thread::spawn(move || {&test_struct.print_first(3);});
    }
}*/