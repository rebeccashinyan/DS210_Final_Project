//data_loader.rs
//This module reads and loads World Happiness data from multiple years into organized records and fix country name differences
use std::collections::HashMap;
use std::path::Path;
use csv::Reader;
use serde::Deserialize;


//this struct holds the datasets features that are going to be evaluated and handles inconsistent feature names across different years.
#[derive(Debug, Deserialize, Clone)]
pub struct CountryData {
    #[serde(rename="Country", alias="Country or region", alias="Country name")]
    pub country: String,

    #[serde(rename="Social support", alias="Family")]
    pub social_support: f64, 

    #[serde(rename="Healthy life expectancy", alias="Health (Life Expectancy)", alias="Health..Life.Expectancy.")]
    pub life_expectancy: f64,

    #[serde(rename="Freedom to make life choices", alias="Freedom")]
    pub freedom: f64,

    #[serde(rename="Generosity")]
    pub generosity: f64,

    #[serde(rename="Perceptions of corruption", alias="Trust (Government Corruption)", alias="Trust..Government.Corruption.")]
    pub corruption: f64,
}

/*
1.What it does
   This function makes sure the countries names are written in a standard way.
   This can fix the inconsistency of the country names throughout different year's datasets.
2.Inputs and outputs
   Input: name, which is a country name as &tr
   Output: A fixed version of the country name as String
3.Core logic and key components:
   First converts the name to lowercase and trims spaces.
   Use a match to check if the name is a form that is known.
   Return a standard name if it matches or give back to the original name if it doesn't match. 
*/
pub fn canonical_country_name(name: &str)->String {
    match name.to_lowercase().trim() { 
        "hong kong s.a.r., china"|"hong kong s.a.r. of china" | "hong kong china"=>"hong kong".to_string(),

        "taiwan province of china"=>"taiwan".to_string(),

        "north macedonia"|"macedonia"=>"macedonia".to_string(),

        "trinidad & tobago"|"trinidad and tobago"=>"trinidad and tobago".to_string(),

        "somaliland region"|"somaliland"=>"somaliland".to_string(), 

        "united states of america"=>"united states".to_string(),

        other=>other.to_string(),
    }
}

/*
1.What it does
   This function helps to read CSV file from 2015 to 2023(except 2022 because 2022 misses too many data),
   cleans country names, and organizes the data by year.
2.Inputs and outputs
   Input: folder_path, which is a string path to the folder that holds the CSV files
   Output: A HashMap where each yea maps to a list of CountryData
3.Core logic and key components
   This function iterates through each year from 2015 to 2023(skip 2022).
   It builds the file path for that year's CSV and tries to read the file using CSV reader.
   If successfully read, the function turns each row into a CountryData object, standardizes the country names, and saves the list of records under that year in the map.
   If there are some errors, the function will print the warning and skip it.
*/
pub fn load_data(folder_path: &str)->HashMap<u32, Vec<CountryData>>{
    let mut all_data=HashMap::new();

    for year in 2015..=2023{
        if year==2022{
            continue;// skip year 2022
        }

        let file_path=format!("{}/World Happiness Report {}.csv", folder_path, year);
        let path=Path::new(&file_path);

        if !path.exists(){
            println!("Warning: File missing for year {} at path: {:?}", year, file_path);
            continue;
        }

        match Reader::from_path(path){
            Ok(mut rdr)=>{
                //deserialize each row into a CountryData struct and skip rows that fail
                let mut records: Vec<CountryData>=rdr.deserialize().filter_map(Result::ok).collect();
                
                //fix inconsistent country naming across datasets
                for record in &mut records{
                    record.country=canonical_country_name(&record.country);
                }

                all_data.insert(year, records);//store cleaned records for this year
            }
            Err(e)=>{
                println!("Error reading file {}: {}", file_path, e);
            }
        }
    }
    all_data
}
