// src/modules/puzzle_wallet.rs

use crate::data::ranges;
use crate::utils::utils::{clear_console, pause_until_enter, calcular_combinacoes};
use std::io::{self, Write};

pub fn puzzle_wallet() {
	clear_console();

	print!("Digite o número do desafio (1-160): ");
	io::stdout().flush().expect("Falha ao exibir a mensagem");

	let mut input = String::new();
	std::io::stdin()
		.read_line(&mut input)
		.expect("Falha ao ler linha");

	let input = input.trim();

	let lista = ranges::get_lista();

	match lista.iter().find(|&desafio| desafio.0 == input) {
		Some(desafio) => {
			println!("\nDesafio encontrado:");
			println!("Bits: {}", desafio.0);
			println!("StartHex: {}", desafio.1);
			println!("EndHex: {}", desafio.2);
			println!("PublicKey: {}", desafio.3);
			println!("Address: {}", desafio.4);
			println!("Status: {}", desafio.5);

			// Calcular o número de combinações no intervalo
			match calcular_combinacoes(desafio.1, desafio.2) {
				Ok(combinacoes) => {
					println!("Número de combinações no range: {}", combinacoes);
				}
				Err(e) => {
					println!("Erro: {}", e);
				}
			}

			// Pergunta ao usuário se ele quer a combinação sequencial ou aleatória
			print!("\nVocê quer gerar as combinações de forma (S)equencial ou (A)leatória? ");
			io::stdout().flush().expect("Falha ao exibir a mensagem");

			let mut escolha = String::new();
			std::io::stdin()
				.read_line(&mut escolha)
				.expect("Falha ao ler linha");

			let escolha = escolha.trim().to_uppercase();

			// Dependendo da escolha do usuário, chama o método correspondente
			if escolha == "S" || escolha == "s" || escolha == "1" {
				// TODO: CRIAR FUNÇÃO PARA SEGUENCIAL
			} else if escolha == "A" || escolha == "a" || escolha == "2" {
				// TODO: CRIAR FUNÇÃO PARA RANDOMICo
			} else {
				println!("Opção inválida. Escolha 'S' para sequencial ou 'A' para aleatória.");
			}
		}

		None => {
			println!("Desafio não encontrado!");
		}
	}

	pause_until_enter();
}