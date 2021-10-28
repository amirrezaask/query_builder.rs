enum ClauseType {
    Where,
    Limit,
    Offset,
    OrderBy,
    GroupBy,
    InnerJoin,
    RightJoin,
    LeftJoin,
    FullOuterJoin,
    Select,
    Having,
}

impl std::fmt::Display for ClauseType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ClauseType::Where => write!(f, "WHERE"),
            ClauseType::Limit => write!(f, "LIMIT"),
            ClauseType::Offset => write!(f, "OFFSET"),
            ClauseType::OrderBy => write!(f, "ORDER BY"),
            ClauseType::GroupBy => write!(f, "GROUP BY"),
            ClauseType::InnerJoin => write!(f, "INNER JOIN"),
            ClauseType::RightJoin => write!(f, "RIGHT JOIN"),
            ClauseType::LeftJoin => write!(f, "LEFT JOIN"),
            ClauseType::FullOuterJoin => write!(f, "FULL OUTER JOIN"),
            ClauseType::Select => write!(f, "SELECT"),
            ClauseType::Having => write!(f, "HAVING"),
        }
    }
}
struct Clause {
    ty: ClauseType,
    arg: Vec<String>,
    delimiter: String,
}

impl std::fmt::Display for Clause {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if self.delimiter == "" {
            self.delimiter = " ".to_string();
        }
        write!(f, "{} {}", self.ty, self.arg.join(&self.delimiter))
    }
}
struct SelectStmt {
    table: String,
    selected: Option<Clause>,
    _where: Option<Clause>,
    order_by: Option<Clause>,
    group_by: Option<Clause>,
    joins: Option<Vec<Clause>>,
    limit: Option<Clause>,
    offset: Option<Clause>,
    having: Option<Clause>,
}

impl SelectStmt {
    pub fn limit(&mut self, n: usize) -> &mut Self {
        self.limit = Some(Clause {
            ty: ClauseType::Limit,
            arg: vec![format!("{}", n)],
            delimiter: "".to_string(),
        });
        self
    }
    pub fn offset(&mut self, n: usize) -> &mut Self {
        self.limit = Some(Clause {
            ty: ClauseType::Offset,
            arg: vec![format!("{}", n)],
            delimiter: "".to_string(),
        });
        self
    }
    pub fn skip(&mut self, n: usize) -> &mut Self {
        self.offset(n)
    }
    pub fn take(&mut self, n: usize) -> &mut Self {
        self.limit(n)
    }
    pub fn inner_join(&mut self, table: String, cond: String) -> &mut Self {
        let args = vec![table, "ON".to_string(), cond];
        if self.joins.is_none() {
            self.joins = Some(Vec::new());
        }
        self.joins.unwrap().push(Clause {
            ty: ClauseType::InnerJoin,
            arg: args,
            delimiter: "".to_string(),
        });
        self
    }
    pub fn left_join(&mut self, table: String, cond: String) -> &mut Self {
        let args = vec![table, "ON".to_string(), cond];
        if self.joins.is_none() {
            self.joins = Some(Vec::new());
        }
        self.joins.unwrap().push(Clause {
            ty: ClauseType::LeftJoin,
            arg: args,
            delimiter: "".to_string(),
        });
        self
    }
    pub fn right_join(&mut self, table: String, cond: String) -> &mut Self {
        let args = vec![table, "ON".to_string(), cond];
        if self.joins.is_none() {
            self.joins = Some(Vec::new());
        }
        self.joins.unwrap().push(Clause {
            ty: ClauseType::RightJoin,
            arg: args,
            delimiter: "".to_string(),
        });
        self
    }
    pub fn full_outer_join(&mut self, table: String, cond: String) -> &mut Self {
        let args = vec![table, "ON".to_string(), cond];
        if self.joins.is_none() {
            self.joins = Some(Vec::new());
        }
        self.joins.unwrap().push(Clause {
            ty: ClauseType::FullOuterJoin,
            arg: args,
            delimiter: "".to_string(),
        });
        self
    }
    pub fn order_by(&mut self, col: String, order: String) -> &mut Self {
        if self.order_by.is_none() {
            self.order_by = Some(Clause {
                ty: ClauseType::OrderBy,
                arg: vec![format!("{} {}", col, order)],
                delimiter: ", ".to_string(),
            });
            return self;
        }
        self.order_by
            .unwrap()
            .arg
            .push(format!("{} {}", col, order));
        self
    }
    pub fn having(&mut self, cond: String) -> &mut Self {
        if self.having.is_none() {
            self.having = Some(Clause {
                ty: ClauseType::Having,
                arg: vec![cond],
                delimiter: "".to_string(),
            });
            return self;
        }
        self.having.unwrap().arg.push("AND".to_string());
        self.having.unwrap().arg.push(cond);

        self
    }

    pub fn select(&mut self, cols: Vec<String>) -> &mut Self {
        if self.selected.is_none() {
            self.selected = Some(Clause {
                ty: ClauseType::Select,
                arg: vec![cols.join(", ")],
                delimiter: "".to_string(),
            });
            return self;
        }

        self.selected.unwrap().arg.push(cols.join(""));
        self
    }
    pub fn distinct(&mut self) -> &mut Self {
        if self.selected.is_none() {
            self.selected = Some(Clause {
                ty: ClauseType::Select,
                arg: vec!["DISTINCT".to_string()],
                delimiter: "".to_string(),
            });
            return self;
        }
        self.selected.unwrap().arg.insert(0, "DISTINCT".to_string());
        self
    }
    pub fn table(&mut self, t: String) -> &mut Self {
        self.table = t;
        self
    }
}
