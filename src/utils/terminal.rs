use rpassword::prompt_password;



pub fn clear_terminal(){
    println!("{cls}c",cls = 27 as char)
}

pub fn hold_enter(){
    prompt_password("Esperando dar ENTER para continuar");
}