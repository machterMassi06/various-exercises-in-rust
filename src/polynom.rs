#[derive(Clone, Debug, PartialEq)]
pub struct Polynomial {  
    coefs: Vec<f64>,  
}  
impl Polynomial{

    pub fn new(coefs:&[f64])-> Self{
        Self { coefs: coefs.into(), }
    }

    pub fn deg2(a2:f64,a1:f64,a0:f64)->Self{
        Self { coefs: vec![a0,a1,a2], }
    }

    pub fn eval(&self,x:f64)->f64{
        let mut res=0.;
        let mut i =0;
        for c in &self.coefs{
            res+=c*x.powi(i);
            i+=1;
        }
        res 
    }
    /// optimized version of (Polynomial::eval)
    pub fn eval_opt(&self,x:f64)->f64{
        let mut res=0.;
        let mut i =1.;
        for c in &self.coefs{
            res+=c*i;
            i *=x;
        }
        res 
    }

}

#[cfg(test)]
mod tests{
    extern crate test;
    use super::*;
    use test::Bencher;
    
    #[bench]
    fn bench_eval(b: &mut Bencher) {
        let p = Polynomial::deg2(2.0, -1.0, 5.0);
        b.iter(|| {
            let _ = p.eval(10000000.0);
        });
    }

    #[bench]
    fn bench_eval_opt(b: &mut Bencher) {
        let p = Polynomial::deg2(2.0, -1.0, 5.0);
        b.iter(|| {
            let _ = p.eval_opt(10000000.0);
        });
    }
} 