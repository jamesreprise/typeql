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

use crate::{
    common::token::{Aggregate, Aggregate::Count},
    Query, TypeQLMatch, TypeQLMatchGroup, UnboundVariable,
};
use std::fmt;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AggregateQuery<T>
where
    T: AggregateQueryBuilder,
{
    pub query: T,
    pub method: Aggregate,
    pub var: Option<UnboundVariable>,
}

pub type TypeQLMatchAggregate = AggregateQuery<TypeQLMatch>;
pub type TypeQLMatchGroupAggregate = AggregateQuery<TypeQLMatchGroup>;

impl<T: AggregateQueryBuilder> AggregateQuery<T> {
    pub fn new_count(base: T) -> Self {
        Self { query: base, method: Count, var: None }
    }

    pub fn new(base: T, method: Aggregate, var: UnboundVariable) -> Self {
        Self { query: base, method, var: Some(var) }
    }
}

impl TypeQLMatchAggregate {
    pub fn into_query(self) -> Query {
        Query::Aggregate(self)
    }
}

impl TypeQLMatchGroupAggregate {
    pub fn into_query(self) -> Query {
        Query::GroupAggregate(self)
    }
}

impl fmt::Display for TypeQLMatchAggregate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}\n{}", self.query, self.method)?;
        if let Some(var) = &self.var {
            write!(f, " {}", var)?;
        }
        f.write_str(";")
    }
}

impl fmt::Display for TypeQLMatchGroupAggregate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.query, self.method)?;
        if let Some(var) = &self.var {
            write!(f, " {}", var)?;
        }
        f.write_str(";")
    }
}

pub trait AggregateQueryBuilder:
    Sized + Clone + fmt::Display + fmt::Debug + Eq + PartialEq
{
    fn count(self) -> AggregateQuery<Self> {
        AggregateQuery::<Self>::new_count(self)
    }

    fn aggregate(self, method: Aggregate, var: UnboundVariable) -> AggregateQuery<Self> {
        AggregateQuery::<Self>::new(self, method, var)
    }

    fn max(self, var: impl Into<UnboundVariable>) -> AggregateQuery<Self> {
        self.aggregate(Aggregate::Max, var.into())
    }

    fn min(self, var: impl Into<UnboundVariable>) -> AggregateQuery<Self> {
        self.aggregate(Aggregate::Min, var.into())
    }

    fn mean(self, var: impl Into<UnboundVariable>) -> AggregateQuery<Self> {
        self.aggregate(Aggregate::Mean, var.into())
    }

    fn median(self, var: impl Into<UnboundVariable>) -> AggregateQuery<Self> {
        self.aggregate(Aggregate::Median, var.into())
    }

    fn std(self, var: impl Into<UnboundVariable>) -> AggregateQuery<Self> {
        self.aggregate(Aggregate::Std, var.into())
    }

    fn sum(self, var: impl Into<UnboundVariable>) -> AggregateQuery<Self> {
        self.aggregate(Aggregate::Sum, var.into())
    }
}
