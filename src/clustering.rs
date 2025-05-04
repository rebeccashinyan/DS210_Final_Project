//clustering.rs
//This module uses a fixed k-means method to group countries and check how tight the groups are

use crate::data_loader::CountryData;
use rand::seq::SliceRandom;
use rand::rngs::StdRng;
use rand::SeedableRng;

const NUM_FEATURES: usize=5;

/*
1.What it does
   This function groups counties into k clusters using the -means algorithm.
2.Inputs and outputs
   Inputs: data-lists of CountryData, k-number of clusters,
           iterations-how many times to update clusters, seed-random seed for reproducibility
   Output: A list of cluster labels, which country belongs to which group;
           A list of cluster centers(average values of each cluster)
3.Core logic and key components
   The function randomly picks k countries as starting cluster centers.
   Then, it repeats 2 steps iterations times:
       Step1-Assign step: each country is assigned to the nearest center(based on 5 feature distance)
       Step2-Update step: Each center is moved to the average of the countries in its group.
   Lastly, the function returns the final cluster labels and the center positions.
*/
pub fn kmeans(data:&Vec<CountryData>, k:usize, iterations:usize, seed:u64,)->(Vec<usize>, Vec<CountryData>){
    let mut rng=StdRng::seed_from_u64(seed);

    //randomly pick k initial centers from the data
    let mut centers:Vec<CountryData>=data.choose_multiple(&mut rng, k).cloned().collect();

    let mut labels=vec![0; data.len()];

    for _ in 0..iterations{
        //step 1: Assign each point to the closest center
        for (i, point) in data.iter().enumerate(){
            labels[i]=(0..k)
                .min_by(|&a, &b|{
                    let da=distance(point, &centers[a]);
                    let db=distance(point, &centers[b]);
                    da.partial_cmp(&db).unwrap()
                })
                .unwrap();
        }

        //step 2: Recompute centers by averaging all points in each cluster
        let mut sums=vec![vec![0.0; NUM_FEATURES]; k];
        let mut counts=vec![0; k];

        for (point, &label) in data.iter().zip(&labels){
            //sum feature values by cluster
            sums[label][0]+=point.social_support;
            sums[label][1]+=point.life_expectancy;
            sums[label][2]+=point.freedom;
            sums[label][3]+=point.generosity;
            sums[label][4]+=point.corruption;
            counts[label]+=1;
        }

        for (i, center) in centers.iter_mut().enumerate(){
            if counts[i]>0{
                //compute average(new center) for each feature
                center.social_support=sums[i][0]/counts[i] as f64;
                center.life_expectancy=sums[i][1]/counts[i] as f64;
                center.freedom=sums[i][2]/counts[i] as f64;
                center.generosity=sums[i][3]/counts[i] as f64;
                center.corruption=sums[i][4]/counts[i] as f64;
            }
        }
    }
    (labels, centers)
}

/*
1.What it does
   This function calculates how different 2 countries are based on the selected 4 features
2.Inputs and outputs
   Input: 2 CountryData entries(a and b)
   Output: A f64 number representing the squared distance between them
3.Core logic and key components
   For each feature, the function subtract one country's value from the other and square it.
   The function then adds all 5 squared differences together.
*/
fn distance(a: &CountryData, b: &CountryData)->f64{
    (a.social_support-b.social_support).powi(2)
        +(a.life_expectancy-b.life_expectancy).powi(2)
        +(a.freedom-b.freedom).powi(2)
        +(a.generosity-b.generosity).powi(2)
        +(a.corruption-b.corruption).powi(2)
}

/*
1.What it does
   This function calculates how close each country is to its assigned cluster center
2.Inputs and outputs
   Input: data-list of country entries, centers-list of cluster centers, labels-which cluster each country belongs to
   Output: The total squared distance as f64
3.Core logic and key components
   For each country, the function finds its cluster center and calculates distance to that center.
   The function then adds up all the distances and returns the total.
*/
pub fn calculate_inertia(data: &Vec<CountryData>,centers: &Vec<CountryData>,labels: &Vec<usize>,)->f64{
    // Sum of squared distances between each point and its assigned cluster center
    data.iter().zip(labels.iter()).map(|(entry, &label)| distance(entry, &centers[label])).sum()
}
