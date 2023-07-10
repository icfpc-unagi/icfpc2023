use super::*;

pub struct CandidateConfig {
    pub use_pattern1: bool,
    pub use_pattern2: bool,
    pub use_pattern3: bool,
    pub use_pattern4: bool,
    pub use_pattern23: bool,
    pub limit_pattern2: Option<usize>,
    pub limit_pattern3: Option<usize>,
    pub limit_pattern23: Option<usize>,
    pub filter_by_reach23: bool,
    pub filter_by_reach14: bool,
    /// この個数より多くintersectしてたら捨てる
    pub filter_by_intersections1: Option<usize>,
    pub filter_by_intersections234: Option<usize>,
    pub pattern2_disallow_blocked: bool,
}

pub fn is_blocked_by_existing(input: &Input, output: &Output, p: P, q: P) -> bool {
    for r in &output.0 {
        if is_blocked(p, q, *r) {
            return true;
        }
    }
    for r in &input.pillars {
        if is_blocked_by_circle(p, q, *r) {
            return true;
        }
    }
    false
}

// どのattendeeにも到達しなかったら要らない?
pub fn does_reach_some_audiences(input: &Input, output: &Output, musician_pos: P) -> bool {
    for attendee_pos in &input.pos {
        if !is_blocked_by_existing(input, output, musician_pos, *attendee_pos) {
            return true;
        }
    }
    false
}

/// 2つのmusician円に接するやつ
pub fn pattern1(input: &Input, output: &Output) -> Vec<P> {
    let eps = 1e-6;
    let mut cposs = vec![];
    for i in 0..input.n_musicians() {
        for j in 0..i {
            cposs.extend(P::pi_cc(
                (output.0[i], 10.0 + eps),
                (output.0[j], 10.0 + eps),
            ));
        }
    }
    cposs
}

/// とあるattendeeからの直線とmusician円に接するやつ
pub fn pattern2(input: &Input, output: &Output) -> Vec<P> {
    // TODO: 意味ないケース結構ありそうなので消す
    let eps = 1e-6;

    let mut am_pairs = vec![];
    for attendee_id in 0..input.n_attendees() {
        for musician_id in 0..input.n_musicians() {
            if is_blocked_by_someone(input, &output.0, musician_id, attendee_id) {
                continue;
            }
            let d = (input.pos[attendee_id] - output.0[musician_id]).abs2();
            am_pairs.push((d, attendee_id, musician_id))
        }
    }
    am_pairs.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

    let mut cposs = vec![];
    for (_, attendee_id, musician_id) in am_pairs {
        let attendee_pos = input.pos[attendee_id];
        let musician_pos = output.0[musician_id];
        for p in P::tan_cp((musician_pos, 5.0 + eps), attendee_pos) {
            for q in P::pi_cl((musician_pos, 10.0 + eps), (attendee_pos, p)) {
                // attendee_pos -> q がブロックされてたらこれいらないでしょっていう
                // TODO: 岩田さんに聞いたほうがいいかも
                if is_blocked_by_existing(input, output, attendee_pos, q) {
                    continue;
                }

                // musicianより更に近くにおけるんだったら何かがおかしい（意味ない）
                if (q - attendee_pos).abs2() < (musician_pos - attendee_pos).abs2() {
                    continue;
                }

                cposs.push(q);
            }
        }
    }
    cposs
}

pub fn get_stage_line(input: &Input, side: i32) -> (P, P) {
    let (s0, s1) = (input.stage0, input.stage1);
    return match side {
        0 => (P(s0.0, s0.1 + 10.0), P(s1.0, s0.1 + 10.0)),
        1 => (P(s0.0, s1.1 - 10.0), P(s1.0, s1.1 - 10.0)),
        2 => (P(s0.0 + 10.0, s0.1), P(s0.0 + 10.0, s1.1)),
        3 => (P(s1.0 - 10.0, s0.1), P(s1.0 - 10.0, s1.1)),
        _ => unreachable!(),
    };
}

/// いま繋がってるmusician-attendeeをギリギリ邪魔しないstageきわ
pub fn pattern3(input: &Input, output: &Output) -> Vec<P> {
    let eps = 1e-6;

    let mut am_pairs = vec![];
    for attendee_id in 0..input.n_attendees() {
        for musician_id in 0..input.n_musicians() {
            if is_blocked_by_someone(input, &output.0, musician_id, attendee_id) {
                continue;
            }
            let d = (input.pos[attendee_id] - output.0[musician_id]).abs2();
            am_pairs.push((d, attendee_id, musician_id))
        }
    }
    am_pairs.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

    let mut cposs = vec![];
    for (_, attendee_id, musician_id) in am_pairs {
        let a = input.pos[attendee_id];
        let m = output.0[musician_id];
        let vec = (m - a).rot();
        let vec = vec * ((5.0 + eps) / vec.abs());

        for stage_side in 0..4 {
            let stage_line = get_stage_line(input, stage_side);

            for sign in [-1.0, 1.0] {
                let am_line = (a + vec * sign, m + vec * sign);
                if let Some(p) = P::pi_ll(stage_line, am_line) {
                    // 線分と近くないと候補点として意味ない
                    let d = P::dist2_sp((a, m), p).sqrt();
                    if d < 5.0 + eps * 10.0 {
                        cposs.push(p);
                    }
                }
            }
        }
    }
    cposs
}

