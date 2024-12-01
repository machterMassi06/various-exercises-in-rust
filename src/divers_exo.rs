use std::collections::HashMap;

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

pub fn power(x:u32,n:u32)->u32{
    match n {
        0=> {1},
        _=>{
            match n%2{
                0=> {
                    let p= power(x,n/2);
                    p*p
                },
                _=>{
                    let p=power(x, (n-1)/2);
                    x*p*p
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
        assert_eq!(power(2, 5),32);
        assert_eq!(power(2, 10),1024);
    }
    #[test]
    fn test_interv_min_covering(){
        let x = vec![1.0,2.0,3.0,4.0,5.0,6.0];
        assert_eq!(minimum_interval_covering(x).len(),3);
        let x = vec![0.5,1.0,1.3];
        assert_eq!(minimum_interval_covering(x).len(),1)
    }
} 