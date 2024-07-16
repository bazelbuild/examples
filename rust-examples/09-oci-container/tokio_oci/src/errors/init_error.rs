// Copyright 2024 The Bazel examples and tutorials Authors & Contributors. // All rights reserved.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//    http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.


use std::error::Error;
use std::fmt;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone)]
pub struct InitError(pub String);

impl From<&str> for InitError {
    fn from(field0: &str) -> Self {
        Self(field0.to_string())
    }
}

impl From<String> for InitError {
    fn from(field0: String) -> Self {
        Self(field0)
    }
}

impl Display for InitError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "InitError: {}", self.0)
    }
}

impl Error for InitError {}