/// 岩田さんいわく２と３は一緒に生成してほしいらしい
pub fn pattern23(input: &Input, output: &Output, config: &CandidateConfig) -> Vec<P> {
    let eps = 1e-6;

    let mut am_pairs = vec![];
    for attendee_id in 0..input.n_attendees() {
        for musician_id in 0..input.n_musicians() {
            if is_blocked_by_someone(input, &output.0, musician_id, attendee_id) {
                continue;
            }
            let d = (input.pos[attendee_id] - output.0[musician_id]).abs2();
            am_pairs.push((d, attendee_id, musician_id))
        }
    }
    am_pairs.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

    let mut cposs = vec![];
    for (_, attendee_id, musician_id) in am_pairs {
        let attendee_pos = input.pos[attendee_id];
        let musician_pos = output.0[musician_id];

        // Pattern 2
        for p in P::tan_cp((musician_pos, 5.0 + eps), attendee_pos) {
            for q in P::pi_cl((musician_pos, 10.0 + eps), (attendee_pos, p)) {
                // attendee_pos -> q がブロックされてたらこれいらないでしょっていう
                if config.pattern2_disallow_blocked
                    && is_blocked_by_existing(input, output, attendee_pos, q)
                {
                    continue;
                }

                // musicianより更に近くにおけるんだったら何かがおかしい（意味ない）
                if (q - attendee_pos).abs2() < (musician_pos - attendee_pos).abs2() {
                    continue;
                }

                cposs.push(q);
            }
        }

        // Pattern 3
        let vec = (musician_pos - attendee_pos).rot();
        let vec = vec * ((5.0 + eps) / vec.abs());
        for stage_side in 0..4 {
            let stage_line = get_stage_line(input, stage_side);

            for sign in [-1.0, 1.0] {
                let am_line = (attendee_pos + vec * sign, musician_pos + vec * sign);
                if let Some(p) = P::pi_ll(stage_line, am_line) {
                    // 線分と近くないと候補点として意味ない
                    let d = P::dist2_sp((attendee_pos, musician_pos), p).sqrt();
                    if d < 5.0 + eps * 10.0 {
                        cposs.push(p);
                    }
                }
            }
        }
    }
    cposs
}

fn is_integer(n: f64) -> bool {
    n.fract() == 0.0
}

/// stage際とmusicianに接するやつ
pub fn pattern4(input: &Input, output: &Output) -> Vec<P> {
    let eps = 1e-6;
    let mut cposs = vec![];

    for side in 0..4 {
        for musician_pos in &output.0 {
            let line = get_stage_line(input, side);
            let (x, y) = (musician_pos.0, musician_pos.1);

            if line.0 .0 == x && line.1 .0 == x && is_integer(y) {
                cposs.extend([P(x, y + 10.0), P(x, y - 10.0)]);
            } else if line.0 .1 == y && line.1 .1 == y && is_integer(x) {
                cposs.extend([P(x + 10.0, y), P(x - 10.0, y)]);
            } else {
                cposs.extend(P::pi_cl((*musician_pos, 10.0 + eps), line));
            }
        }
    }
    cposs
}

///////////////////////////////////////////////////////////////////////////////
/// Filters
///////////////////////////////////////////////////////////////////////////////

pub fn filter_outside(mut cp: Vec<P>, input: &Input) -> Vec<P> {
    let len_old = cp.len();
    cp = cp.into_iter().filter(|p| input.in_stage(*p)).collect();
    eprintln!("Filter valid: {} -> {}", len_old, cp.len());
    cp
}

pub fn filter_by_reach(mut cp: Vec<P>, input: &Input, output: &Output) -> Vec<P> {
    let len_old = cp.len();
    cp = cp
        .into_iter()
        .filter(|p| does_reach_some_audiences(input, output, *p))
        .collect();
    eprintln!("Filter by reach: {} -> {}", len_old, cp.len());
    cp
}

pub fn filter_by_intersections(mut cp: Vec<P>, output: &Output, limit: usize) -> Vec<P> {
    let len_old = cp.len();
    cp = cp
        .into_iter()
        .filter(|p| {
            let mut n_intersections = 0;
            for q in &output.0 {
                if (*p - *q).abs2() < 100.0 {
                    n_intersections += 1;
                }
            }
            n_intersections <= limit
        })
        .collect();
    eprintln!("Filter by intersections: {} -> {}", len_old, cp.len());
    cp
}

///////////////////////////////////////////////////////////////////////////////
/// Interface
///////////////////////////////////////////////////////////////////////////////

