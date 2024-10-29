
pub fn armstrung(x:u32)->bool{
    let mut y=x;
    let mut v=Vec::new();
    while y!=0 {
        v.push(y%10);
        y=y/10;
    }
    let mut n=0;let p=v.len() as u32;
    for e in v {
        n+=e.pow(p);
    }
    n==x
}


#[cfg(test)]
pub mod tests{
    use super::*;
    #[test]
    fn test_not_armstrong_number(){
        let x: u32 =10;
        let b = armstrung(x);
        assert!(!b);
    }
    #[test]
    fn test_not_armstrong_number1(){
        let x: u32 =154;
        let b = armstrung(x);
        assert!(!b);
    }
    #[test]
    fn test_armstrong_number(){
        let x: u32 =153;
        let b = armstrung(x);
        assert!(b);
    }
    #[test]
    fn test_armstrong_number1(){
        let x: u32 =9;
        let b = armstrung(x);
        assert!(b);

    }
}