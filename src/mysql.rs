use anyhow::anyhow;
use anyhow::Result;
use mysql::prelude::*;
use mysql::*;
use once_cell::sync::Lazy;
use std::env;

static CLIENT: Lazy<mysql::Pool> = Lazy::new(|| {
    let password = env::var("UNAGI_PASSWORD").unwrap_or_else(|_| "".into());
    let url = format!("mysql://root:{}@34.146.125.93:3306/unagi", password);
    let pool = Pool::new(url).unwrap();
    eprintln!("MySQL connection established.");
    pool
});

pub fn select<T>(query: &str, f: impl FnMut(Row) -> T) -> Result<Vec<T>> {
    let mut conn = CLIENT.get_conn()?;
    let selected_data: Vec<T> = conn.query_map(query, f)?;
    Ok(selected_data)
}

pub fn row<T>(query: &str) -> Result<T> where T: FromRow {
    let mut conn = CLIENT.get_conn()?;
    let row: T = conn.query_first(query)?.ok_or(anyhow!("No data"))?;
    Ok(row)
}

pub fn cell<T>(query: &str) -> Result<T> where T: FromValue {
    match row::<(T, )>(query) {
        Ok(row) => Ok(row.0),
        Err(e) => Err(e),
    }
}

pub fn exec(query: &str) -> Result<()> {
    let mut conn = CLIENT.get_conn()?;
    conn.query_drop(query)?;
    Ok(())
}

pub fn insert<T>(query: &str, values: &[T]) -> Result<u64>
where
    for<'a> &'a T: Into<Params>
{
    let mut conn = CLIENT.get_conn()?;
    let params: Vec<Params> = values.iter().map(|v| v.into()).collect();
    conn.exec_batch(query, params)?;
    Ok(conn.affected_rows())
}
