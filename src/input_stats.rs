use serde::Serialize;

use crate::Input;

pub fn get_stats(input: &Input) -> (MusiciansInfo, AttendeesInfo) {
    let stage_wh = input.stage1 - input.stage0;
    let stage_area = stage_wh.0 * stage_wh.1;
    let n_musicians = input.n_musicians();
    let n_attendees = input.n_attendees();
    let n_instruments = input.n_instruments();
    // assert_eq!(n_instruments, BTreeSet::from_iter(input.musicians.clone()).len());
    // assert_eq!(n_instruments, input.musicians.iter().max().unwrap() + 1);

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
        stats_musicians_per_instrument: Stats::from_iter(&n_musicians_per_instrument),
    };
    let attendees_info = AttendeesInfo {
        n_attendees,
        stats_tastes: Stats::from_iter(&all_tastes),
    };
    // println!("{:?}", (musicians_info, attendees_info));
    (musicians_info, attendees_info)
}

#[derive(Serialize)]
pub struct Stats {
    pub mean: f64,
    pub std: f64,
    pub min: f64,
    pub max: f64,
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

#[derive(Debug, Serialize)]
#[allow(dead_code)] // has a derived impl for the trait `Debug`, but this is intentionally ignored during dead code analysis
pub struct MusiciansInfo {
    // #[serde(rename = "Mus")]
    pub n_musicians: usize,
    pub area_per_musician: f64,
    pub n_instruments: usize,
    // #[serde(with = "stats1")]
    pub stats_musicians_per_instrument: Stats,
}

// serde_with::with_prefix!(stats1 "Mus/Ins");

// impl MusiciansInfo {
//     pub fn header() -> Vec<String> {
//         let mut out = vec![
//             "#Mus".to_string(),
//             "Area/Mus".to_string(),
//             "#Ins".to_string(),
//         ];
//         out.extend(["mean", "std", "min", "max"].iter().map(|k| format!("#Mus/Ins.{}", k)));
//         out
//     }

//     pub fn to_strings(&self) -> Vec<String> {
//         let mut out = vec![
//             self.n_musicians.to_string(),
//             self.area_per_musician.to_string(),
//             self.n_instruments.to_string(),
//         ];
//         let stats = &self.stats_musicians_per_instrument;
//         out.extend(
//             [
//                 stats.mean,
//                 stats.std,
//                 stats.min,
//                 stats.max,
//             ]
//             .iter()
//             .map(|x| x.to_string()),
//         );
//         out
//     }
// }

#[derive(Debug, Serialize)]
#[allow(dead_code)] // has a derived impl for the trait `Debug`, but this is intentionally ignored during dead code analysis
pub struct AttendeesInfo {
    pub n_attendees: usize,
    pub stats_tastes: Stats,
}

fn mean_and_std<T: Copy + Into<f64>>(data: &[T]) -> (f64, f64) {
    let data = data.iter().map(|&x| x.into()).collect::<Vec<_>>();
    let n = data.len() as f64;
    let mean = data.iter().sum::<f64>() / n;
    let std = (data.iter().map(|&x| (x - mean).powi(2)).sum::<f64>() / n).sqrt();
    (mean, std)
}
