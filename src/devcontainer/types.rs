// Copyright (c) The Amphitheatre Authors. All rights reserved.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      https://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum IntegerOrString {
    Integer(i32),
    String(String),
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum StringOrArray {
    String(String),
    Array(Vec<String>),
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum IntegerOrStringOrArray {
    Integer(u32),
    String(String),
    Array(Vec<IntegerOrString>),
}

#[cfg(test)]
mod test {
    use super::IntegerOrString;
    use super::IntegerOrStringOrArray;
    use super::StringOrArray;

    #[test]
    fn test_integer_or_string() {
        use serde_json::json;

        let json = json!(42);
        let integer: IntegerOrString = serde_json::from_value(json).unwrap();
        assert_eq!(integer, IntegerOrString::Integer(42));

        let json = json!("42");
        let string: IntegerOrString = serde_json::from_value(json).unwrap();
        assert_eq!(string, IntegerOrString::String("42".to_string()));
    }

    #[test]
    fn test_string_or_array() {
        use serde_json::json;

        let json = json!("42");
        let string: StringOrArray = serde_json::from_value(json).unwrap();
        assert_eq!(string, StringOrArray::String("42".to_string()));

        let json = json!(["42", "43"]);
        let array: StringOrArray = serde_json::from_value(json).unwrap();
        assert_eq!(
            array,
            StringOrArray::Array(vec!["42".to_string(), "43".to_string()])
        );
    }

    #[test]
    fn test_integer_or_string_or_array() {
        use serde_json::json;

        let json = json!(42);
        let integer: IntegerOrStringOrArray = serde_json::from_value(json).unwrap();
        assert_eq!(integer, IntegerOrStringOrArray::Integer(42));

        let json = json!("42");
        let string: IntegerOrStringOrArray = serde_json::from_value(json).unwrap();
        assert_eq!(string, IntegerOrStringOrArray::String("42".to_string()));

        let json = json!([42, 432]);
        let array: IntegerOrStringOrArray = serde_json::from_value(json).unwrap();
        assert_eq!(
            array,
            IntegerOrStringOrArray::Array(vec![IntegerOrString::Integer(42), IntegerOrString::Integer(432)])
        );

        let json = json!(["42", "43"]);
        let array: IntegerOrStringOrArray = serde_json::from_value(json).unwrap();
        assert_eq!(
            array,
            IntegerOrStringOrArray::Array(vec![
                IntegerOrString::String("42".to_string()),
                IntegerOrString::String("43".to_string())
            ])
        );
    }
}
