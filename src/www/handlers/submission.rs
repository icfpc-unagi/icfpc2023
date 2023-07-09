use crate::{input_stats::get_stats, *};

use actix_web::{web, HttpResponse, Responder};
use anyhow::Result;
use mysql::params;
use serde::Deserialize;
use svg;

#[derive(Debug, Deserialize)]
pub struct Query {
    pub submission_id: String,
    #[serde(default = "default_color_type")]
    pub color_type: i32,
}

fn default_color_type() -> i32 {
    1
}

// use actix_web::web;
// use actix_web::HttpResponse;
// use actix_web::Responder;
use std::fmt::Write;

pub async fn handler(info: web::Query<Query>) -> impl Responder {
    match handle(info).await {
        Ok(contents) => HttpResponse::Ok()
            .content_type("text/html")
            .body(www::handlers::template::render(&contents)),
        Err(e) => HttpResponse::InternalServerError()
            .content_type("text/html")
            .body(www::handlers::template::render(&format!("{}", e))),
    }
}

async fn handle(info: web::Query<Query>) -> Result<String> {
    let mut buf = String::new();
    let submission = api::get_submission(&info.submission_id).await?;
    // Fetch submission tags
    let rows = sql::select(
        "
SELECT
    submission_tag
FROM
    submission_tags
NATURAL LEFT JOIN
    submissions
WHERE
    CAST(submission_id AS CHAR) = :submission_id OR official_id = :submission_id",
        params! {
            "submission_id" => &info.submission_id,
        },
    )?;

    let mut tags = Vec::new();
    for row in rows {
        tags.push(row.get::<String>("submission_tag")?);
    }

    // TODO: Cache problem data
    let problem_id = submission.submission.problem_id;
    let input: Input = api::get_problem(problem_id).await?.into();
    let output = parse_output(&submission.contents)?;
    let computed_scores = compute_score_fast(&input, &output);

    write!(
        &mut buf,
        "<a href=\"/visualizer?submission_id={}\">[Show on Visualizer]</a>",
        submission.submission._id,
    )?;
    write!(
        &mut buf,
        "<h1>Submission ID: {}</h1>",
        submission.submission._id
    )?;
    for tag in tags {
        write!(
            &mut buf,
            "<a href=\"/my_submissions?tag={}\" class=\"tag\">{}</a>",
            percent_encoding::utf8_percent_encode(&tag, percent_encoding::NON_ALPHANUMERIC),
            tag,
        )?;
    }
    write!(
        &mut buf,
        "<ul><li>Problem ID: {}</li><ul>",
        submission.submission.problem_id
    )?;
    let (musicians_info, attendees_info, pillars_info) = get_stats(&input);
    write!(
        &mut buf,
        "<li>n_musicians: {}</li>",
        musicians_info.n_musicians
    )?;
    write!(
        &mut buf,
        "<li>area_per_musician: {}</li>",
        musicians_info.area_per_musician
    )?;
    write!(
        &mut buf,
        "<li>border_len_per_musician: {}</li>",
        musicians_info.border_len_per_musician
    )?;
    write!(
        &mut buf,
        "<li>n_instruments: {}</li>",
        musicians_info.n_instruments
    )?;
    write!(
        &mut buf,
        "<li>stats_musicians_per_instrument: {:?}</li>",
        musicians_info.stats_musicians_per_instrument
    )?;
    write!(
        &mut buf,
        "<li>n_attendees: {}</li>",
        attendees_info.n_attendees
    )?;
    write!(
        &mut buf,
        "<li>stats_tastes: {:?}</li>",
        attendees_info.stats_tastes
    )?;
    write!(&mut buf, "<li>n_pillars: {}</li>", pillars_info.n_pillars)?;
    write!(
        &mut buf,
        "<li>stats_radius: {:?}</li>",
        pillars_info.stats_radius
    )?;

    write!(
        &mut buf,
        "</uL><li>Submitted at: {}</li>",
        submission.submission.submitted_at
    )?;
    write!(
        &mut buf,
        "<li>Score: {} (computed score: {})</li>",
        submission.submission.score, computed_scores.0,
    )?;
    let svg = vis::vis(&input, &output, info.color_type, !0, None);
    write!(&mut buf, "{}", svg.2)?;

    // Construct a SVG for charting musician scores fit in a rect.
    let musicians_svg = generate_svg_chart(computed_scores.1.clone(), "blue", "🎤");
    write!(&mut buf, "{}", musicians_svg)?;

    // Construct a SVG for charting attendee scores fit in a rect.
    let attendees_svg = generate_svg_chart(computed_scores.2.clone(), "red", "👂");
    write!(&mut buf, "{}", attendees_svg)?;

    write!(
        &mut buf,
        "<pre style=\"white-space: pre-wrap;\"><code>{}</code></pre>",
        submission.contents
    )?;
    Ok(buf)
}
fn generate_svg_chart(mut scores: Vec<i64>, color: &str, symbol: &str) -> svg::Document {
    scores.sort_unstable_by(|a, b| b.partial_cmp(&a).unwrap());
    let max_score = scores.first().copied().unwrap_or(1).max(1) as f64;
    let min_score = scores.last().copied().unwrap_or(0).min(0) as f64;
    let range = (max_score - min_score) as f64;
    let lift = -min_score / range;
    let mut svg = svg::Document::new()
        .set("viewBox", (0, 0, 1, 1))
        .set("width", 200)
        .set("height", 200)
        .set("transform", "scale(1, -1)")
        .set("style", "margin: 10pt;")
        .set("fill", color);
    svg = svg.add(
        svg::node::element::Group::new()
            .set("transform", "scale(1, -1)")
            .add(
                svg::node::element::Text::new()
                    .set("x", 0.95)
                    .set("y", -0.95)
                    .set("text-anchor", "end")
                    .set("dominant-baseline", "text-before-edge")
                    .set("font-size", 0.2)
                    .add(svg::node::Text::new(symbol)),
            ),
    );
    svg = svg.add(
        svg::node::element::Rectangle::new()
            .set("x", 0)
            .set("y", 0)
            .set("width", 1)
            .set("height", 1)
            .set("fill", "none")
            .set("stroke", "black")
            .set("stroke-width", 0.01),
    );
    for i in 0..scores.len() {
        let normalized = scores[i] as f64 / range;
        let (y, height) = if normalized.is_sign_positive() {
            (lift, normalized)
        } else {
            (lift + normalized, -normalized)
        };
        svg = svg.add(
            svg::node::element::Rectangle::new()
                .set("x", i as f64 / scores.len() as f64)
                .set("y", y)
                .set("width", 1.0 / scores.len() as f64)
                .set("height", height),
        );
    }
    svg
}
