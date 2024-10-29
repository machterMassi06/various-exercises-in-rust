
pub struct Complex {
    re:f64,
    im:f64,
}

pub fn sum(v:&Vec<Complex>)-> Complex{
    let mut newc =Complex{re:0.0,im:0.0};
    for c in v {
        newc.re+=c.re;
        newc.im+=c.im;
    }
    newc
}

#[cfg(test)]
pub mod tests{
    use super::*;
    #[test]
    fn test_sum_vector_complex(){
        let v =vec![
            Complex{
                re:1.0,im:3.0
            },
            Complex{
                re:2.0,im:4.0
            }
        ];
        let res =sum(&v);
        assert_eq!(res.im,7.0);
        assert_eq!(res.re,3.0);

    }
}