use mysql::{Pool, QueryResult, Error};

pub struct SqlHelper {
    sql_helper: Pool,
}

impl SqlHelper {
    pub fn connect() -> SqlHelper {
        let sql_helper = mysql::Pool::new("mysql://root:12345678@localhost:3306/info").unwrap();
        SqlHelper {
            sql_helper
        }
    }

    pub fn execute_query(&self, cmd: String) -> Result<QueryResult, Error> {
        return self.sql_helper.prep_exec(cmd, ())
    }
}