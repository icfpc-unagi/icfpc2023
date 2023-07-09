use actix_web::{HttpResponse, HttpResponseBuilder, Responder};
use anyhow::Result;
use handlebars::Handlebars;
use once_cell::sync::Lazy;
use serde_json::json;

static ENGINE: Lazy<Handlebars> = Lazy::new(|| new_engine());

pub fn new_engine() -> Handlebars<'static> {
    let mut handlebars = Handlebars::new();
    handlebars
        .register_template_string(
            "main",
            r#"
<html lang="ja">
<header>
<meta charset="utf-8">
<meta name="viewport" content="width=device-width,initial-scale=1.0,user-scalable=yes">
<link rel="stylesheet" type="text/css" href="/static/style.css">
<script src="https://ajax.googleapis.com/ajax/libs/jquery/3.3.1/jquery.min.js"></script>
<script src="/static/jquery-linedtextarea.js"></script>
<link href="/static/jquery-linedtextarea.css" rel="stylesheet"/>
</header>
<body>
<nav>
<a href="/"></a>
<ul>
<li><a href="/my_userboard">問題一覧</a></li>
<li><a href="/my_submissions">提出一覧</a></li>
<li><a href="/visualizer">可視化</a></li>
</ul>
</nav>
<main>
<article>
{{{contents}}}
</article>
</main>
</body>
</html>"#,
        )
        .unwrap();
    handlebars.register_template_string("visualizer", r#"
<p>
    <label>
    Problem:
    <input type="number" id="seed" value="{{{problem_id}}}" min="0" max="18446744073709551615" onchange="generate()" />
    </label>
</p>
<p>
    <label>
    Input:<br>
    <textarea id="input" rows="4" style="width:650px;" data-gramm_editor="false" oninput="updateOutput()">{{{input}}}</textarea>
    </label>
</p>
<p>
    <label>
    Output:<br>
    <textarea id="output" rows="4" style="width:650px;" data-gramm_editor="false"
        oninput="updateOutput()">{{{output}}}</textarea>
    </label>
</p>
<p>
    <input type="button" id="save_png" value="Save as PNG">&ensp;
</p>
<p>
    <label>
    color_type:
    <select id="color_type" onchange="visualize()">
        <option value="0">楽器ID</option>
        <option value="1">スコア</option>
    </select>
    </label>&ensp;
</p>
<p style="display:flex;">
    <input type="button" id="play" value="▶" style="width:32px;height:32px;bottom:5px;position:relative;">&ensp;
    <label>
    slow
    <input type="range" id="speed" min="1" max="60" value="30" style="width:200px;">
    fast&emsp;
    </label>
    <label>
    turn:
    <input type="number" id="turn" value="0" min="0" max="0" style="width:70px;text-align:right;"
        onchange="update_t(this.value)" />
    </label>&ensp;
</p>
<p>
    <input type="range" id="t_bar" min="0" max="0" value="0" style="width:780px;" onchange="update_t(this.value)"
    oninput="update_t(this.value)">
</p>

<hr>
<p id="score" style="user-select:none"></p>
<div id="result">
</div>
<script type="module">
    import init, { vis, get_max_turn } from './vis.js';

    var seed0 = "";

    async function run() {
    await init();
    // seed0 = gen("0");
    generate();
    }
    run();

    function generate() {
    const seed = document.getElementById("seed").value;
    // const input = gen(seed);
    // document.getElementById("input").value = input;
    updateOutput();
    }
    window.generate = generate;

    function visualize() {
    const input = document.getElementById("input").value;
    const output = document.getElementById("output").value;
    const t = document.getElementById("turn").value;
    const select = document.getElementById("color_type");
    const color_type = parseInt(select.options[select.selectedIndex].value);
    try {
        const ret = vis(input, output, t, color_type);
        document.getElementById("score").innerHTML = "Score = " + ret.score;
        if (ret.error != "") {
        document.getElementById("score").innerHTML += " <font color='red'>(" + ret.error + ")</font>";
        }
        document.getElementById("result").innerHTML = ret.svg;
    } catch (error) {
        console.log(error);
        document.getElementById("result").innerHTML = "<p>Invalid</p>";
    }
    }
    window.visualize = visualize;

    function update_t(t) {
    const max_turn = Number(document.getElementById("turn").max);
    const new_turn = Math.min(Math.max(0, t), max_turn);
    document.getElementById("turn").value = new_turn;
    document.getElementById("t_bar").value = new_turn;
    visualize();
    }
    window.update_t = update_t;

    var prev = Date.now();
    const play = document.getElementById("play");
    const speed = document.getElementById("speed");

    function start_autoplay() {
    if (Number(document.getElementById("turn").value) >= Number(document.getElementById("turn").max)) {
        document.getElementById("turn").value = 0;
    }
    prev = Date.now();
    play.value = "■";
    update_t(document.getElementById("turn").value);
    }
    window.start_autoplay = start_autoplay;

    function updateOutput() {
    play.value = "▶";
    const input = document.getElementById("input").value;
    const output = document.getElementById("output").value;
    try {
        const t = get_max_turn(input, output);
        document.getElementById("turn").max = t;
        document.getElementById("t_bar").max = t;
        update_t(t);
    } catch (error) {
        document.getElementById("result").innerHTML = "<p>Invalid</p>";
    }
    }
    window.updateOutput = updateOutput;

    play.onclick = event => {
    if (play.value == "■") {
        play.value = "▶";
    } else {
        start_autoplay();
    }
    }

    function autoplay() {
    if (play.value == "■") {
        const now = Date.now();
        let s = 500;
        if ((now - prev) * speed.value >= s) {
        const inc = Math.floor((now - prev) * speed.value / s);
        prev += Math.floor(inc * s / speed.value);
        update_t(Number(document.getElementById("turn").value) + inc);
        if (Number(document.getElementById("turn").value) >= Number(document.getElementById("turn").max)) {
            play.value = "▶";
        }
        }
    }
    requestAnimationFrame(autoplay);
    }
    autoplay();

    document.getElementById("save_png").onclick = event => {
    const input = document.getElementById("input").value;
    const output = document.getElementById("output").value;
    const t = document.getElementById("turn").value;
    const select = document.getElementById("color_type");
    const color_type = parseInt(select.options[select.selectedIndex].value);
    const svgData = vis(input, output, t, color_type).svg;
    const svg = new DOMParser().parseFromString(svgData, "image/svg+xml").getElementById("vis");
    const canvas = document.createElement("canvas");
    canvas.width = svg.width.baseVal.value;
    canvas.height = svg.height.baseVal.value;
    const ctx = canvas.getContext("2d");
    const image = new Image;
    image.onload = function () {
        ctx.drawImage(image, 0, 0);
        const a = document.createElement("a");
        a.href = canvas.toDataURL("image/png");
        a.download = "vis.png";
        a.click();
    }
    image.src = "data:image/svg+xml;charset=utf-8;base64," + btoa(unescape(encodeURIComponent(svgData)));
    }
</script>
"#).unwrap();
    handlebars
}

