// src/modules/puzzle_wallet.rs

use crate::data::ranges;
use crate::modules::address_generate::address_generate;
use crate::modules::private_to_public_key;
use crate::utils::utils::{clear_console, pause_until_enter, calcular_combinacoes, salvar_resultado};
use colored::*;
use num_bigint::BigUint;
use rand::RngCore;
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
				// TODO: CRIAR FUNÇÃO PARA SEQUENCIAL
				println!("ESSA OPÇÃO AINDA NÃO ESTA DISPONIVEL.");
			} else if escolha == "A" || escolha == "a" || escolha == "2" {
				let mut target_address = desafio.4.to_string();
				gerar_combinacoes_random(desafio.1, desafio.2, &mut target_address);
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

pub fn gerar_combinacoes_random(start_hex: &str, end_hex: &str, target_address: &str) {
	// Converte StartHex e EndHex para BigUint
	let start_value = BigUint::parse_bytes(start_hex.as_bytes(), 16).expect("StartHex inválido");
	let end_value = BigUint::parse_bytes(end_hex.as_bytes(), 16).expect("EndHex inválido");

	// Calcula o intervalo entre start e end
	let range = &end_value - &start_value;

	let mut rng = rand::thread_rng(); // Gerador mutável

	// Gera números aleatórios até encontrar o endereço correspondente
	let mut random_hex = String::new();

	while random_hex != target_address {
		// Gera um número aleatório dentro do intervalo utilizando um vetor de bytes
		let byte_size = range.to_bytes_be().len(); // Tamanho em bytes do intervalo
		let mut random_bytes = vec![0u8; byte_size]; // Vetor de bytes aleatórios
		rng.fill_bytes(&mut random_bytes); // Preencher o vetor com valores aleatórios

		// Converte os bytes gerados para um BigUint
		let random_value = BigUint::from_bytes_be(&random_bytes) % &range; // Garantir que o valor caiba no intervalo

		// Adiciona o valor gerado ao start_value
		let random_bigint = &start_value + random_value;

		// Formata o número gerado como hexadecimal
		random_hex = format!("{:X}", random_bigint);
		let complete_random_hex = format!("{:0>64}", random_hex);

		// Tenta gerar a chave pública e o endereço
		match private_to_public_key::private_to_public_key(&complete_random_hex) {
			Ok(public_key) => {
				let public_key_hex = hex::encode(public_key);

				match address_generate(&public_key_hex) {
					Ok(address) => {
						if address == target_address {
							print!(
								"\rEndereço encontrado: {}, Chave Privada: {}\n",
								address.blue(),
								random_hex.yellow().bold()
							);
							salvar_resultado(&address, &random_hex);
							break;
						}
					}
					Err(err) => println!("Erro ao gerar o endereço: {}", err),
				}
			}
			Err(err) => println!("Erro: {}", err),
		}

		// Continua gerando e exibindo o progresso
		print!("\rChave Privada: {}", random_hex);
		std::io::stdout()
			.flush()
			.expect("Falha ao escrever na saída");
	}
}