use std::fs::File;

// for IO 
use std::io::{self, BufRead, Write};

// for seeded randomness
use rand::rngs::StdRng;
use rand::seq::SliceRandom;
use rand::SeedableRng;

// get user input 
fn get_user_input() -> [f64; 4] {
    let mut features = [0.0; 4];

    let feature_names = [
        "Sepal length",
        "Sepal width",
        "Petal length",
        "Petal width",
    ];

    for i in 0..4 {
        loop {
            print!("Enter {} : ", feature_names[i]);
            io::stdout().flush().unwrap();

            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();

            match input.trim().parse::<f64>() {
                Ok(value) => {
                    features[i] = value;
                    break;
                }
                Err(_) => {
                    println!("Please enter a valid number...")
                }
            }

        }
    }

    features 
}

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

fn calculate_std_dev(data: &[Sample], means: &[f64; 4]) -> [f64; 4] {
  let mut squared_diffs = [0.0; 4];

  for item in data {
    for j in 0..4 {
      let diff = item.features[j] - means[j];
      squared_diffs[j] += diff * diff;
    }
  }

  let n = data.len() as f64;

  for j in 0..4 {
    squared_diffs[j] = (squared_diffs[j] / n).sqrt();
  }

  squared_diffs
}

// scaled samples struct <- tbh not required 
#[derive(Debug)]
struct ScaledSample {
    features: [f64; 4],
    label: usize,
}

// function to scale data 
// basically we need to iterate over each sample, 
// perform the og - mean / stddev calculation for 4 features in each sample 
// return a vector of scaled values
fn scale_data(
    data: &[Sample],
    means: &[f64; 4],
    stds: &[f64; 4],
) -> Vec<ScaledSample> {
    let mut scaled = Vec::with_capacity(data.len());

    for item in data {
        let mut features = [0.0; 4];

        for j in 0..4 {
            // this line directly corresponds to x_dash_ij = x_ij - mean_j / stddev_j
            features[j] = (item.features[j] - means[j]) / stds[j];
        }

        scaled.push(ScaledSample{
            features,
            label: item.label,
        });
    }

    scaled
}

fn euclidean_distance(a: &[f64; 4], b: &[f64; 4]) -> f64 {
    let mut sum = 0.0;

    for j in 0..4 {
        let diff = a[j] - b[j];
        sum += diff * diff;
    }

    sum.sqrt()
}

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
    let std_dev = calculate_std_dev(train_data, &means);

    println!("mean of features : {:?}", means);
    println!("std deviation of features : {:?}", std_dev);
    
    // scaled train, scaled test 
    let scaled_train = scale_data(train_data, &means, &std_dev);
    let scaled_test = scale_data(test_data, &means, &std_dev);

    // debug
    // dbg!(scaled_train);
    // dbg!(scaled_test);

    // debug <- only to check that euclidean distance formula works
    // let a: [f64; 4] = [1.0,2.0,3.0,4.0];
    // let b: [f64; 4] = [4.0,3.0,2.0,1.0];
    // let distance = euclidean_distance(&a, &b);
    // dbg!(distance);


    // get user inputs 
    let user_features = get_user_input();

    println!("User input : {:?}", user_features);

}
