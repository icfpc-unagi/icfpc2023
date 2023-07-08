use anyhow::Result;
use mysql::prelude::*;
use mysql::*;
use mysql;
use once_cell::sync::Lazy;
use std::env;

static CLIENT: Lazy<mysql::Pool> = Lazy::new(|| {
    let password = env::var("UNAGI_PASSWORD").unwrap_or_else(|_| "".into());
    let url = format!("mysql://root:{}@34.146.125.93:3306/unagi", password);
    let pool = Pool::new(url).unwrap();
    eprintln!("MySQL connection established.");
    pool
});

pub fn select<T, P>(query: &str, params: P, mut f: impl FnMut(Row) -> T) -> Result<Vec<T>>
where
    P: Into<Params>,
{
    let mut conn = CLIENT.get_conn()?;
    let selected_data: Vec<T> = conn.exec_map(query, params, move |r| f(Row{row: r}))?;
    Ok(selected_data)
}

pub fn row<T, P>(query: &str, params: P) -> Result<Option<T>>
where
    T: FromRow,
    P: Into<Params>,
{
    let mut conn = CLIENT.get_conn()?;
    let row: Option<T> = conn.exec_first(query, params)?;
    Ok(row)
}

pub fn cell<T, P>(query: &str, params: P) -> Result<Option<T>>
where
    T: FromValue,
    P: Into<Params>,
{
    match row::<(T,), P>(query, params) {
        Ok(Some(row)) => Ok(Some(row.0)),
        Ok(None) => Ok(None),
        Err(e) => Err(e),
    }
}

pub fn exec<P>(query: &str, params: P) -> Result<()>
where
    P: Into<Params>,
{
    let mut conn = CLIENT.get_conn()?;
    conn.exec_drop(query, params)?;
    Ok(())
}

pub fn insert<T>(query: &str, values: &[T]) -> Result<u64>
where
    for<'a> &'a T: Into<Params>,
{
    let mut conn = CLIENT.get_conn()?;
    let params: Vec<Params> = values.iter().map(|v| v.into()).collect();
    conn.exec_batch(query, params)?;
    Ok(conn.affected_rows())
}

pub struct Row {
    row: mysql::Row
}

impl Row {
    pub fn get<T>(&self, name: &str) -> Result<T>
    where
        T: FromValue,
    {
        let idx = name.idx(&*self.row.columns())
            .ok_or_else(|| anyhow::anyhow!("Column {} is not found", name))?;
        match self.row.get_opt::<mysql::Value, usize>(idx) {
            Some(Ok(value)) => match value {
                mysql::Value::NULL => return Err(anyhow::anyhow!("{} is null", name)),
                x => mysql::from_value_opt::<T>(x.clone())
                    .map_err(|e| anyhow::anyhow!("{}: {}", name, e)),
            },
            Some(Err(e)) => return Err(anyhow::anyhow!("{}: {}", name, e)),
            None => return Err(anyhow::anyhow!("{} is null", name)),
        }
    }

    pub fn get_option<T>(&self, name: &str) -> Result<Option<T>>
    where
        T: FromValue,
    {
        let idx = name.idx(&*self.row.columns())
            .ok_or_else(|| anyhow::anyhow!("Column {} is not found", name))?;
        match self.row.get_opt::<mysql::Value, usize>(idx) {
            Some(Ok(value)) => match value {
                mysql::Value::NULL => Ok(None),
                x => mysql::from_value_opt::<T>(x.clone())
                    .map_err(|e| anyhow::anyhow!("{}: {}", name, e))
                    .map(Some),
            },
            Some(Err(e)) => return Err(anyhow::anyhow!("{}: {}", name, e)),
            None => Ok(None),
        }
    }
}
