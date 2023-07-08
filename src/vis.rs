#![allow(non_snake_case)]

use crate::*;

use svg::node::{
    element::{Circle, Group, Line, Rectangle, Title},
    Text,
};

/// 0 <= val <= 1
pub fn color(mut val: f64) -> String {
    val.setmin(1.0);
    val.setmax(0.0);
    let (r, g, b) = if val < 0.5 {
        let x = val * 2.0;
        (
            30. * (1.0 - x) + 144. * x,
            144. * (1.0 - x) + 255. * x,
            255. * (1.0 - x) + 30. * x,
        )
    } else {
        let x = val * 2.0 - 1.0;
        (
            144. * (1.0 - x) + 255. * x,
            255. * (1.0 - x) + 30. * x,
            30. * (1.0 - x) + 70. * x,
        )
    };
    format!(
        "#{:02x}{:02x}{:02x}",
        r.round() as i32,
        g.round() as i32,
        b.round() as i32
    )
}

pub fn rect(x: f64, y: f64, w: f64, h: f64, fill: &str) -> Rectangle {
    Rectangle::new()
        .set("x", x)
        .set("y", y)
        .set("width", w)
        .set("height", h)
        .set("fill", fill)
}

pub fn vis(input: &Input, out: &Output, color_type: i32, focus: usize) -> (i64, String, String) {
    let room = (
        input.pos.iter().map(|a| a.0.ceil() as usize).max().unwrap() as f64 + 10.0,
        input.pos.iter().map(|a| a.1.ceil() as usize).max().unwrap() as f64 + 10.0,
    );
    let mul = (1000.0 / room.0).min(700.0 / room.1);
    let W = (room.0 * mul).ceil() as usize;
    let H = (room.1 * mul).ceil() as usize;
    let score = compute_score(input, out);
    let score_musicians = compute_score_for_musician(input, out);
    let score_attendees = compute_score_for_attendees(input, out);
    let score_musician_max = score_musicians.iter().map(|a| a.abs()).max().unwrap();
    let score_attendees_max = score_attendees.iter().map(|a| a.abs()).max().unwrap();
    let mut doc = svg::Document::new()
        .set("id", "vis")
        .set("viewBox", (-5, -5, W + 10, H + 10))
        .set("width", W + 10)
        .set("height", H + 10);
    doc = doc.add(
        rect(0.0, 0.0, room.0 * mul, room.1 * mul, "white")
            .set("stroke-width", 1)
            .set("stroke", "black"),
    );
    doc = doc.add(
        rect(
            input.stage0.0 * mul,
            input.stage0.1 * mul,
            (input.stage1.0 - input.stage0.0) * mul,
            (input.stage1.1 - input.stage0.1) * mul,
            "wheat",
        )
        .set("stroke-width", 1)
        .set("stroke", "black"),
    );
    let t = input.tastes[0].len();
    for i in 0..input.pos.len() {
        doc = doc.add(
            Group::new()
                .add(Title::new().add(Text::new(format!(
                    "attendees {}\n({:.0}, {:.0})\nscore = {}",
                    i, input.pos[i].0, input.pos[i].1, score_attendees[i]
                ))))
                .add(
                    Circle::new()
                        .set("cx", input.pos[i].0 * mul)
                        .set("cy", input.pos[i].1 * mul)
                        .set("r", 2)
                        .set(
                            "fill",
                            match color_type {
                                0 => "black".to_owned(),
                                1 => color(
                                    0.5 + 0.5 * score_attendees[i] as f64
                                        / score_attendees_max as f64,
                                ),
                                _ => unimplemented!(),
                            },
                        ),
                ),
        );
    }
    for i in 0..out.len() {
        doc = doc.add(
            Group::new()
                .add(Title::new().add(Text::new(format!(
                    "musicians {}, inst = {}\n({}, {})\nscore = {}",
                    i, input.musicians[i], out[i].0, out[i].1, score_musicians[i]
                ))))
                .add(
                    Circle::new()
                        .set("cx", out[i].0 * mul)
                        .set("cy", out[i].1 * mul)
                        .set("r", 5.0 * mul)
                        .set(
                            "fill",
                            match color_type {
                                0 => color(input.musicians[i] as f64 / t as f64),
                                1 => color(
                                    0.5 + 0.5 * score_musicians[i] as f64
                                        / score_musician_max as f64,
                                ),
                                _ => unimplemented!(),
                            },
                        )
                        .set("onclick", format!("visualize({})", i)),
                ),
        )
    }
    if focus != !0 {
        let mut max = 0;
        for i in 0..input.n_attendees() {
            max.setmax(compute_score_for_pair(input, out, focus, i));
        }
        for i in 0..input.n_attendees() {
            let score = compute_score_for_pair(input, out, focus, i);
            if score != 0 {
                doc = doc.add(
                    Line::new()
                        .set("x1", out[focus].0 * mul)
                        .set("y1", out[focus].1 * mul)
                        .set("x2", input.pos[i].0 * mul)
                        .set("y2", input.pos[i].1 * mul)
                        .set("stroke", color(0.5 + 0.5 * score as f64 / max as f64))
                        .set("stroke-width", 2),
                )
            }
        }
    }
    (score, String::new(), doc.to_string())
}
