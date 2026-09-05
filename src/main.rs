use std::fs::File;
use std::io::{self, BufRead};

#[derive(Debug)]
struct Sample {
    features: [f64; 4],
    label: String,
}

fn main() {
    let path: &str = "./data/iris.csv";

    let mut dataset: Vec<Sample> = Vec::new();

    match File::open(path) {
        Ok(file) => {
            let reader = io::BufReader::new(file);

            for line_result in reader.lines() {
                match line_result {
                    Ok(line) => {
                        let fields: Vec<&str> = line.split(',').collect();

                        let sepal_length: f64 = fields[0].trim().parse().unwrap();
                        let sepal_width: f64 = fields[1].trim().parse().unwrap();
                        let petal_length: f64 = fields[2].trim().parse().unwrap();
                        let petal_width: f64 = fields[3].trim().parse().unwrap();
                        let species: &str = fields[4].trim();

                        // println!(
                        //     "Sepal: {}-{}, Petal: {}-{}, Species: {}",
                        //     sepal_length, sepal_width, petal_length, petal_width, species
                        // )

                        let sample = Sample {
                            features: [
                                sepal_length,
                                sepal_width,
                                petal_length,
                                petal_width,
                            ], 
                            label: species.to_string(),
                        };

                        // sample struct needs to implement Debug macro to be printed in dbg mode
                        println!("{:?}", sample);

                        // pushing each sample in dataset vector
                        dataset.push(sample);
                    }
                    Err(e) => {
                        println!("Error reading line: {}", e);
                    }
                }
            }
        }
        Err(e) => {
            println!("Error opening file '{}': {}", path, e);
        }
    }

    // checking if the samples are pushed into vector
    // println!("{:?}", dataset); <- only for debugging, 

    println!("Loaded {} samples", dataset.len());
}
