fn position(i: usize, j: usize,n: usize) -> usize{
    assert_eq!(0 as usize <=i && i<=n-1,true);
    assert_eq!(0 as usize <=j && j<=n-1,true);
    i*n+j
}

fn check_sym(a: &Vec<f64>,n: usize) -> bool{
    assert_eq!(a.len(),n*n);
    for i in 0..n {
        for j in i+1..n{
            if a[position(i, j, n)]!=a[position(j, i, n)]{
                return false;
            }
        }
    }
    return true ;
}

struct Sym {
    n: usize,
    values: Vec<f64>,
}

impl Sym {
    pub fn id(n: usize) -> Self{
        let mut values = vec![0.;(n*(n+1))/2];
        let mut k=0;
        for i in  0..n{
            for j in 0..=i{
                if i==j{
                    values[k]=1.;
                }
                k+=1;
            } 
        }
        Self { n, values }
    }
    fn get(&self,i: usize, j: usize)-> f64{
        assert_eq!(0 as usize <=i && i<=self.n-1,true);
        assert_eq!(0 as usize <=j && j<=self.n-1,true);
        if j<=i {
            self.values[(i*(i+1))/2 +j]
        }else {
            self.get(j, i)
        }
    }

    fn prod(&self,b:&Vec<f64>)->Vec<f64>{
        let mut r = vec![0.;self.n];
        let mut idx=0 as usize;
        for i in 0..self.n{
            for j in 0..=i{
                let value = self.values[idx];
                r[i]+=value*b[j];
                if i!=j{
                    r[j]+=value*b[i];
                }
                idx+=1;
            }
        }
        r
    }
}


#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn test_position(){
        assert_eq!(position(1, 2, 3),5);
        assert_eq!(position(2, 2, 3),8);
    }
    #[test]
    fn test_check_sym(){
        let mut a = vec![
            1., 2., 3.,
            2., 4., 5.,
            3., 5., 6.
            ];
        assert_eq!(check_sym(&a, 3),true);
        a[1]=7.;
        assert_eq!(check_sym(&a, 3),false);
    }
    #[test]
    fn test_id_sym(){
        let id = Sym::id(3);
        assert_eq!(id.values,vec![1.,0.,1.,0.,0.,1.]);
        let id = Sym::id(4);
        assert_eq!(id.values,vec![1.,0.,1.,0.,0.,1.,0.,0.,0.,1.]);
    }

    #[test]
    fn test_sym_get(){
        let id = Sym::id(3);
        assert_eq!(id.get(0,0),1.0);
        assert_eq!(id.get(0,1),0.0);
        assert_eq!(id.get(0,2),0.0);
        assert_eq!(id.get(1,0),0.0);
        assert_eq!(id.get(1,1),1.0);
        assert_eq!(id.get(1,2),0.0);
        assert_eq!(id.get(2,0),0.0);
        assert_eq!(id.get(2,1),0.0);
        assert_eq!(id.get(2,2),1.0);
    }
    #[test]
    fn test_sym_prod(){
        let id = Sym::id(3);
        let b = vec![2.,3.,4.];
        let r=id.prod(&b);
        assert_eq!(r,vec![2.,3.,4.]);
    }
}

