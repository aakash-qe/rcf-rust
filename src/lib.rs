extern crate rcflib;

use rcflib::{
    common::multidimdatawithkey::MultiDimDataWithKey,
    rcf::{RCF},
};
use rcflib::rcf::{RCFBuilder, RCFOptionsBuilder};
use std::error::Error;

use pyo3::prelude::*;
use pyo3::types::PyList;

fn make_forest() -> Result<Box<dyn RCF>, Box<dyn Error>> {
    let shingle_size = 8;
    let base_dimension = 3;
    let data_size = 100000;
    let number_of_trees = 30;
    let capacity = 256;
    let initial_accept_fraction = 0.1;
    let _point_store_capacity = capacity * number_of_trees + 1;
    let time_decay = 0.1 / capacity as f64;
    let bounding_box_cache_fraction = 1.0;
    let random_seed = 17;
    let parallel_enabled: bool = false;
    let store_attributes: bool = false;
    let internal_shingling: bool = true;
    let internal_rotation = false;
    let noise = 5.0;

    let mut forest: Box<dyn RCF> = RCFBuilder::<u64,u64>::new(base_dimension,shingle_size)
        .tree_capacity(capacity)
        .number_of_trees(number_of_trees)
        .random_seed(random_seed)
        .store_attributes(store_attributes)
        .parallel_enabled(parallel_enabled)
        .internal_shingling(internal_shingling)
        .internal_rotation(internal_rotation)
        .time_decay(time_decay)
        .initial_accept_fraction(initial_accept_fraction)
        .bounding_box_cache_fraction(bounding_box_cache_fraction).build_default().unwrap();
    
    return Ok(forest)
}

fn score_and_update_vec(values: &Vec<Vec<f32>>) -> Result<Vec<f64>, Box<dyn Error>> {
    let mut forest = make_forest().unwrap();
    let mut scores: Vec<f64> = Vec::new();

    for i in 0..values.len() {
        let new_score = forest.score(&values[i]).unwrap();
        scores.push(new_score);
        forest.update(&values[i], 0).unwrap();
    }

    println!("Success! {}", forest.entries_seen());
    println!("PointStore Size {} ", forest.point_store_size());
    println!("Total size {} bytes (approx)", forest.size());

    return Ok(scores)
}

fn score_values(values: &Vec<Vec<f32>>) -> Result<Vec<f64>, Box<dyn Error>> {
    let mut forest = make_forest().unwrap();
    let mut scores: Vec<f64> = Vec::new();

    for i in 0..values.len() {
        let new_score = forest.score(&values[i]).unwrap();
        scores.push(new_score);
    }

    println!("Success! {}", forest.entries_seen());
    println!("PointStore Size {} ", forest.point_store_size());
    println!("Total size {} bytes (approx)", forest.size());

    return Ok(scores)
}

// Define the Python binding for the Rust function
#[pyfunction]
fn score_and_update_values(py: Python, pylist: &PyList) -> PyResult<Py<PyList>> {
    let vec: Vec<Vec<f32>> = pylist.extract().expect("Expected a List of List of Integers");

    println!("Hello from Rust");
    let scores = score_and_update_vec(&vec);
    
    let py_list = PyList::new(py, scores);
    Ok(py_list.into())
}

// Create a Python module
#[pymodule]
fn my_rust_lib(py: Python, m: &PyModule) -> PyResult<()> {
    // Add the Rust function to the Python module
    m.add_function(wrap_pyfunction!(score_and_update_values, m)?)?;
    Ok(())
}