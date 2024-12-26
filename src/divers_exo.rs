use std::{cmp::min, collections::HashMap};

pub fn word_count(s:&String )->HashMap<String,usize>{
    let mut d =HashMap::new();
    for word in s.to_lowercase().split(" "){
        let word=word.to_string();
        *d.entry(word).or_insert(0)+=1;

    }
    d

}

pub fn without_duplicates(v:&Vec<i32>)->Vec<i32>{
    let mut res:Vec<i32> =Vec::new();
    for x in v {
        if !res.contains(x){
            res.push(*x);
        }
    }
    res
}

pub fn power(x:f64,n:i32)->f64{
    match n {
        0=> {1f64},
        _=>{
            match n%2{
                0=> {
                    let p= power(x,n/2);
                    p*p as f64
                },
                _=>{
                    let p=power(x, (n-1)/2);
                    x*p*p as f64
                }
            }
        }
    }

}
pub fn minimum_interval_covering(x:Vec<f64>)->Vec<(f64,f64)>{
    // Intervalles de longeur 1 
    // on suppose que x est triee par ordre croissant 
    let mut covered = vec![0;x.len()];
    let mut interv=Vec::new();
    for i in 0..x.len(){
        match covered[i]{
            0=>{
                interv.push((x[i],x[i]+1.0));
                for j in i+1..x.len(){
                    if x[j]<=x[i]+1.0 {
                        covered[j]=1;
                    }
                    else {
                        break;
                    }
                }
            },
            _=>{},
        }
    }
    interv
}
///The Levenshtein distance is a mathematical distance giving a measure of the similarity 
/// between two words. 
/// It is equal to the minimum number of letters that must be deleted/inserted/replaced to move from one 
/// word 'a' has word 'b'
/// 

pub fn dist_levenshtein(a:&String,b:&String)->i32{
    let n= a.len();
    let m = b.len();
    let mut c = vec![vec![0;m+1];n+1];
    for i in 1..=n{
        c[i][0]=i as i32;
    }
    for j in 1..=m{
        c[0][j]=j as i32;
    }
    
    for i in 1..=n{
        for j in 1..=m{
            if a.get(i-1..i)==b.get(j-1..j){
                c[i][j]=c[i-1][j-1];
            }else {
                c[i][j]=min(min(c[i-1][j-1], c[i-1][j]),c[i][j-1])+1;
            }
        }
    }
    c[n][m]

}

#[cfg(test)]
pub mod tests{
    use super::*;
     
    #[test]
    fn test_word_count(){
        let phrase = String::from("Rust is great and rust is fast");
        let hp=word_count(&phrase);
        assert_eq!(hp.get("rust"),Some(&2));
    }
    #[test]
    fn test_without_duplicates(){
        let vec =vec![1,3,2,1,2,4];
        assert_eq!(without_duplicates(&vec),vec![1,3,2,4]);
    }
    #[test]
    fn test_power(){
        assert_eq!(power(2., 5),32.);
        assert_eq!(power(2., 10),1024.);
    }
    #[test]
    fn test_interv_min_covering(){
        let x = vec![1.0,2.0,3.0,4.0,5.0,6.0];
        assert_eq!(minimum_interval_covering(x).len(),3);
        let x = vec![0.5,1.0,1.3];
        assert_eq!(minimum_interval_covering(x).len(),1)
    }

    #[test]
    fn test_levenshtein1(){
        let a = String::from("test");
        let b = String::from("test");
        assert_eq!(dist_levenshtein(&a, &b),0);
    }

    #[test]
    fn test_levenshtein2(){
        let a = String::from("test");
        let b = String::from("tast");
        assert_eq!(dist_levenshtein(&a, &b),1);
    }

    #[test]
    fn test_levenshtein3(){
        let a = String::from("japon");
        let b = String::from("savon");
        assert_eq!(dist_levenshtein(&a, &b),2);
    }
} 