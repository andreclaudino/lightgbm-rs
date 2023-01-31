extern crate csv;
extern crate itertools;
extern crate lightgbm;
extern crate serde_json;

use itertools::zip;
use lightgbm::{Booster, Dataset};
use serde_json::json;

fn load_file(file_path: &str) -> (Vec<Vec<f64>>, Vec<f32>) {
    let rdr = csv::ReaderBuilder::new()
        .has_headers(false)
        .delimiter(b'\t')
        .from_path(file_path);
    let mut labels: Vec<f32> = Vec::new();
    let mut features: Vec<Vec<f64>> = Vec::new();
    for result in rdr.unwrap().records() {
        let record = result.unwrap();
        let label = record[0].parse::<f32>().unwrap();
        let feature: Vec<f64> = record
            .iter()
            .map(|x| x.parse::<f64>().unwrap())
            .collect::<Vec<f64>>()[1..]
            .to_vec();
        labels.push(label);
        features.push(feature);
    }
    (features, labels)
}

fn main() -> std::io::Result<()> {
    let (train_features, train_labels) =
        load_file("../../lightgbm-sys/lightgbm/examples/binary_classification/binary.train");
    let (test_features, test_labels) =
        load_file("../../lightgbm-sys/lightgbm/examples/binary_classification/binary.test");
    let train_dataset = Dataset::from_mat(train_features, train_labels).unwrap();

    let params = json! {
        {
            "num_iterations": 100,
            "objective": "binary",
            "metric": "auc"
        }
    };

    let booster = Booster::train(train_dataset, &params).unwrap();
    let result = booster.predict(test_features).unwrap();

    let mut tp = 0;
    for (label, pred) in zip(&test_labels, &result[0]) {
        if (*label == 1_f32 && *pred > 0.5_f64) || (*label == 0_f32 && *pred <= 0.5_f64) {
            tp += 1;
        }
        println!("{}, {}", label, pred)
    }
    println!("feature importance");
    let feature_name = booster.feature_name().unwrap();
    let feature_importance = booster.feature_importance().unwrap();
    for (feature, importance) in zip(&feature_name, &feature_importance) {
        println!("{}: {}", feature, importance);
    }
    println!("result: {} / {}", &tp, result[0].len());
    Ok(())
}
