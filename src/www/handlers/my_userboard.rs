use crate::*;

use actix_web::{web, HttpResponse, Responder};

pub async fn handler() -> impl Responder {
    let mut buf = String::new();
    buf.push_str("<table><tr><td>問題番号</td><td>スコア</td></tr>");
    let mut total_score = 0;
    sql::select("
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
    problem_id", mysql::Params::Empty, |row| {
        let submission_id: i64 = row.get("submission_id").unwrap();
        let official_id: Option<String> = row.get("official_id").unwrap();
        let problem_id: i64 = row.get("problem_id").unwrap();
        let submission_score: Option<i64> = row.get("submission_score").unwrap();
        let submission_score = submission_score.unwrap_or(0);
        total_score += submission_score;
        buf.push_str(&format!("<tr><td>{}</td>", problem_id));
        match official_id {
            Some(official_id) => {
                buf.push_str(&format!(
                    "<td><a href=\"/submission?submission_id={}\">{}</a></td></tr>",
                    official_id, submission_score));
            }
            None => {
                buf.push_str(&format!("<td>{}</td></tr>", submission_score));
            }
        }
    }).unwrap();
    buf.push_str("</table>");
    buf = format!("
        <h1>Unagi Userboard</h1>
        <p>合計点: {}</p>
        {}",
        total_score,
        buf);
    HttpResponse::Ok()
        .content_type("text/html")
        .body(www::handlers::template::render(&buf))
}
