
pub struct Carro{
    model: String,
    age: u16,
}

impl Carro{

    pub fn new(model: String, age: u16) -> Carro{
        Carro { model, age }
    }

    pub fn set_model(&mut self, name: String){
        self.model = name;
    }

    pub fn get_model(&self) -> &String{
        &self.model
    }

    pub fn get_age(&self) -> u16{
        self.age
    }

    pub fn to_string(&self) -> String{
        format!("{} {}",self.get_model(),self.age)
    }
}