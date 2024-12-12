use std::{collections::HashMap};

// Key-Value format:
// key=value
// key2 = value2
// key3 = value3 - Comment
#[derive(Debug, Clone)]
pub struct KeyValue {
    data: Vec<(String, String)>
}

impl KeyValue {
    pub fn new() -> KeyValue {
        KeyValue {
            data: Vec::new()
        }
    }

    pub fn from(serialized: &str) -> KeyValue {
        let mut data = Vec::new();
        for line in serialized.lines() {
            let line = line.trim();
            if line.is_empty() {
                continue;
            }
            let mut parts = line.splitn(2, '=');
            let key = parts.next().unwrap().trim();
            let value = parts.next().unwrap_or("").trim();

            // Ignore comments ("xyz - Comment" -> "xyz", the "-" symbol is the beginning of a comment)
            if let Some(comment_start) = value.find('-') {
                let value = &value[..comment_start].trim();
                data.push((key.to_string(), value.to_string()));
            }
            else {
                data.push((key.to_string(), value.to_string()));
            }
        }
        KeyValue {
            data
        }
    }

    pub fn as_str(&self) -> String {
        // concat, not push
        let mut result = String::new();
        for (key, value) in self.data.iter() {
            result = result + key + "=" + value + "\n";
        }

        result
    }

    pub fn insert(&mut self, key: String, value: String) {
        self.data.push((key, value));
    }

    pub fn get(&self, key: &str) -> Option<String> {
        for (k, v) in self.data.iter() {
            if k == key {
                return Some(v.clone());
            }
        }
        None
    }

    pub fn remove(&mut self, key: &str) {
        self.data.retain(|(k, _)| k != key);
    }
}

impl Iterator for KeyValue {
    type Item = (String, String);

    fn next(&mut self) -> Option<Self::Item> {
        if self.data.is_empty() {
            return None;
        }
        Some(self.data.remove(0))
    }
}

#[cfg(test)]
mod key_value_tests {
    use super::KeyValue;

    #[test]
    fn test_key_value() {
        let mut kv = KeyValue::new();
        kv.insert("key1".to_string(), "value1".to_string());
        kv.insert("key2".to_string(), "value2".to_string());
        kv.insert("key3".to_string(), "value3".to_string());
        kv.insert("key4".to_string(), "value4".to_string());
        kv.insert("key5".to_string(), "value5".to_string());

        assert_eq!(kv.get("key1"), Some("value1".to_string()));
        assert_eq!(kv.get("key2"), Some("value2".to_string()));
        assert_eq!(kv.get("key3"), Some("value3".to_string()));
        assert_eq!(kv.get("key4"), Some("value4".to_string()));
        assert_eq!(kv.get("key5"), Some("value5".to_string()));

        kv.remove("key1");
        kv.remove("key2");
        kv.remove("key3");
        kv.remove("key4");
        kv.remove("key5");

        assert_eq!(kv.get("key1"), None);
        assert_eq!(kv.get("key2"), None);
        assert_eq!(kv.get("key3"), None);
        assert_eq!(kv.get("key4"), None);
        assert_eq!(kv.get("key5"), None);
    }

    #[test]
    fn test_key_value_from() {
        let bytes = r#"key1=value1
key2= value2
key3 =value3
key4 = value4
key5 = value5 - Comment
"#;

        let kv = KeyValue::from(bytes);
        assert_eq!(kv.get("key1"), Some
            ("value1".to_string()));
        assert_eq!(kv.get("key2"), Some
            ("value2".to_string()));
        assert_eq!(kv.get("key3"), Some
            ("value3".to_string()));
        assert_eq!(kv.get("key4"), Some
            ("value4".to_string()));
        assert_eq!(kv.get("key5"), Some
            ("value5".to_string()));
    }

    #[test]
    fn test_key_value_as_str() {
        let mut kv = KeyValue::new();

        let reference_bytes = "";

        assert_eq!(kv.as_str(), reference_bytes);

        kv.insert("key1".to_string(), "value1".to_string());


        let reference_bytes = r#"key1=value1
"#;

        assert_eq!(kv.as_str(), reference_bytes);
    }        
}