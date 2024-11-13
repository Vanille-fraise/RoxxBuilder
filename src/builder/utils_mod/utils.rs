use serde::de::{Deserializer, MapAccess, Visitor};
use std::collections::HashMap;
use std::fmt;

// Custom deserializer to convert any JSON value to a String
pub fn deserialize_to_string_map<'de, D>(deserializer: D) -> Result<HashMap<String, String>, D::Error>
where
    D: Deserializer<'de>,
{
    struct StringMapVisitor;

    impl<'de> Visitor<'de> for StringMapVisitor {
        type Value = HashMap<String, String>;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a map with string keys and values convertible to strings")
        }

        fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
        where
            M: MapAccess<'de>,
        {
            let mut hashmap = HashMap::new();
            while let Some((key, value)) = map.next_entry::<String, serde_json::Value>()? {
                // Convert all values to strings
                let value_str = match value {
                    serde_json::Value::String(s) => s,
                    serde_json::Value::Number(n) => n.to_string(),
                    serde_json::Value::Bool(b) => b.to_string(),
                    _ => continue, // Skip other types
                };
                hashmap.insert(key, value_str);
            }
            Ok(hashmap)
        }
    }

    deserializer.deserialize_map(StringMapVisitor)
}
