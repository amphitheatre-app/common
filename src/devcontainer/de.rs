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

use json_comments::StripComments;
use serde::de;

/// Deserialize an instance of type T from an I/O stream of JSON.
pub fn from_reader<R, T>(rdr: R) -> Result<T, serde_json::Error>
where
    R: std::io::Read,
    T: de::DeserializeOwned,
{
    let stripped = StripComments::new(rdr);
    serde_json::from_reader(stripped)
}

/// Deserialize an instance of type T from bytes of JSON text.
pub fn from_slice<T>(slice: &[u8]) -> Result<T, serde_json::Error>
where
    T: de::DeserializeOwned,
{
    let stripped = StripComments::new(slice);
    serde_json::from_reader(stripped)
}

/// Deserialize an instance of type `T` from a string of JSON text.
pub fn from_str<T>(s: &str) -> Result<T, serde_json::Error>
where
    T: de::DeserializeOwned,
{
    let stripped = StripComments::new(s.as_bytes());
    serde_json::from_reader(stripped)
}

/// Interpret a serde_json::Value as an instance of type T.
pub fn from_value<T>(value: serde_json::Value) -> Result<T, serde_json::Error>
where
    T: de::DeserializeOwned,
{
    serde_json::from_value(value)
}

#[cfg(test)]
mod test {
    use serde_json::json;

    #[test]
    fn test_from_reader() {
        let json = r#"{
            "foo": "bar" // This is a comment.
        }"#;
        let result: serde_json::Value = super::from_reader(json.as_bytes()).unwrap();
        assert_eq!(result["foo"], "bar");
    }

    #[test]
    fn test_from_slice() {
        let json = r#"{
            // This is a comment.
            "foo": "bar"
        }"#;
        let result: serde_json::Value = super::from_slice(json.as_bytes()).unwrap();
        assert_eq!(result["foo"], "bar");
    }

    #[test]
    fn test_from_str() {
        let json = r#"{
            // This is a comment.
            "foo": "bar"
        }"#;
        let result: serde_json::Value = super::from_str(json).unwrap();
        assert_eq!(result["foo"], "bar");
    }

    #[test]
    fn test_from_value() {
        let json = json!({"foo": "bar"});
        let result: serde_json::Value = super::from_value(json).unwrap();
        assert_eq!(result["foo"], "bar");
    }
}
