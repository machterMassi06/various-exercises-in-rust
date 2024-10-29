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
} 