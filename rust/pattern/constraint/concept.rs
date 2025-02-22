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

use crate::{common::token::Constraint::Is, pattern::*, var};
use std::fmt;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct IsConstraint {
    variable: Box<ConceptVariable>,
}

impl From<&str> for IsConstraint {
    fn from(string: &str) -> Self {
        Self::from(var(string))
    }
}
impl From<String> for IsConstraint {
    fn from(string: String) -> Self {
        Self::from(var(string))
    }
}

impl From<UnboundVariable> for IsConstraint {
    fn from(var: UnboundVariable) -> Self {
        Self::from(var.into_concept())
    }
}

impl From<ConceptVariable> for IsConstraint {
    fn from(var: ConceptVariable) -> Self {
        Self { variable: Box::new(var) }
    }
}

impl fmt::Display for IsConstraint {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", Is, self.variable)
    }
}
