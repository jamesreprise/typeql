/*
 * Copyright (C) 2022 Vaticle
 *
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
 *
 */

use std::fmt;

use crate::common::error::{SYNTAX_ERROR_DETAILED, SYNTAX_ERROR_NO_DETAILS};

pub struct SyntaxError {
    pub query_line: Option<String>,
    pub line: usize,
    pub char_position_in_line: usize,
    pub message: String,
}

impl fmt::Display for SyntaxError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(query_line) = &self.query_line {
            // Error message appearance:
            //
            // syntax error at line 1:
            // match $
            //       ^
            // blah blah antlr blah
            f.write_str(
                &SYNTAX_ERROR_DETAILED
                    .format(&[
                        self.line.to_string().as_str(),
                        query_line,
                        &(" ".repeat(self.char_position_in_line) + "^"),
                        &self.message,
                    ])
                    .message,
            )
        } else {
            f.write_str(
                &SYNTAX_ERROR_NO_DETAILS
                    .format(&[self.line.to_string().as_str(), &self.message])
                    .message,
            )
        }
    }
}
