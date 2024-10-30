pub struct Frac {
    n : i32 ,
    d:u32,
}
impl Frac {
    pub fn from(n:i32,d:u32)->Self{
        // constucteur 
        if d==0 {panic!("Denominateur nulle!");}
        Frac { n, d }
    }
    pub fn to(&self)->f32{
        // plus proche float de la fraction 
        self.n as f32 / self.d as f32
    }
    pub fn mult(&self,other:&Self)->Self{
        // multiplication de deux fractions
        Self::from((self.n)*(other.n),
             (self.d)*(other.d))
    }
    pub fn prod(factors:Vec<Self>)->Self {
        // produit d'un vecteur de fraction est une fraction 
        let mut res_prod =Self::from(1, 1);
        for f in &factors{
            res_prod=res_prod.mult(f);
        }
        res_prod
    }
}

#[cfg(test)]
pub mod tests{
    use super::*;
    #[test]
    #[should_panic]
    fn test_panic_from(){
        let _f = Frac::from(2, 0);
    }
    #[test]
    fn test_create_a_valid_fraction(){
        let f = Frac::from(2, 3);
        assert_eq!(f.n,2);
    }
    #[test]
    fn test_to(){
        let f = Frac::from(2, 3);
        let r = 2 as f32 / 3 as f32;
        assert_eq!(f.to(),r);
    }
    #[test]
    fn test_mult(){
        let f1 = Frac::from(2, 3);
        let f2 = Frac::from(4, 7);
        let r =f1.mult(&f2);
        assert_eq!(8,r.n);
        assert_eq!(21,r.d);
    }
    #[test]
    fn test_prod(){
        let v = vec![
            Frac::from(2, 3),
            Frac::from(4, 9),
            Frac::from(1, 5),
        ];
        let r=Frac::prod(v);
        assert_eq!(r.n,8);
        assert_eq!(r.d,135);
    }

}