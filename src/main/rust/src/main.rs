use directories::UserDirs;
use std::{fs, os::unix::fs::MetadataExt, time::Instant};

const FILE_PATH: &str = "useCode/1brc/data/measurements.txt";

fn main() {
    let start_at = Instant::now();

    let binding = UserDirs::new().unwrap();
    let home_dir = binding.home_dir();

    let measurements_path = home_dir.join(FILE_PATH);

    let medadata = fs::metadata(&measurements_path).expect(&format!("Can't open FILE {}", measurements_path.to_string_lossy()));
    println!("{}", medadata.size());

    // let file = File::open(measurements_path).unwrap();

    // let reader = BufReader::new(file);
    // let mut lines_count = 0;

    // for line in reader.lines() {
    //     if let Ok(_line) = line {
    //         lines_count += 1;
    //     } else {
    //         panic!("Error reading lines");
    //     }
    // }

    // println!("Total lines count: {}", lines_count);

    println!("Total run time: {:?}", start_at.elapsed());
}
