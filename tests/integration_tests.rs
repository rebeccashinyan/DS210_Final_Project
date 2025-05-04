//tests/integration_tests.rs
//tests the main modules

use ds210_final_project::{
    data_loader::{CountryData, canonical_country_name},
    preprocessing::{filter_valid_data, normalize_features},
    clustering::{kmeans, calculate_inertia},
};

//test for canonical country name correction
#[test]
fn test_country_name_fixing(){
    assert_eq!(canonical_country_name("hong kong s.a.r., china"), "hong kong");
    assert_eq!(canonical_country_name("Taiwan Province of China"), "taiwan");
    assert_eq!(canonical_country_name("  United States of America "), "united states");
    assert_eq!(canonical_country_name("France"), "france"); //should remain unchanged
}

//test to check that invalid entries are filtered correctly
#[test]
fn test_remove_invalid_rows(){
    let input_data=vec![
        CountryData{country:"A".to_string(), social_support:0.5, life_expectancy:0.6, freedom:0.7, generosity:0.3, corruption:0.2},
        CountryData{country:"B".to_string(), social_support:f64::NAN, life_expectancy:0.6, freedom:0.7, generosity:0.3, corruption:0.2},
    ];
    let filtered=filter_valid_data(&input_data);
    assert_eq!(filtered.len(), 1);
    assert_eq!(filtered[0].country, "A");
}

//test that normalization scales values to between 0.0 and 1.0
#[test]
fn test_normalize_between_0_and_1(){
    let mut sample_data = vec![
        CountryData { country:"A".to_string(), social_support:1.0, life_expectancy:2.0, freedom:3.0, generosity:4.0, corruption:5.0},
        CountryData { country:"B".to_string(), social_support:2.0, life_expectancy:4.0, freedom:6.0, generosity:8.0, corruption:10.0},
    ];
    normalize_features(&mut sample_data);
    for country in &sample_data{
        assert!((0.0..=1.0).contains(&country.social_support));
        assert!((0.0..=1.0).contains(&country.life_expectancy));
        assert!((0.0..=1.0).contains(&country.freedom));
        assert!((0.0..=1.0).contains(&country.generosity));
        assert!((0.0..=1.0).contains(&country.corruption));
    }
}

//test that kmeans returns correct number of clusters and labels
#[test]
fn test_kmeans_returns_right_size(){
    let input=vec![
        CountryData{country: "A".to_string(), social_support: 0.1, life_expectancy: 0.1, freedom: 0.1, generosity: 0.1, corruption: 0.1 },
        CountryData{country: "B".to_string(), social_support: 0.9, life_expectancy: 0.9, freedom: 0.9, generosity: 0.9, corruption: 0.9 },
    ];
    let (labels, centers) = kmeans(&input, 2, 5, 42);
    assert_eq!(labels.len(), input.len());
    assert_eq!(centers.len(), 2);
}

//test that inertia is never negative
#[test]
fn test_inertia_is_positive(){
    let points=vec![
        CountryData{country: "A".to_string(), social_support:0.1, life_expectancy:0.2, freedom:0.3, generosity:0.4, corruption:0.5},
        CountryData{country: "B".to_string(), social_support:0.5, life_expectancy:0.6, freedom:0.7, generosity:0.8, corruption:0.9},
    ];
    let (labels, centers)=kmeans(&points, 2, 5, 100);
    let inertia=calculate_inertia(&points, &centers, &labels);
    assert!(inertia>=0.0);
}