pub fn enumerate_candidate_positions_with_config(
    input: &Input,
    output: &Output,
    config: &CandidateConfig,
) -> Vec<P> {
    let mut cp1 = vec![];
    let mut cp2 = vec![];
    let mut cp3 = vec![];
    let mut cp4 = vec![];
    let mut cp23 = vec![];

    // Generation
    if config.use_pattern1 {
        cp1 = filter_outside(pattern1(input, output), input);
        if let Some(limit) = config.filter_by_intersections1 {
            cp1 = filter_by_intersections(cp1, output, limit);
        }
        if config.filter_by_reach14 {
            cp1 = filter_by_reach(cp1, input, output);
        }
    }
    if config.use_pattern2 {
        cp2 = filter_outside(pattern2(input, output), input);
        if let Some(limit) = config.filter_by_intersections234 {
            cp2 = filter_by_intersections(cp2, output, limit);
        }
        if let Some(limit) = config.limit_pattern2 {
            cp2 = cp2.into_iter().take(limit * 10).collect();
        }
        if config.filter_by_reach23 {
            cp2 = filter_by_reach(cp2, input, output);
        }
        if let Some(limit) = config.limit_pattern2 {
            cp2 = cp2.into_iter().take(limit).collect();
        }
    }
    if config.use_pattern3 {
        cp3 = filter_outside(pattern3(input, output), input);
        if let Some(limit) = config.filter_by_intersections234 {
            cp3 = filter_by_intersections(cp3, output, limit);
        }
        if let Some(limit) = config.limit_pattern3 {
            cp3 = cp3.into_iter().take(limit * 10).collect();
        }
        if config.filter_by_reach23 {
            cp3 = filter_by_reach(cp3, input, output);
        }
        if let Some(limit) = config.limit_pattern3 {
            cp3 = cp3.into_iter().take(limit).collect();
        }
    }
    if config.use_pattern4 {
        cp4 = filter_outside(pattern4(input, output), input);
        if let Some(limit) = config.filter_by_intersections234 {
            cp4 = filter_by_intersections(cp4, output, limit);
        }
    }
    if config.use_pattern23 {
        cp23 = filter_outside(pattern23(input, output, config), input);
        if let Some(limit) = config.filter_by_intersections234 {
            cp23 = filter_by_intersections(cp23, output, limit);
        }
        if config.filter_by_reach23 {
            cp23 = filter_by_reach(cp23, input, output);
        }
        if let Some(limit) = config.limit_pattern23 {
            cp23 = cp23.into_iter().take(limit).collect();
        }
    }

    let (l1, l2, l3, l4, l23) = (cp1.len(), cp2.len(), cp3.len(), cp4.len(), cp23.len());

    let mut cp: Vec<_> = cp1
        .into_iter()
        .chain(cp2)
        .chain(cp3)
        .chain(cp4)
        .chain(cp23)
        .collect();
    cp.sort();
    cp.dedup();

    eprintln!(
        "Candidate sets: {} + {} + {} + {} + {} = {} -> {}",
        l1,
        l2,
        l3,
        l4,
        l23,
        (l1 + l2 + l3 + l4 + l23),
        cp.len()
    );

    cp
}

///////////////////////////////////////////////////////////////////////////////
// プリセット
///////////////////////////////////////////////////////////////////////////////

/// これは試運転山登り用設定のサンプル
pub fn enumerate_candidate_positions_hc(input: &Input, output: &Output) -> Vec<P> {
    enumerate_candidate_positions_with_config(
        input,
        output,
        &CandidateConfig {
            use_pattern1: true,
            use_pattern2: true,
            use_pattern3: true,
            use_pattern4: true,
            use_pattern23: false,
            limit_pattern2: Some(1000),
            limit_pattern3: Some(100),
            limit_pattern23: None,
            filter_by_intersections1: Some(0),
            filter_by_intersections234: Some(0),
            filter_by_reach23: true,
            filter_by_reach14: true,
            pattern2_disallow_blocked: true,
        },
    )
}

/// これは岩田さん用設定のサンプル
pub fn enumerate_candidate_positions_sa(input: &Input, output: &Output) -> Vec<P> {
    enumerate_candidate_positions_with_config(
        input,
        output,
        //     &CandidateConfig {
        //         use_pattern1: false,
        //         use_pattern2: false,
        //         use_pattern3: false,
        //         use_pattern4: false,
        //         use_pattern23: true,
        //         limit_pattern2: None,
        //         limit_pattern3: None,
        //         limit_pattern23: Some(100),
        //         filter_by_intersections1: Some(0),
        //         filter_by_intersections234: None,
        //         filter_by_reach23: false,
        //         filter_by_reach14: false,
        //         pattern2_disallow_blocked: false,
        //     },
        &CandidateConfig {
            use_pattern1: true,
            use_pattern2: false,
            use_pattern3: false,
            use_pattern4: true,
            use_pattern23: true,
            limit_pattern2: None,
            limit_pattern3: None,
            limit_pattern23: Some(1000),
            filter_by_intersections1: Some(0),
            filter_by_intersections234: None,
            filter_by_reach23: false,
            filter_by_reach14: false,
            pattern2_disallow_blocked: false,
        },
    )
}
