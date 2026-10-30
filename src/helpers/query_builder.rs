#![allow(clippy::needless_return, clippy::manual_let_else)]

use sqlx::{QueryBuilder, Sqlite};

pub struct SqlQueryBuilder<'a> {
    pub builder: QueryBuilder<'a, Sqlite>,
    pub has_where: bool,
    pub parentheses_level: u64,
}

impl SqlQueryBuilder<'_> {
    pub fn add_condition(&mut self) {
        if !self.has_where {
            self.builder.push(" where ");
            self.has_where = true;
        }
    }
    pub fn add_and(&mut self) {
        if !self.has_where {
            self.builder.push(" where ");
            self.has_where = true;
        } else {
            self.builder.push(" and ");
        }
    }

    pub fn add_or(&mut self) {
        if !self.has_where {
            self.builder.push(" where ");
            self.has_where = true;
        }else {
            self.builder.push(" or ");
        }
    }

    pub fn begin_parentheses(&mut self) {
        if !self.has_where {
            self.builder.push(" where ");
            self.has_where = true;
        }
        self.builder.push("( ");
        self.parentheses_level+=1;
    }

    pub fn end_parentheses(&mut self) {
        if self.parentheses_level <= 0 {
            let sql = self.builder.sql();
            panic!("Mismatched end parentheses in expression: {sql}");
        }
        self.builder.push(" )");
        self.parentheses_level -= 1;
    }

    pub fn new(query: String) -> Self {
        Self {
            builder: QueryBuilder::new(query),
            has_where: false,
            parentheses_level: 0,
        }
    }
}
