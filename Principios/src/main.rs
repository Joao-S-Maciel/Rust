use std::io; //biblioteca de leitura e escrita

fn convert_to_int(data_input: &String) -> i32{
    let x = data_input.trim().parse::<i32>().unwrap(); //.trim = corta a string ao achar o primeiro espaço 
    x                                                       //.parse = converte o tipo, neste caso um inteiro de 32 bits
}                                                           //.unwrap = se retornar nulo mostra como erro
fn main() {
   let mut number1 = String::new(); //declara uma variavel mutavel do tipo String
   io::stdin().read_line(&mut number1).expect("Erro ao ler number1"); //lê a entrada do usuario, se der erro apresenta a mensagem
   let mut number2 = String::new();
   io::stdin().read_line(&mut number2).expect("Erro ao ler number2");

    if convert_to_int(&number1) > convert_to_int(&number2){ //chama as funções no if e ja faz a comparação
        println!("O numero {} eh maior que {}", number1, number2);
    }else{
        println!("O numero {} eh menor ou igual que {}", number1, number2);
    }
}
