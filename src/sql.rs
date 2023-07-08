use anyhow::Result;
use mysql;
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

pub fn select<P>(query: &str, params: P) -> Result<Vec<Row>>
where
    P: Into<Params>,
{
    let mut conn = CLIENT.get_conn()?;
    conn.exec_map(query, params, |r| Row { row: r })
        .map_err(|e| anyhow::anyhow!("{}", e))
}

pub fn row<P>(query: &str, params: P) -> Result<Option<Row>>
where
    P: Into<Params>,
{
    let mut conn = CLIENT.get_conn()?;
    let row = conn
        .exec_first(query, params)?
        .and_then(|r| Some(Row { row: r }));
    Ok(row)
}

pub fn cell<T, P>(query: &str, params: P) -> Result<Option<T>>
where
    T: FromValue,
    P: Into<Params>,
{
    match row::<P>(query, params) {
        Ok(Some(row)) => Ok(row.at(0)?),
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
    row: mysql::Row,
}

impl Row {
    pub fn at_option<T>(&self, idx: usize) -> Result<Option<T>>
    where
        T: FromValue,
    {
        match self.row.get_opt::<mysql::Value, usize>(idx) {
            Some(Ok(value)) => match value {
                mysql::Value::NULL => Ok(None),
                x => mysql::from_value_opt::<T>(x.clone())
                    .map_err(|e| anyhow::anyhow!("Column {}: {}", idx, e))
                    .map(Some),
            },
            Some(Err(e)) => return Err(anyhow::anyhow!("Column {}: {}", idx, e)),
            None => Ok(None),
        }
    }

    pub fn at<T>(&self, idx: usize) -> Result<T>
    where
        T: FromValue,
    {
        self.at_option(idx)?
            .ok_or_else(|| anyhow::anyhow!("Column {} is null", idx))
    }

    fn idx(&self, name: &str) -> Result<usize> {
        name.idx(&*self.row.columns())
            .ok_or_else(|| anyhow::anyhow!("Column {} is not found", name))
    }

    pub fn get<T>(&self, name: &str) -> Result<T>
    where
        T: FromValue,
    {
        self.at(self.idx(name)?)
    }

    pub fn get_option<T>(&self, name: &str) -> Result<Option<T>>
    where
        T: FromValue,
    {
        self.at_option(self.idx(name)?)
    }
}
