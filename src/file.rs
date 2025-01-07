pub struct File {
    name : String ,
    size : f64,
}


impl File {
    pub fn new(name:String,size:f64)->Self{
        Self { name, size }
    }
}
pub struct Directory{
    name : String,
    entries : Vec<Box<dyn Entry>>
}

impl Directory{

    pub fn new(name:String)->Self{
        Self { name, entries:vec![],}
    }

    pub fn add(&mut self,entry :Box<dyn Entry>){
        self.entries.push(entry);
    }
}

pub trait Entry {
    fn name(&self)-> &String;

    fn size(&self)-> f64;
}

impl Entry for File {
    fn name(&self)-> &String {
        &self.name
    }
    fn size(&self)-> f64 {
        self.size
    }
}

impl Entry for Directory {
    fn name(&self)-> &String {
        &self.name
    }
    fn size(&self)-> f64 {
        let mut res=0.;
        for e in &self.entries{
            res+=e.size();
        }
        res
    }
}


#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_file_methodes(){
        let f =File::new(String::from("test.txt"),15.0);
        assert_eq!(*f.name(),"test.txt".to_string());
        assert_eq!(f.size(),15.0)
    }
    #[test]
    fn test_directory(){
        let mut d1=Directory::new(String::from("d1"));
        d1.add(Box::new(File::new(String::from("f1"),10.0)));
        d1.add(Box::new(File::new(String::from("f2"),20.0)));

        let mut d2=Directory::new(String::from("d2"));
        d2.add(Box::new(File::new(String::from("f3"),30.0)));
        d2.add(Box::new(File::new(String::from("f4"),40.0)));

        d1.add(Box::new(d2));

        assert_eq!(d1.size(),100.);
    }
}

