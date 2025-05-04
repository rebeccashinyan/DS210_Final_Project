//main.rs
//This module runs the whole process and tracks how countries move between clusters over the years.


mod data_loader;
mod preprocessing;
mod clustering;

use crate::data_loader::load_data;
use crate::preprocessing::{filter_valid_data, normalize_features};
use crate::clustering::{kmeans, calculate_inertia};
use std::collections::HashMap;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

/* 
1. What it does:
   This function generates a consistent random seed based on the year and k.
2. Inputs and outputs:
   Input: year-year of the data, k-number of clusters
   Output: A seed value for randomness as u64
3. Core logic and key components:
   The function combines year and k into tuple.
   It hashes the tuple using a default hasher.
   Then, it returns the result number as the seed.
*/
fn compute_seed(year: u32, k: usize)->u64{
    let mut hasher=DefaultHasher::new();
    (year, k).hash(&mut hasher);
    hasher.finish()
}


fn main(){
    let folder_path="final_project_datasets";
    let raw_data=load_data(folder_path);

    let mut clustered: HashMap<u32, Vec<(String, usize)>>=HashMap::new();
    let mut transition_counts: HashMap<(usize, usize), usize>=HashMap::new();

    //get sorted list of years (excluding 2022)
    let mut years: Vec<u32>=raw_data.keys().cloned().collect();
    years.sort();

    let mut previous_labels:Option<HashMap<String, usize>>=None;

    for year in years{
        if year==2022 {
            continue; //skip incomplete year
        }

        if let Some(countries)=raw_data.get(&year){
            //step 1: clean and normalize data
            let mut clean_data=filter_valid_data(countries);
            normalize_features(&mut clean_data);
            clean_data.sort_by(|a, b|a.country.cmp(&b.country)); // ensure stable label assignment

            //step 2: try different values of k and print inertia
            println!("Finding best k for year {}", year);
            for k in 2..=6{
                let seed = compute_seed(year, k);
                let (labels, centers)=kmeans(&clean_data, k, 10, seed);
                let inertia=calculate_inertia(&clean_data, &centers, &labels);
                println!("k = {}: Inertia = {:.2}", k, inertia);
            }

            //step 3: final clustering using chosen k
            let best_k=4;
            let seed=compute_seed(year, best_k);
            let (labels, _)=kmeans(&clean_data, best_k, 10, seed);

            //attach each country to its cluster label
            let year_clusters: Vec<(String, usize)>=clean_data
                .iter()
                .zip(&labels)
                .map(|(entry, &label)|(entry.country.clone(), label))
                .collect();

            //create a map: country name → cluster label
            let current_labels: HashMap<String, usize>= year_clusters.iter().cloned().collect();

            //step 4: track cluster changes from previous year
            if let Some(prev)=&previous_labels {
                for (country, &prev_cluster) in prev.iter(){
                    if let Some(&curr_cluster)=current_labels.get(country){
                        *transition_counts.entry((prev_cluster, curr_cluster)).or_insert(0)+=1;
                    }
                }
            }

            previous_labels=Some(current_labels);
            clustered.insert(year, year_clusters);
        }
    }

    //step 5: print transition matrix with percentages
    println!("\nCluster Transition Matrix (from -> to):");

    let mut from_totals: HashMap<usize, usize> = HashMap::new();
    for (&(from, _), &count) in &transition_counts{
        *from_totals.entry(from).or_insert(0)+=count;
    }
    
    // Sort keys for consistent output order
    let mut keys: Vec<_>=transition_counts.keys().cloned().collect();
    keys.sort();

    for (from, to) in keys{
        let count=transition_counts[&(from, to)];
        let total=from_totals[&from];
        let percent=(count as f64 / total as f64)*100.0;
        println!("{} -> {}: {} ({:.2}%)", from, to, count, percent);
    }
}
