use clap::Parser;
use csv;
use intmap::IntMap;
use std::{error, fs, io, path};
use vcd;

type AnyError = Box<dyn error::Error>;

#[derive(Parser, Debug, Clone)]
struct CliArgs {
    input_file: path::PathBuf,
    output_file: path::PathBuf,
}

fn main() -> Result<(), AnyError> {
    let args = CliArgs::parse();

    let activity_map = {
        let input_file = fs::File::open(&args.input_file).map(|file| io::BufReader::new(file))?;

        let mut vcd_parser = vcd::Parser::new(input_file);
        let _vcd_header = vcd_parser.parse_header();

        let mut activity_map = IntMap::<u16>::new();
        let mut current_time: u64 = 0;

        for command in vcd_parser {
            let command = command?;

            match command {
                vcd::Command::Timestamp(val) => current_time = val,
                vcd::Command::ChangeReal(..)
                | vcd::Command::ChangeScalar(..)
                | vcd::Command::ChangeString(..)
                | vcd::Command::ChangeVector(..) => {
                    let entry = match activity_map.entry(current_time) {
                        intmap::Entry::Occupied(entry) => entry.into_mut(),
                        intmap::Entry::Vacant(entry) => entry.insert(0),
                    };
                    *entry += 1;
                }
                _ => {}
            }
        }
        activity_map
    };

    let mut activity_list: Vec<_> = activity_map.into_iter().collect();
    activity_list.sort_unstable_by_key(|(time, _)| *time);

    let mut csv_writer = csv::Writer::from_path(&args.output_file)?;

    csv_writer.write_record(&["time", "activity"])?;
    for (time, activity) in activity_list {
        csv_writer.write_record(&[time.to_string(), activity.to_string()])?;
    }

    Ok(())
}
