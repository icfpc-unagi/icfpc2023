use std::collections::BTreeSet;

use icfpc2023::read_input;

fn main() {
    let input = read_input();
    let stage_wh = input.stage1 - input.stage0;
    let stage_area = stage_wh.0 * stage_wh.1;
    let n_musicians = input.n_musicians();
    let n_attendees = input.n_attendees();
    let n_instruments = BTreeSet::from_iter(input.musicians.clone()).len();
    assert_eq!(n_instruments, input.n_instruments());
    assert_eq!(n_instruments, input.musicians.iter().max().unwrap() + 1);

    let area_per_musician = stage_area / n_musicians as f64;
    let n_musicians_per_instrument =
        input
            .musicians
            .iter()
            .fold(vec![0; n_instruments], |mut acc, &m| {
                acc[m] += 1;
                acc
            });

    let all_tastes = input.tastes.iter().flatten().copied().collect::<Vec<_>>();

    let musicians_info = MusiciansInfo {
        n_musicians,
        area_per_musician,
        n_instruments,
        stats_musician_per_instrument: Stats::from_iter(&n_musicians_per_instrument),
    };
    let attendees_info = AttendeesInfo {
        n_attendees,
        stats_tastes: Stats::from_iter(&all_tastes),
    };
    println!("{:?}", (musicians_info, attendees_info));
}

struct Stats {
    mean: f64,
    std: f64,
    min: f64,
    max: f64,
}

impl Stats {
    fn to_string(&self) -> String {
        format!(
            "{} ± {} ({} .. {})",
            self.mean as f32, self.std as f32, self.min, self.max
        )
    }
}

impl std::fmt::Debug for Stats {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

impl<'a, T> FromIterator<&'a T> for Stats
where
    T: Copy + 'a,
    f64: From<T>,
{
    fn from_iter<I: IntoIterator<Item = &'a T>>(iter: I) -> Self {
        let data = iter
            .into_iter()
            .copied()
            .map(|x| f64::from(x))
            .collect::<Vec<_>>();
        let (mean, std) = mean_and_std(&data);
        let min = data.iter().copied().fold(f64::INFINITY, f64::min);
        let max = data.iter().copied().fold(f64::NEG_INFINITY, f64::max);
        Self {
            mean,
            std,
            min,
            max,
        }
    }
}

#[derive(Debug)]
#[allow(dead_code)] // has a derived impl for the trait `Debug`, but this is intentionally ignored during dead code analysis
struct MusiciansInfo {
    n_musicians: usize,
    area_per_musician: f64,
    n_instruments: usize,
    stats_musician_per_instrument: Stats,
}

#[derive(Debug)]
#[allow(dead_code)] // has a derived impl for the trait `Debug`, but this is intentionally ignored during dead code analysis
struct AttendeesInfo {
    n_attendees: usize,
    stats_tastes: Stats,
}

fn mean_and_std<T: Copy + Into<f64>>(data: &[T]) -> (f64, f64) {
    let data = data.iter().map(|&x| x.into()).collect::<Vec<_>>();
    let n = data.len() as f64;
    let mean = data.iter().sum::<f64>() / n;
    let std = (data.iter().map(|&x| (x - mean).powi(2)).sum::<f64>() / n).sqrt();
    (mean, std)
}
