use sqlx::{QueryBuilder, Sqlite};

pub struct SqlQueryBuilder<'a> {
    pub builder: QueryBuilder<'a, Sqlite>,
    pub has_where: bool,
    pub has_multi_filter: bool,
}

impl SqlQueryBuilder<'_> {
    pub fn add_condition(&mut self) {
        if !self.has_where {
            self.builder.push(" where ");
            self.has_where = true;
        } else {
            self.builder.push(" and ");
        }
    }

    pub fn begin_multi_filter(&mut self) {
        self.builder.push("( ");
        self.has_multi_filter = false;
    }

    pub fn add_multi_filter(&mut self) {
        if self.has_multi_filter {
            self.builder.push(" or ");
        }
        self.has_multi_filter = true;
    }

    pub fn end_multi_filter(&mut self) {
        self.builder.push(" )");
        self.has_multi_filter = false;
    }

    pub fn new(query: String) -> Self {
        Self {
            builder: QueryBuilder::new(query),
            has_where: false,
            has_multi_filter: false,
        }
    }
}
