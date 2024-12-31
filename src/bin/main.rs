use test_rust::divers_exo::knapsack_mult;
fn main () {
    let mut d:Vec<(u32, u32, u32)>=vec![
        (51,8,15),(79,15,10),(73,13,8),(70,9,11),
        (53,8,5),(53,10,9),(51,5,7),(84,16,16),(72,14,14),(68,9,10)
    ];
    // (weight, volume)
    let b =(60u32,53u32);
    // greedy algo , sort with value desc 
    let s1 = knapsack_mult(&mut d.clone(), &mut b.clone(), 0 as usize);
    // greedy algo , sort with weight asc
    let s2 = knapsack_mult(&mut d.clone(), &mut b.clone(), 1 as usize);
    // greedy algo , sort with volume desc
    let s3 = knapsack_mult(&mut d.clone(), &mut b.clone(), 2 as usize);
    // greedy algo , sort with value/weight desc
    let s4 = knapsack_mult(&mut d.clone(), &mut b.clone(), 3 as usize);
    // greedy algo , sort with value/volume desc
    let s5 = knapsack_mult(&mut d.clone(), &mut b.clone(), 4 as usize);
    println!("[Sort with: Value of sac] : [Value Desc:{s1}],[Weight Asc :{s2}],[Volume Asc :{s3}],[(Value/Weight) Desc :{s4}],[(Value/Volume) Asc :{s5}]");
}