use std::fs::File;
use std::io::{self, BufRead};
// for seeded randomness
use rand::rngs::StdRng;
use rand::seq::SliceRandom;
use rand::SeedableRng;

#[derive(Debug)]
struct Sample {
    features: [f64; 4],
    label: usize,
}

// encodes string label into numeric form
fn encode_label(label: &str) -> usize {
    match label {
        "Iris-setosa" => 0,
        "Iris-versicolor" => 1,
        "Iris-virginica" => 2,
        _ => panic!("Unknown species : {}", label),
    }
}

// decode numeric labels into string form
fn decode_label(label: usize) -> &'static str {
    match label {
        0 => "Iris-setosa",
        1 => "Iris-versicolor",
        2 => "Iris-virginica",
        _ => panic!("Unknown Label : {}", label),
    }
}

// function to calculate mean 
// taking immutable reference to slice of Sample vector
fn calculate_mean(data: &[Sample]) -> [f64; 4] {
    let length = data.len() as f64;
    let mut feature_1_sum = 0.0;
    let mut feature_2_sum = 0.0;
    let mut feature_3_sum = 0.0;
    let mut feature_4_sum = 0.0;

    for item in data {
        feature_1_sum += item.features[0];
        feature_2_sum += item.features[1];
        feature_3_sum += item.features[2];
        feature_4_sum += item.features[3];
    }

    [
        feature_1_sum / length,
        feature_2_sum / length,
        feature_3_sum / length,
        feature_4_sum / length,
    ]
}

// fn calculate_std_dev(data: &[Sample], mean: ) -> [f64; 4] {

// }

fn main() {
    let path: &str = "./data/iris.csv";

    let mut dataset: Vec<Sample> = Vec::new();

    // random seed 
    let mut rng = StdRng::seed_from_u64(42);
    dataset.shuffle(&mut rng);

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

                        // encoding the labels 
                        let label = encode_label(species);

                        let sample = Sample {
                            features: [
                                sepal_length,
                                sepal_width,
                                petal_length,
                                petal_width,
                            ], 
                            label,
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

    // debug only 
    // checking the encode & decode fns 
    // dbg!(encode_label("Iris-virginica"));
    // dbg!(decode_label(0));

    // splitting the dataset into training & testing 
    let split_index = (dataset.len() as f64 * 0.8) as usize;
    let train_data = &dataset[..split_index];
    let test_data = &dataset[split_index..];

    // calculating mean & std. dev for training dataset
    // means of all features as array 
    let means = calculate_mean(train_data);
    // let std_dev = calculate_std_dev(train_data, &means);

    println!("mean of features : {:?}", means);
    
    // scaled train, scaled test 
    // let scaled_train = scale_data(train_data, &means, &std_dev);
    // let scaled_test = scale_data(test_data, &means, &std_dev);

}
