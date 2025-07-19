use crate::traits::{digitando::Digitando, falar::Falar, gritar::Gritar};

pub struct Pessoa{
     name: String,
     age: u8
}

impl Pessoa{
    pub fn new(name: String, age: u8) -> Pessoa{
        Pessoa { name, age }
    }

    pub fn get_name(&self) -> &String{
        &self.name
    }

    pub fn get_age(&self) -> u8{
        self.age
    }

    pub fn set_name(&mut self, name: String){
        self.name = name;
    }

    pub fn set_age(&mut self, age: u8){
        self.age = age;
    }
    pub fn to_string(&self) -> String {
        format!("{} {}", self.get_name(), self.get_age())
    }

}

impl Falar for Pessoa {
    fn falar(&self) {
        println!("{} está falando!", self.get_name());
    }
}

impl Gritar for Pessoa {
    fn gritar(&self) {
        println!("{} está gritando!", self.get_name());
    }
    
}

impl Digitando for Pessoa {
    fn digitar(&self, texto: &str) {
        println!("{} está digitando: {}", self.get_name(), texto);
    }
}