pub enum Instruction {
    Avance(i32),
    Tourne,
}
fn execute_logo(programme:&Vec<Instruction>)-> (i32,i32){
    let mut t =(0,0);
    // 0 droit , 1 haut , 2 gauche , 3 bas 
    let mut d=0;
    for inst in programme {
        match inst {
            Instruction::Avance(x)=>{
                match d {
                    0=>t.0+=x,
                    1=>t.1+=x,
                    2=>t.0-=x,
                    _=>t.1-=x 
                }
            },
            Instruction::Tourne=> d=(d+1)%4,
        }
    }
    t
}
#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn test_logo(){
        let programme=vec![
            Instruction::Avance(10),
            Instruction::Tourne,
            Instruction::Avance(5),
            Instruction::Tourne,
            Instruction::Avance(15),
        ];
        assert_eq!(execute_logo(&programme),(-5,5))
    }
}
