use crate::*;

use actix_web::Responder;
use anyhow::Result;

pub async fn internal_handler() -> Result<String> {
    let mut buf = String::new();
    let mut total_score = 0;
    for row in sql::select(
        "
SELECT
    *
FROM
    (
    SELECT
        problem_id,
        submission_score,
        MIN(submission_id) AS submission_id,
        MIN(official_id) AS official_id
    FROM
        submissions
    NATURAL JOIN(
        SELECT
            problem_id,
            MAX(submission_score) AS submission_score
        FROM
            submissions
        GROUP BY
            problem_id
    ) AS t
GROUP BY
    problem_id
ORDER BY
    problem_id
) AS t1
NATURAL RIGHT JOIN(
    SELECT DISTINCT
        problem_id
    FROM
        problem_chunks
) AS t2",
        mysql::Params::Empty,
    )? {
        let problem_id: i64 = row.get("problem_id")?;
        let submission_id = row.get_option::<i64>("submission_id")?;
        let official_id = row.get_option::<String>("official_id")?;
        let submission_score = row.get_option::<i64>("submission_score")?;
        let submission_score = submission_score.unwrap_or(0);
        total_score += submission_score;
        let score = match official_id.or_else(|| submission_id.map(|x| x.to_string())) {
            Some(id) => format!(
                "<a href=\"/submission?submission_id={}\">{}</a>",
                id, submission_score
            ),
            None => format!("{}", submission_score),
        };
        buf.push_str(&format!(
            r#"
<div style="display: inline-block; min-width: 200px; width: 20%; margin: 1em; text-align: right">
問題番号: {}<br>
スコア: {}<br>
<img src="/problem_png?problem_id={}" style="width: 200px; height: 200px; object-fit: contain;">
</div>
"#,
            problem_id, score, problem_id,
        ));
    }
    Ok(format!(
        "
        <h1>Unagi Userboard</h1>
        <p>合計点: {}</p>
        <center>{}</center>",
        total_score, buf
    ))
}

pub async fn handler() -> impl Responder {
    www::handlers::template::to_response(internal_handler().await)
}
