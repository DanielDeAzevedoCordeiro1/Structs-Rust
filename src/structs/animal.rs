use crate::traits::{emitir_som::EmitirSom, latir::Latir};

pub struct Dados {
    pub id: String,
}


pub struct Animal{
    pub nome: String,
    pub especie: String,
    pub idade: u8,
    pub dados: Dados,
    pub zoo: Zoo
} 

pub struct Zoo{
    pub id: String,
    pub localizacao: Endereco,
}

pub struct Endereco{
    pub cidade: String,
    pub bairro: String,
    pub numero: String,
    pub complemento: String
}

impl Animal {
    pub fn new(nome: String, especie: String, idade: u8, dados: Dados, zoo: Zoo) -> Animal{
        Animal { nome, especie, idade, dados, zoo }
    }

    pub fn to_string(&self) -> String{
        format!("{} \n{} \n{}\n{}\n{}\n{}\n{}\n{}\n{}",self.nome,self.especie,self.idade,self.dados.id,self.zoo.id, self.zoo.localizacao.bairro,self.zoo.localizacao.cidade, self.zoo.localizacao.complemento,self.zoo.localizacao.numero)
    }   
}

impl Dados{
    pub fn new(id: String) -> Dados{
        Dados { id }
    }
}

impl Zoo{
    pub fn new(id: String, localizacao:Endereco) -> Zoo{
        Zoo { id ,localizacao }
    }
}

impl Endereco{
    pub fn new(cidade: String, bairro: String, numero: String, complemento: String) -> Endereco {
        Endereco { cidade, bairro, numero, complemento }
    }
}

impl EmitirSom for Animal {
    fn emitir_som(&self) {
        println!("O animal {} da espécie {} está emitindo um som!", self.nome, self.especie);
    }
}

impl Latir for Animal {
    fn latir(&self) {
        println!("O animal {} da espécie {} está latindo!", self.nome, self.especie);
    }
}