///Objective:a program in Rust that generates a random
/// password according to custom criteria specified by the user
use rand::Rng;
use std::ops::RangeInclusive;
pub struct Password {
    size : usize , 
    val : String,  
}

impl Password {

    const CAR :&[RangeInclusive<char>]=&[
        'a'..='z',
        'A'..='Z',
        '0'..='9',
        '!'..='/',
    ];

    pub fn from(val:String)->Self{
        Password{size:val.len() ,val }
    }

    pub fn new_alea(size:usize)->Self{
        let mut rng =rand::thread_rng();
        // the password necessarily starts with a capital letter
        let first_letter = *Password::CAR[1]
            .clone()
            .collect::<Vec<_>>()
            .get(rng.gen_range(0..26)).expect("Error random");

        let mut val=String::with_capacity(size);
        val.push(first_letter);

        for _ in 1..size{
            let idx = rng.gen_range(0..=3) as usize;
            let len =Password::CAR[idx].clone().count();

            let c = *Password::CAR[idx]
            .clone()
            .collect::<Vec<_>>()
            .get(rng.gen_range(0..len)).expect("Error random");
            
            val.push(c);
        }
        Password{size,val}
    }
    pub fn is_strong(&self)-> Result<(),Vec<String>> {
        let mut errors=vec![];

        if self.size <8 {errors.push("The minimum size is 8.".to_string());}

        let has_uppercase = self.val.chars().any(|c| c.is_uppercase());
        if !has_uppercase{
            errors.push("The password must contain at least one uppercase letter.".to_string());
        }

        let has_lowercase = self.val.chars().any(|c| c.is_lowercase());
        if !has_lowercase{
            errors.push("The password must contain at least one lowercase letter.".to_string());
        }

        let has_digit = self.val.chars().any(|c| c.is_digit(10));
        if !has_digit{
            errors.push("Password must contain at least one digit.".to_string());
        }

        let has_car_spc=self.val.chars().any(|c| Password::CAR[3].contains(&c));
        if !has_car_spc{
            errors.push("Password must contain at least one caractere special.".to_string());
        }

        if errors.is_empty(){
            Ok(())
        }else {
            Err(errors)
        }

    }
    
    pub fn get_val(&self)->&String{
        &self.val
    }
}
#[cfg(test)]
pub mod tests{
    use super::*;
     
    #[test]
    fn test_from(){
        let value = "ABcsdi21".to_string();
        let p = Password::from(value.clone());
        assert_eq!(p.val,value);
        assert_eq!(p.size,8);
    }
    #[test]
    fn test_weak_password1(){
        let value = "ABcsdi2121".to_string();
        let p = Password::from(value.clone());
        let res=p.is_strong();
        assert_eq!(res,Err(vec![
            "Password must contain at least one caractere special.".to_string(),
        ]));
    }
    #[test]
    fn test_weak_password2(){
        let value = "ABCDEFG".to_string();
        let p = Password::from(value.clone());
        let res=p.is_strong();
        assert_eq!(res,Err(vec![
            "The minimum size is 8.".to_string(),
            "The password must contain at least one lowercase letter.".to_string(),
            "Password must contain at least one digit.".to_string(),
            "Password must contain at least one caractere special.".to_string(),
        ]));
    }

    #[test]
    fn test_strong_password(){
        let value = "Ae56nP+%a6U".to_string();
        let p = Password::from(value.clone());
        let res=p.is_strong();
        assert_eq!(res,Ok(()));
    }
}
