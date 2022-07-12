use serde::{Deserialize, Serialize};


#[derive(PartialEq, Eq, Deserialize, Serialize, Debug)]
struct TestJson {
    id: usize,
    name: String,
}

#[test]
fn small_json_test() {
    let json_str = "{\"id\":10,\"name\":\"testwork\",\"notpresent\":false}";
    let parsed: TestJson = serde_json::from_str(json_str).unwrap();
    assert_eq!("TestJson { id: 10, name: \"testwork\" }", format!("{:?}", parsed));
}