fn escape_html(s: &str) -> String {
    s.chars()
        .map(|c| match c {
            '&' => "&amp;".to_string(),
            '<' => "&lt;".to_string(),
            '>' => "&gt;".to_string(),
            '"' => "&quot;".to_string(),
            '\'' => "&#x27;".to_string(),
            '/' => "&#x2F;".to_string(),
            _ => c.to_string(),
        })
        .collect()
}

pub fn render(contents: &str) -> String {
    ENGINE
        .render(
            "main",
            &json!({
                "contents": contents,
            }),
        )
        .unwrap()
}

pub fn to_error_response(result: &anyhow::Error) -> HttpResponse {
    HttpResponse::InternalServerError()
        .content_type("text/html")
        .body(render(&format!(
            "<h1>エラー</h1><pre><code>{}</code></pre>",
            escape_html(&format!("{:?}", result))
        )))
}

pub fn to_html_response(result: &str) -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html")
        .body(render(result))
}

pub fn to_png_response(result: &Vec<u8>) -> HttpResponse {
    HttpResponse::Ok()
        .content_type("image/png")
        .append_header(("Cache-Control", "public, max-age=600"))
        .body(result.clone())
}

pub fn to_response(result: Result<String>) -> impl Responder {
    match result {
        Ok(x) => to_html_response(&x),
        Err(e) => to_error_response(&e),
    }
}

pub fn render_visualize(problem_id: u32, input: &str, output: &str) -> String {
    ENGINE
        .render(
            "visualize",
            &json!({
                "problem_id": problem_id,
                "input": escape_html(input),
                "output": escape_html(output),
            }),
        )
        .unwrap()
}
