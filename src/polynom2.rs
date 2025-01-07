type Degree = u32;
type Coef = f64;
type Link = Option<Box<Term>>;

pub struct Polynomial {
    coefs: Link,
}

struct Term {
    coef: (Degree, Coef),
    next: Link,
}


impl Polynomial{
    ///constructs the null polynomial
    ///
    pub fn zero()->Self{
        Polynomial{coefs:None}
    }

    ///adds a new term to a polynomial.
    /// If the term of degree `degree` exists, its coefficient is replaced by `coef`
    pub fn push(&mut self,coef:Coef,degree:Degree){
        let mut current =&mut self.coefs;
        while let Some(term)=current{
            if term.coef.0==degree{
                term.coef.1=coef;
                return;
            }
            current=&mut term.next;
        }
        *current=Some(Box::new(Term{coef:(degree,coef),next:None}));
    }

    /// evaluate a polynom 
    /// 
    pub fn eval(&self,x:f64)->f64{
        let mut res=0.0;
        let mut term =&self.coefs;
        while let Some(ref t)=term{
            res+=(t.coef.1)*x.powi(t.coef.0 as i32);
            term =&t.next;
        }
        res
    }

}

impl std::ops::Index<Degree> for Polynomial{
    type Output = Coef;
    fn index(&self, index: Degree) -> &Self::Output {
        let mut current = &self.coefs;
        while let Some(term)=current{
            if term.coef.0==index{
                return &term.coef.1;
            } 
            current=&term.next;
        }
        &0.0
    }
}

#[cfg(test)]
pub mod tests{
    use super::*;
    fn example()->Polynomial{
        let mut p = Polynomial::zero();
        assert!(p.coefs.is_none());
        p.push(-4.0,0 );
        p.push(1.0, 2);
        p
    }
    #[test]
    fn test_push(){
        let p1=example();
        let p2=Polynomial{coefs:Some(Box::new(
            Term{
                coef:(0,-4.0),
                next:Some(Box::new(
                    Term{
                        coef:(2,1.0),
                        next:None,
                    }
                )),
            }
        ))};
        let mut term1=&p1.coefs;
        let mut term2=&p2.coefs;
        while let (Some(t1),Some(t2))=(term1,term2){
            assert_eq!(t1.coef,t2.coef);
            term1=&t1.next;
            term2=&t2.next;
        }
        assert!(term1.is_none());assert!(term2.is_none());

    }

    #[test]
    fn test_eval(){
        let mut p =Polynomial::zero();
        p.push(1.0,2);
        p.push(-4.0, 0);
        assert_eq!(p.eval(8.),60.);
    }
    #[test]
    fn test_index(){
        let mut p =Polynomial::zero();
        p.push(1.0,2);
        p.push(-2.0, 0);
        assert_eq!(p[0],-2.0);
        assert_eq!(p[1],0.0);
        assert_eq!(p[2],1.0);
    }

}