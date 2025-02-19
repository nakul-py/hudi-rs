/*
 * Licensed to the Apache Software Foundation (ASF) under one
 * or more contributor license agreements.  See the NOTICE file
 * distributed with this work for additional information
 * regarding copyright ownership.  The ASF licenses this file
 * to you under the Apache License, Version 2.0 (the
 * "License"); you may not use this file except in compliance
 * with the License.  You may obtain a copy of the License at
 *
 *   http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing,
 * software distributed under the License is distributed on an
 * "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
 * KIND, either express or implied.  See the License for the
 * specific language governing permissions and limitations
 * under the License.
 */
use std::num::ParseIntError;
use std::str::ParseBoolError;
use thiserror::Error;

pub type Result<T, E = ConfigError> = std::result::Result<T, E>;

#[derive(Error, Debug)]
pub enum ConfigError {
    /// ParseBool(key, value, source)
    #[error("Failed to parse '{1}' for config '{0}': {2}")]
    ParseBool(String, String, ParseBoolError),

    /// ParseInt(key, value, source)
    #[error("Failed to parse '{1}' for config '{0}': {2}")]
    ParseInt(String, String, ParseIntError),

    /// ParseLine(message)
    #[error("Failed to parse line: {0}'")]
    ParseLine(String),

    #[error("Config value '{0}' is not valid")]
    InvalidValue(String),

    #[error("Config '{0}' not found")]
    NotFound(String),

    #[error("Config value '{0}' is not supported")]
    UnsupportedValue(String),
}
