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
    pub filter_by_reach: bool,
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
            if is_blocked_by_someone(input, output, musician_id, attendee_id) {
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
            if is_blocked_by_someone(input, output, musician_id, attendee_id) {
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
            if is_blocked_by_someone(input, output, musician_id, attendee_id) {
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

/// stage際とmusicianに接するやつ
pub fn pattern4(input: &Input, output: &Output) -> Vec<P> {
    let eps = 1e-6;
    let mut cposs = vec![];

    for side in 0..4 {
        for musician_pos in &output.0 {
            let line = get_stage_line(input, side);
            cposs.extend(P::pi_cl((*musician_pos, 10.0 + eps), line));
        }
    }
    cposs
}

pub fn filter(mut cp: Vec<P>, input: &Input, output: &Output, config: &CandidateConfig) -> Vec<P> {
    let len_old = cp.len();
    cp = cp.into_iter().filter(|p| input.in_stage(*p)).collect();
    if config.filter_by_reach {
        cp = cp
            .into_iter()
            .filter(|p| does_reach_some_audiences(input, output, *p))
            .collect()
    }
    eprintln!("Filter: {} -> {}", len_old, cp.len());
    cp
}

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
        cp1 = filter(pattern1(input, output), input, output, config);
    }
    if config.use_pattern2 {
        cp2 = filter(pattern2(input, output), input, output, config);
        if let Some(limit) = config.limit_pattern2 {
            cp2 = cp2.into_iter().take(limit).collect();
        }
    }
    if config.use_pattern3 {
        cp3 = filter(pattern3(input, output), input, output, config);
        if let Some(limit) = config.limit_pattern3 {
            cp3 = cp3.into_iter().take(limit).collect();
        }
    }
    if config.use_pattern4 {
        cp4 = filter(pattern4(input, output), input, output, config);
    }
    if config.use_pattern23 {
        cp23 = filter(pattern23(input, output, config), input, output, config);
        if let Some(limit) = config.limit_pattern23 {
            cp23 = cp23.into_iter().take(limit).collect();
        }
    }
    eprintln!(
        "Candidate sets: {} + {} + {} + {} + {}",
        cp1.len(),
        cp2.len(),
        cp3.len(),
        cp4.len(),
        cp23.len(),
    );

    cp1.into_iter()
        .chain(cp2)
        .chain(cp3)
        .chain(cp4)
        .chain(cp23)
        .collect()
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
            filter_by_reach: true,
            pattern2_disallow_blocked: true,
        },
    )
}

/// これは岩田さん用設定のサンプル
pub fn enumerate_candidate_positions_sa(input: &Input, output: &Output) -> Vec<P> {
    enumerate_candidate_positions_with_config(
        input,
        output,
        &CandidateConfig {
            use_pattern1: true,
            use_pattern2: false,
            use_pattern3: false,
            use_pattern4: true,
            use_pattern23: true,
            limit_pattern2: None,
            limit_pattern3: None,
            limit_pattern23: Some(2000),
            filter_by_reach: false,
            pattern2_disallow_blocked: false,
        },
    )
}
