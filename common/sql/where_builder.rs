use tokio_postgres::types::ToSql;

pub struct NamedParam<'a> {
    name: &'a str,
    param: &'a (dyn ToSql + Sync),
}

#[derive(Default)]
pub struct WhereBuilder<'a> {
    params: Vec<NamedParam<'a>>,
    offset: Option<usize>,
    limit: Option<usize>,
}

impl<'a> WhereBuilder<'a> {
    pub fn new() -> Self {
        WhereBuilder {
            params: Vec::new(),
            offset: None,
            limit: None,
        }
    }

    pub fn add_param(self, k: &'a str, v: &'a (dyn ToSql + Sync)) -> Self {
        self.add_param_opt(k, v, true)
    }

    pub fn add_param_opt(mut self, k: &'a str, v: &'a (dyn ToSql + Sync), add: bool) -> Self {
        if add {
            self.params.push(NamedParam { name: k, param: v });
        }
        self
    }

    pub fn offset(mut self, offset: Option<usize>) -> Self {
        self.offset = offset;
        self
    }

    pub fn limit(mut self, limit: Option<usize>) -> Self {
        self.limit = limit;
        self
    }

    pub fn build(self) -> (String, Vec<&'a (dyn ToSql + Sync)>) {
        if self.params.is_empty() {
            return ("".to_owned(), Vec::new());
        }

        let mut statements = Vec::new();
        let mut params = Vec::new();

        for p in self.params.into_iter() {
            let param_index = format!("${}", params.len() + 1);
            let statement = p.name.replace("$$", &param_index);
            statements.push(statement);
            params.push(p.param);
        }

        let mut sql = if statements.len() > 0 {
            format!("WHERE {}", statements.join(" AND "))
        } else {
            "".to_owned()
        };

        if let Some(offset) = self.offset {
            sql = format!(
                "{}
                OFFSET {}",
                sql, offset,
            );
        }

        if let Some(limit) = self.limit {
            sql = format!(
                "{}
                LIMIT {}",
                sql, limit,
            );
        }

        (sql, params)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn null_params() {
        let null: Option<&str> = None;
        let (sql, params) = WhereBuilder::new()
            .add_param("param_1 = $$", &null)
            .add_param("param_2 = $$", &Some("not_null"))
            .add_param("param_3 = $$", &Some(3))
            .add_param("param_4 >= $$", &3)
            .add_param("param_5 like '%$$%'", &"string")
            .build();

        assert_eq!(sql, "WHERE param_1 = $1 AND param_2 = $2 AND param_3 = $3 AND param_4 >= $4 AND param_5 like '%$5%'");
        assert_eq!(params.len(), 5);
    }

    #[test]
    fn without_null_params() {
        let null: Option<&str> = None;
        let (sql, params) = WhereBuilder::new()
            .add_param_opt("param_1 = $$", &null, false)
            .add_param_opt("param_2 = $$", &Some("not_null"), true)
            .add_param_opt("param_3 = $$", &Some(3), false)
            .add_param_opt("param_4 >= $$", &3, true)
            .add_param_opt("param_5 like '%$$%'", &"string", true)
            .build();

        assert_eq!(
            sql,
            "WHERE param_2 = $1 AND param_4 >= $2 AND param_5 like '%$3%'"
        );
        assert_eq!(params.len(), 3);
    }
}
