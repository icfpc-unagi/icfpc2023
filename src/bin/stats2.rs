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

fn main1(paths: Vec<String> /*, writer: impl std::io::Write */) -> Result<(), Box<dyn std::error::Error>> {
    let flattener = flatten_json_object::Flattener::new();
    let flatten = |json| {
        flattener.flatten(&json).unwrap().as_object().unwrap().clone()
    };

    // let mut tsv_writer = csv::WriterBuilder::new()
    //     .delimiter(b'\t')
    //     .from_writer(writer);
    let mut keys: Option<Vec<String>> = None;
    for path in paths {
        let input = read_input_from_file(&path);
        // let data = get_stats(&input);
        let (musicians_info, attendees_info) = get_stats(&input);
        // let mut data: serde_json::Map<String, serde_json::Value> = flatten1(musicians_info)?;
        // data.extend(flatten2(attendees_info)?);
        let mut data = flatten(serde_json::to_value(&musicians_info)?);
        data.extend( flatten(serde_json::to_value(&attendees_info)?));
        if let Some(keys) = &keys {
            assert!(keys.iter().zip(data.keys()).all(|(k1, k2)| k1 == k2));
        } else {
            keys = Some(data.keys().cloned().collect_vec());
            println!("{}", keys.clone().unwrap().join("\t"));
        }
        println!("{}", data.values().join("\t"));
            // dbg!(flat_data.keys().collect_vec());
            // dbg!(flat_data.values().collect_vec());    
        // let (keys, values) = flat_data.as_object().unwrap().iter().cloned().unzip();
        // tsv_writer.serialize(&flat_data)?;
        // println!("{}", flat_data);
        // if write_header {
        //     println!(
        //         "id\t#Mus\t#Att\tMus/Att\tMusArea\tMusInst\tMusInstStats\tAttTasteStats"
        //     ); //todo
        //     write_header = false;
        // }
        // println!("{}\t{}\t{}\t{}\t{}\t{}")
        // println!("{:?}", (musicians_info, attendees_info));
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
