use serde_json::Value;

#[derive(Debug)]
pub struct Where {
    column: String,
    condition: String,
    value: Value,
}

#[derive(Debug)]
pub struct SqlBuilder<'a> {
    table: String,
    fields: Vec<String>,
    wheres: Vec<Where>,
    sql: &'a str,
}

impl SqlBuilder<'_> {
    /// 初始化
    pub fn new(table: String) -> Self {
        SqlBuilder {
            table,
            fields: vec![],
            wheres: vec![],
            sql: "SELECT * FROM ",
        }
    }

    /// 修改表名称
    pub fn table(&mut self, table: String) -> &mut Self {
        // self.sql += &*table;
        self.table = table;
        self
    }

    /// 获取sql语句
    pub fn build_sql(&mut self) -> Result<String, String> {
        if self.table.is_empty() {
            return Err("table:不能为空".to_string());
        }

        // 字段处理
        self.handle_fields();

        Ok(self.sql.to_string())
    }

    /// 添加字段
    pub fn field(&mut self, fields: &str) -> &mut Self {
        let res = fields
            .split(",");
        println!("{:?}", res);

        self
    }

    /// 追击field
    pub fn field_arr(&mut self, fields: Vec<&str>) -> &mut Self {
        for x in fields {
            self.fields.push(x.to_string())
        }
        self
    }

    /// 字符处理
    fn handle_fields(&mut self) {
        // self.sql += &*self.fields.join(",");
    }
}