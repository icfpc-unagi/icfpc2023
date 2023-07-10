use icfpc2023::{problem_id_from_path, read_input_from_file, P};

fn main() {
    // std::env::args().skip(1).try_for_each(|path| main1(&path)).unwrap();
    let mut paths: Vec<String> = std::env::args().skip(1).collect();
    paths.sort_by_cached_key(|path| problem_id_from_path(path));
    paths.iter().try_for_each(|path| main1(path)).unwrap();
}
fn main1(path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let input = read_input_from_file(path);
    let stage0 = input.stage0 + P(10.0, 10.0);
    let stage1 = input.stage1 - P(10.0, 10.0);

    let mut res = 0.0;
    // let mut sum1 = 0.0;
    // let mut sum2 = 0.0;
    for (i, &p) in input.pos.iter().enumerate() {
        // if i != 24 {continue}
        // relative
        let st0 = stage0 - p;
        let st1 = stage1 - p;

        let tastes = &input.tastes[i];

        // dist(p, stage) and its vector
        let distx = 0.0f64.max(st0.0).max(-st1.0);
        let disty = 0.0f64.max(st0.1).max(-st1.1);
        let d2 = P(distx, disty).abs2();

        // dbg!(st0, st1);
        // dbg!(distx, disty, d2);

        let mut ub1 = 0.0;
        for (j, taste) in tastes.iter().copied().enumerate() {
            if taste > 0.0 {
                let cnt = input.inst_musicians[j].len();
                ub1 += cnt as f64 * (taste / d2);
            }
        }

        let max_taste = tastes.iter().copied().fold(f64::NEG_INFINITY, f64::max);
        let max_taste = max_taste.max(0.0);
        let ii = integ(distx, st0.1, st1.1) + integ(disty, st0.0, st1.0);
        let ub2 = max_taste * ii / 5.0;
        // dbg!(ub1, ub2);
        res += ub1.min(ub2);

        // sum1 += ub1;
        // sum2 += ub2;
    }
    // dbg!(sum1);
    // dbg!(sum2);
    // dbg!(res);
    println!("{}", res);
    // println!("{}", res * 1e7);
    Ok(())
}

fn integ(c: f64, a: f64, b: f64) -> f64 {
    if c <= 0.0 {
        return 0.0;
    }
    // b.atan2(c) - a.atan2(c)

    let fa = a / (c * (c * c + a * a).sqrt());
    let fb = b / (c * (c * c + b * b).sqrt());
    fb - fa

    // let fa = c * a / (c * c + a * a) + a.atan2(c);
    // let fb = c * b / (c * c + b * b) + b.atan2(c);
    // dbg!(a, b, c, fa, fb);
    // (fb - fa) / (2.0 * c * c)
}
