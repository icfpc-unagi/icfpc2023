#![allow(non_snake_case)]

use crate::*;

use svg::node::{
    element::{Circle, Group, Rectangle, Title},
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

pub fn vis(input: &Input, out: &Output) -> (i64, String, String) {
    let mul = (800.0 / input.room.0).min(800.0 / input.room.1);
    let W = (input.room.0 * mul).ceil() as usize;
    let H = (input.room.1 * mul).ceil() as usize;
    let score = 0; //compute_score(input, out);
    let mut doc = svg::Document::new()
        .set("id", "vis")
        .set("viewBox", (-5, -5, W + 10, H + 10))
        .set("width", W + 10)
        .set("height", H + 10);
    doc = doc.add(
        rect(0.0, 0.0, input.room.0 * mul, input.room.1 * mul, "white")
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
                    "attendees {}\n({:.0}, {:.0})",
                    i, input.pos[i].0, input.pos[i].1
                ))))
                .add(
                    Circle::new()
                        .set("cx", input.pos[i].0 * mul)
                        .set("cy", input.pos[i].1 * mul)
                        .set("r", 1)
                        .set("fill", "black"),
                ),
        );
    }
    for i in 0..input.musicians.len() {
        doc = doc.add(
            Group::new()
                .add(Title::new().add(Text::new(format!(
                    "musicians {}, inst = {}\n({}, {})",
                    i, input.musicians[i], out[i].0, out[i].1
                ))))
                .add(
                    Circle::new()
                        .set("cx", out[i].0 * mul)
                        .set("cy", out[i].1 * mul)
                        .set("r", 5.0 * mul)
                        .set("fill", color(input.musicians[i] as f64 / t as f64)),
                ),
        )
    }
    (score, String::new(), doc.to_string())
}
