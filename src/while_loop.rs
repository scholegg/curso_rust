use std::io;

fn convert_to_int(data_input: &String) -> i32 {
  let x = data_input.trim().parse::<i32>().unwrap();
  x
}

pub fn while_loop() {
	let mut soma = 0;
  let mut valor_entrada = String::new();
  io::stdin().read_line(&mut valor_entrada).expect("Erro ao ler valor_entrada");

  // variavel para receber a conversao do valor_entrada para i32,
  // usando a func convert_to_int
  let mut valor_int = convert_to_int(&valor_entrada);

  // o while ira receber todos os digitos de entrada
  // e calcular a divisao por 10 ate a sobra ser 0
  // e somar cada numeral da entrada
  while valor_int != 0 {
    let mut r = valor_int % 10;
    soma = soma + r;
    valor_int = valor_int / 10;
  }
  println!("O valor da soma dos digitos eh {}", soma);
}