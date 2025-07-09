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