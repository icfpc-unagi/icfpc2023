use std::collections::BTreeSet;

use icfpc2023::read_input;

fn main() {
    let input = read_input();
    let stage_wh = input.stage1 - input.stage0;
    let stage_area = stage_wh.0 * stage_wh.1;
    let num_musician = input.n_musicians();
    let num_attendee = input.n_attendees();
    let num_instruments = BTreeSet::from_iter(input.musicians.clone()).len();
    assert_eq!(num_instruments, input.n_instruments());
    assert_eq!(num_instruments, input.musicians.iter().max().unwrap() + 1);

    let area_per_musician = stage_area / num_musician as f64;
    let num_musician_per_instrument =
        input
            .musicians
            .iter()
            .fold(vec![0; num_instruments], |mut acc, &m| {
                acc[m] += 1;
                acc
            });

    let all_tastes = input.tastes.iter().flatten().copied().collect::<Vec<_>>();

    let musicians_info = MusiciansInfo {
        num_musician,
        area_per_musician,
        num_instruments,
        stats_musician_per_instrument: Stats::from_iter(&num_musician_per_instrument),
    };
    let attendees_info = AttendeesInfo {
        num_attendee,
        stats_tastes: Stats::from_iter(&all_tastes),
    };
    println!("{:?}", (musicians_info, attendees_info))
    // println!(
    //     "{:?}",
    //     (
    //         num_musician,
    //         num_attendee,
    //         num_instruments,
    //         area_per_musician,
    //         Stats::from_iter(&num_musician_per_instrument).to_string(),
    //         Stats::from_iter(&all_tastes).to_string(),
    //         // mean_and_std(&num_musician_per_instrument),
    //         // mean_and_std(&all_tastes)
    //     )
    // );
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
    num_musician: usize,
    area_per_musician: f64,
    num_instruments: usize,
    stats_musician_per_instrument: Stats,
}

#[derive(Debug)]
#[allow(dead_code)] // has a derived impl for the trait `Debug`, but this is intentionally ignored during dead code analysis
struct AttendeesInfo {
    num_attendee: usize,
    stats_tastes: Stats,
}

fn mean_and_std<T: Copy + Into<f64>>(data: &[T]) -> (f64, f64) {
    let data = data.iter().map(|&x| x.into()).collect::<Vec<_>>();
    let n = data.len() as f64;
    let mean = data.iter().sum::<f64>() / n;
    let std = (data.iter().map(|&x| (x - mean).powi(2)).sum::<f64>() / n).sqrt();
    (mean, std)
}
