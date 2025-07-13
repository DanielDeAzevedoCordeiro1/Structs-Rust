mod utils;
mod math;
mod structs;
mod traits;


use std::io;

use utils::terminal::{
    hold_enter,
    clear_terminal
};

use math::soma::soma;
use structs::{
    pessoa::Pessoa,
    carro::Carro,
    animal::{Endereco, Dados, Zoo, Animal}
};
use traits::emitir_som::EmitirSom;
use traits::latir::Latir;
use traits::falar::Falar;


fn main() {


    println!("Hello, world!");
    println!("Hello, world!");
    println!("Hello, world!");
    clear_terminal();
    hold_enter();
    println!("Hello, world!");

    let a = 10;
    let b = 30;

    let (x, y) = (a, b);

    println!("{}",soma((x,y)));

    let mut buffer = String::new();

    println!("Digite o nome da pessoa ...");
    io::stdin().read_line(&mut buffer).expect("Deu ruim");

    let mut pessoa: Pessoa = Pessoa::new(buffer.to_string(), 23);

    println!("{}",pessoa.to_string());

    pessoa.set_name(String::from("Fodasse"));
    println!("{}",pessoa.get_name());


    let mut carro = Carro::new(String::from("Fusca"), 2020);
    println!("Carro antes de mudar o nome -> {}",carro.to_string());

    carro.set_model(String::from("Opala 2354"));
    println!("Carro depois de mudar de modelo -> {}",carro.get_model());

    let animal = Animal::new(String::from("Leao"),
     String::from("Leonino"),
      10,
       Dados::new(String::from("1234567890")),
       Zoo::new(String::from("Zoo Sao Paulo"), Endereco::new(
            String::from("Sao Paulo"),
            String::from("Bairro Teste"),
            String::from("1234"),
            String::from("Apto 101")
        ))
   );

   animal.emitir_som();
   animal.latir();
   pessoa.falar();

    println!("{}",animal.to_string());

}
