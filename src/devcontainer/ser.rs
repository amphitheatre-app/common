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

/// Serialize the given data structure as a String of JSON.
pub fn to_string<T>(value: &T) -> Result<String, serde_json::Error>
where
    T: serde::Serialize,
{
    serde_json::to_string(value)
}

/// Serialize the given data structure as a pretty-printed String of JSON.
pub fn to_string_pretty<T>(value: &T) -> Result<String, serde_json::Error>
where
    T: serde::Serialize,
{
    serde_json::to_string_pretty(value)
}

/// Convert a T into serde_json::Value which is an enum that can represent any valid JSON data.
pub fn to_value<T>(value: &T) -> Result<serde_json::Value, serde_json::Error>
where
    T: serde::Serialize,
{
    serde_json::to_value(value)
}

/// Serialize the given data structure as a byte vector of JSON.
pub fn to_vec<T>(value: &T) -> Result<Vec<u8>, serde_json::Error>
where
    T: serde::Serialize,
{
    serde_json::to_vec(value)
}

/// Serialize the given data structure as a pretty-printed JSON byte vector.
pub fn to_vec_pretty<T>(value: &T) -> Result<Vec<u8>, serde_json::Error>
where
    T: serde::Serialize,
{
    serde_json::to_vec_pretty(value)
}

/// Serialize the given data structure as JSON into the I/O stream.
pub fn to_writer<W, T>(wr: W, value: &T) -> Result<(), serde_json::Error>
where
    W: std::io::Write,
    T: serde::Serialize,
{
    serde_json::to_writer(wr, value)
}

/// Serialize the given data structure as pretty-printed JSON into the I/O stream.
pub fn to_writer_pretty<W, T>(wr: W, value: &T) -> Result<(), serde_json::Error>
where
    W: std::io::Write,
    T: serde::Serialize,
{
    serde_json::to_writer_pretty(wr, value)
}
