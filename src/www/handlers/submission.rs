use crate::{input_stats::get_stats, *};

use actix_web::{web, HttpResponse, Responder};
use anyhow::Result;
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
    // TODO: Cache problem data
    let problem_id = submission.submission.problem_id;
    let input: Input = api::get_problem(problem_id).await?.into();
    let output = parse_output(&submission.contents)?;
    let computed_scores = compute_score_fast(&input, &output);

    write!(
        &mut buf,
        "<h1>Submission ID: {}</h1>",
        submission.submission._id
    )?;
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
    let svg = vis::vis(&input, &output, info.color_type, !0);
    write!(&mut buf, "{}", svg.2)?;

    // Construct a SVG for charting musician scores fit in a rect.
    let mut musician_scores = computed_scores.1.clone();
    musician_scores.sort_unstable_by(|a, b| b.cmp(&a));
    let mut musicians_svg = svg::Document::new()
        .set("viewBox", (0, 0, 1, 1))
        .set("width", 200)
        .set("height", 200)
        .set("transform", "scale(1, -1)")
        .set("style", "margin: 10pt;");
    musicians_svg = musicians_svg.add(
        svg::node::element::Group::new()
            .set("transform", "scale(1, -1)")
            .add(
                svg::node::element::Text::new()
                    .set("x", 0.95)
                    .set("y", -0.95)
                    .set("text-anchor", "end")
                    .set("dominant-baseline", "text-before-edge")
                    .set("font-size", 0.2)
                    .add(svg::node::Text::new("🎤")),
            ),
    );
    musicians_svg = musicians_svg.add(
        svg::node::element::Rectangle::new()
            .set("x", 0)
            .set("y", 0)
            .set("width", 1)
            .set("height", 1)
            .set("fill", "none")
            .set("stroke", "black")
            .set("stroke-width", 0.01),
    );
    for i in 0..musician_scores.len() {
        let normalized = musician_scores[i] as f64 / musician_scores[0] as f64;
        musicians_svg = musicians_svg.add(
            svg::node::element::Rectangle::new()
                .set("x", i as f64 / musician_scores.len() as f64)
                .set("y", 0)
                .set("width", 1.0 / musician_scores.len() as f64)
                .set("height", normalized)
                .set("fill", "blue"),
        );
    }
    write!(&mut buf, "{}", musicians_svg)?;

    // Construct a SVG for charting attendee scores fit in a rect.
    let mut attendee_scores = computed_scores.2.clone();
    attendee_scores.sort_unstable_by(|a, b| b.cmp(&a));
    let mut attendees_svg = svg::Document::new()
        .set("viewBox", (0, 0, 1, 1))
        .set("width", 200)
        .set("height", 200)
        .set("transform", "scale(1, -1)")
        .set("style", "margin: 10pt;");
    attendees_svg = attendees_svg.add(
        svg::node::element::Group::new()
            .set("transform", "scale(1, -1)")
            .add(
                svg::node::element::Text::new()
                    .set("x", 0.95)
                    .set("y", -0.95)
                    .set("text-anchor", "end")
                    .set("dominant-baseline", "text-before-edge")
                    .set("font-size", 0.2)
                    .add(svg::node::Text::new("👂")),
            ),
    );
    attendees_svg = attendees_svg.add(
        svg::node::element::Rectangle::new()
            .set("x", 0)
            .set("y", 0)
            .set("width", 1)
            .set("height", 1)
            .set("fill", "none")
            .set("stroke", "black")
            .set("stroke-width", 0.01),
    );
    for i in 0..attendee_scores.len() {
        let normalized = attendee_scores[i] as f64 / attendee_scores[0] as f64;
        attendees_svg = attendees_svg.add(
            svg::node::element::Rectangle::new()
                .set("x", i as f64 / attendee_scores.len() as f64)
                .set("y", 0)
                .set("width", 1.0 / attendee_scores.len() as f64)
                .set("height", normalized)
                .set("fill", "red"),
        );
    }
    write!(&mut buf, "{}", attendees_svg)?;

    write!(
        &mut buf,
        "<pre style=\"white-space: pre-wrap;\"><code>{}</code></pre>",
        submission.contents
    )?;
    Ok(buf)
}
