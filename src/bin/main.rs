use test_rust::password_gen::Password;
fn main () {
    let password=Password::new_alea(12);
    println!("The password is :{}",password.get_val());
    match password.is_strong(){
        Ok(()) => println!("The password is secure :)"),
        Err(err)=>{
        println!("The password is weak !");
        for e in err{
            println!("{}",e);
        }
        },
    }
}