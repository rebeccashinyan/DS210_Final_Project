//preprocessing.rs
//This module removes bad data and adjusts happiness-related values so countries can be compared fairly

use crate::data_loader::CountryData;

/*
1.What it does
   This function removes any rows that have missing or invalid numbers.
2.Inputs and outputs
   Input: A reference to a list of CountryData
   Output: A new list with only valid entries
3.Core logic and key components
   This function keeps only the countries where all 5 feature values are real numbers, excluding those with NaN or infinity.
   Then, the function returns a clean list with those valid entries.
*/
pub fn filter_valid_data(data: &Vec<CountryData>)->Vec<CountryData>{
    //keep only rows where all five feature values are finite (not NaN or infinity)
    data.iter().filter(|entry|{
            entry.social_support.is_finite()&& entry.life_expectancy.is_finite()
                && entry.freedom.is_finite()&& entry.generosity.is_finite() && entry.corruption.is_finite()
        })
        .cloned()
        .collect()
}

/*
1.What it does
   This function helps to scale each feature so values can be set between 0.0 to 1.0,
   fixing the inconsistency of the value throughout the different year's dataset.
2.Inputs and outputs
   Input: A mutable slice of CountryData
   Output: None (the function directly changes the input data)
3.Core logic and key components
   For each feature, the function finds the smallest and largest values
   and rescale every value using the formula normalized=(value-min)/(max-min).
   The function skips normalization if all values are the same in order to avoid dividing by zero.
*/
pub fn normalize_features(data: &mut [CountryData]){
    fn normalize_column<FGet, FSet>(data: &mut[CountryData], get:FGet, set:FSet) where FGet: Fn(&CountryData) -> f64, FSet: Fn(&mut CountryData, f64),{
        //find the min and max values for the feature
        let (min, max) = data.iter().fold((f64::INFINITY, f64::NEG_INFINITY), |(min, max), d| {
            let val = get(d);
            (min.min(val), max.max(val))
        });

        //only normalize if values are not all the same to avoid division by zero
        if (max - min).abs()>f64::EPSILON{
            for d in data.iter_mut(){
                //apply normalization formula (value - min) / (max - min)
                let raw=get(d);
                let norm=(raw-min)/(max-min);
                set(d, norm);
            }
        }
    }
    
    //normalize each of the five features individually
    normalize_column(data, |d|d.social_support, |d, v|d.social_support=v);
    normalize_column(data, |d|d.life_expectancy, |d, v|d.life_expectancy=v);
    normalize_column(data, |d|d.freedom, |d, v|d.freedom=v);
    normalize_column(data, |d|d.generosity, |d, v|d.generosity=v);
    normalize_column(data, |d|d.corruption, |d, v|d.corruption=v);
}