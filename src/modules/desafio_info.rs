// src/modules/desafio_info.rs

use std::io;
use std::io::Write;
use crate::data::ranges;
use crate::utils::utils::calcular_combinacoes;
use crate::utils::utils::clear_console;
use crate::utils::utils::pause_until_enter;

pub fn desafio_info() {
	clear_console();

	let lista = ranges::get_lista();

	print!("Digite o número do desafio (1-160): ");
  io::stdout().flush().unwrap();

	let mut input = String::new();
	std::io::stdin()
			.read_line(&mut input)
			.expect("Falha ao ler linha");

	let input = input.trim();

	match lista.iter().find(|&d| d.0 == input) {
		Some(desafio) => {
			println!("Desafio encontrado:");
			println!("Bits: {}", desafio.0);
			println!("StartHex: {}", desafio.1);
			println!("EndHex: {}", desafio.2);
			println!("PublicKey: {}", desafio.3);
			println!("Address: {}", desafio.4);
			println!("Status: {}", desafio.5);

			let start_hash = desafio.1;
			let end_hash = desafio.2;

			match calcular_combinacoes(start_hash, end_hash) {
				Ok(combinacoes) => {
					println!("Número de combinações no range: {}", combinacoes);
				}
				Err(e) => {
					println!("Erro: {}", e);
				}
			}
		}
		
		None => {
			println!("Desafio não encontrado!");
		}
	}

	pause_until_enter();
}