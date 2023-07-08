use std::path::Path;
use std::str::FromStr;

use icfpc2023::{input_stats::*, read_input_from_file};
use itertools::Itertools;

fn main() {
    let mut paths: Vec<String> = std::env::args().skip(1).collect();
    paths.sort_by_cached_key(|path| extract_number_from_path(path));
    // let writer = std::io::stdout();
    main1(paths).unwrap();
}

fn main1(
    paths: Vec<String>, /*, writer: impl std::io::Write */
) -> Result<(), Box<dyn std::error::Error>> {
    let mut keys0 = None;
    for path in paths {
        let mut keys = vec!["id".to_string()];
        let mut values = vec![extract_number_from_path(&path).to_string()];
        let input = read_input_from_file(&path);
        let data = get_stats(&input);
        let dbgstr = format!("{:?}", data);
        let mut iter = dbgstr.chars();
        loop {
            let key = iter
                .by_ref()
                .take_while(|&c| c != ':')
                .collect::<String>()
                .split(' ')
                .last()
                .unwrap()
                .to_string();
            if key.is_empty() {
                break;
            }
            let value = iter
                .by_ref()
                .take_while(|&c| !",}".contains(c))
                .collect::<String>()
                .trim()
                .to_string();
            if value.is_empty() {
                break;
            }
            keys.push(key.to_string());
            values.push(value);
        }
        let keys = keys.join("\t");
        let values = values.join("\t");
        if let Some(keys0) = keys0.as_ref() {
            assert_eq!(keys0, &keys);
        } else {
            println!("{}", keys);
            keys0 = Some(keys.clone());
        }
        println!("{}", values);
        // println!("{}", dbgstr);
        // println!("{:?} {:?}", keys, values);
        // println!("{}", data.values().join("\t"));
    }
    Ok(())
}

#[allow(dead_code)]
fn main1_old(
    paths: Vec<String>, /*, writer: impl std::io::Write */
) -> Result<(), Box<dyn std::error::Error>> {
    let flattener = flatten_json_object::Flattener::new();
    let flatten = |json| {
        flattener
            .flatten(&json)
            .unwrap()
            .as_object()
            .unwrap()
            .clone()
    };

    // let mut tsv_writer = csv::WriterBuilder::new()
    //     .delimiter(b'\t')
    //     .from_writer(writer);
    let mut keys: Option<Vec<String>> = None;
    for path in paths {
        let mut data = vec![serde_json::json!({ "id": extract_number_from_path(&path) })];
        let input = read_input_from_file(&path);
        // let data = get_stats(&input);
        let (musicians_info, attendees_info) = get_stats(&input);
        data.extend([
            serde_json::to_value(&musicians_info)?,
            serde_json::to_value(&attendees_info)?,
        ]);
        // dbg!(&data);
        let data: serde_json::Map<String, serde_json::Value> = data
            .into_iter()
            .flat_map(|x| flatten(x).into_iter())
            .collect();
        if let Some(keys) = &keys {
            assert!(keys.iter().zip(data.keys()).all(|(k1, k2)| k1 == k2));
        } else {
            keys = Some(data.keys().cloned().collect_vec());
            println!("{}", keys.clone().unwrap().join("\t"));
        }
        println!("{}", data.values().join("\t"));
    }
    Ok(())
}

// fn write_row<W, T>(&mut csv_writer: csv::Writer<W>, value: T) -> Result<(), Box<dyn std::error::Error>>
// where T: Serialize,
//       W: std::io::Write,
// {
//     let json = serde_json::to_value(value)?;
//     csv_writer.serialize(flattener.flatten(&json))?;
//     Ok(())
// }

// from bin/stats.rs
fn extract_number_from_path(path: &str) -> i32 {
    let filename = Path::new(path).file_name().unwrap().to_str().unwrap();

    let start = filename.find('-').unwrap() + 1;
    let end = filename.find('.').unwrap();

    i32::from_str(&filename[start..end]).unwrap()
}
