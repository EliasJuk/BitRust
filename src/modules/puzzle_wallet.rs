// src/modules/puzzle_wallet.rs

use crate::data::ranges;
use crate::modules::address_generate::address_generate;
use crate::modules::private_to_public_key;
use crate::utils::utils::{clear_console, pause_until_enter, calcular_combinacoes, salvar_resultado};
use colored::*;
use num_bigint::BigUint;
use rand::RngCore;
use std::io::{self, Write};
use std::thread;
use std::sync::{Arc, Mutex};


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

			// Exibe o status com cor
			let status_text = match desafio.5.to_lowercase().as_str() { 
				"solved" => "Solved".green().bold(),
				"unsolved" => "Unsolved".red().bold(),
				_ => desafio.5.normal(),
			};
			println!("Status: {}", status_text);

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
			print!("\nVocê quer gerar as combinações de forma (S)equencial - (A)leatória ou (T)hread? ");
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
			} else if escolha == "T" || escolha == "t" || escolha == "3" {
				print!("Quantas threads você gostaria de usar? ");
				io::stdout().flush().expect("Falha ao exibir a mensagem");

				let mut threads_input = String::new();
				std::io::stdin().read_line(&mut threads_input).expect("Falha ao ler linha");
				let threads_input = threads_input.trim();

				// Tenta converter a entrada para um número
				let num_threads: usize = match threads_input.parse() {
					Ok(num) => num,
					Err(_) => {
						println!("Por favor, insira um número válido de threads.");
						return;
					}
				};

				let mut target_address = desafio.4.to_string();
				gerar_combinacoes_random_threads(desafio.1, desafio.2, &mut target_address, num_threads);
			} else {
				println!("Opção inválida. Escolha 'S' para sequencial, 'A' para aleatória ou 'T' para threds.");
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

	let mut rng = rand::thread_rng();

	// Gera números aleatórios até encontrar o endereço correspondente
	let mut random_hex = String::new();

	while random_hex != target_address {
		// Gera um número aleatório dentro do intervalo utilizando um vetor de bytes
		let byte_size = range.to_bytes_be().len();
		let mut random_bytes = vec![0u8; byte_size];
		rng.fill_bytes(&mut random_bytes);

		// Converte os bytes gerados para um BigUint
		let random_value = BigUint::from_bytes_be(&random_bytes) % &range;

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


/// Função para gerar combinações aleatórias em múltiplas threads
pub fn gerar_combinacoes_random_threads(start_hex: &str, end_hex: &str, target_address: &str, num_threads: usize) {
	// Converte StartHex e EndHex para BigUint
	let start_value = BigUint::parse_bytes(start_hex.as_bytes(), 16).expect("StartHex inválido");
	let end_value = BigUint::parse_bytes(end_hex.as_bytes(), 16).expect("EndHex inválido");

	// Calcula o intervalo entre start e end
	let range = &end_value - &start_value;

	// Mutex para garantir acesso exclusivo ao resultado encontrado
	let found_address = Arc::new(Mutex::new(None));
	
	// Controle de interrupção para todas as threads
	let interrupted = Arc::new(Mutex::new(false));

	// Número de combinações por thread
	let chunk_size = range / BigUint::from(num_threads);

	// Criar threads
	let mut threads = vec![];
	for i in 0..num_threads {
		let start_range = &start_value + &chunk_size * BigUint::from(i);
		let end_range = if i == num_threads - 1 {
				end_value.clone() // Última thread vai até o final
		} else {
				&start_value + &chunk_size * BigUint::from(i + 1)
		};

		let target_address = target_address.to_string();
		let found_address = Arc::clone(&found_address);
		let interrupted = Arc::clone(&interrupted);

		// Cria thread para gerar combinações aleatórias
		let thread = thread::spawn(move || {
			let mut rng = rand::thread_rng();
			let mut random_hex = String::new();

			while random_hex != target_address {
				// Verifica se a thread deve ser interrompida
				let is_interrupted = *interrupted.lock().unwrap();
				if is_interrupted {
					break; // Se interrompido, sai do loop
				}

				// Gera um número aleatório dentro do intervalo utilizando um vetor de bytes
				let range = &end_range - &start_range;
				let byte_size = range.to_bytes_be().len();
				let mut random_bytes = vec![0u8; byte_size];
				rng.fill_bytes(&mut random_bytes);

				// Converte os bytes gerados para um BigUint
				let random_value = BigUint::from_bytes_be(&random_bytes) % &range;

				// Adiciona o valor gerado ao start_range
				let random_bigint = &start_range + random_value;

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
										let mut found = found_address.lock().unwrap();
										if found.is_none() {
											// Usando referências ao invés de mover os valores
											*found = Some((address.clone(), random_hex.clone()));
											println!(
												"\rEndereço encontrado: {}, Chave Privada: {}\n",
												address.blue(),
												random_hex.yellow().bold()
											);
											salvar_resultado(&address, &random_hex);

											// Notifica que o endereço foi encontrado
											let mut interrupted_flag = interrupted.lock().unwrap();
											*interrupted_flag = true; // Interrompe as outras threads
										}

										break; // Se encontrar, sai do loop
									}
								}
								Err(err) => println!("Erro ao gerar o endereço: {}", err),
							}
						}
					Err(err) => println!("Erro: {}", err),
				}

				// Exibe progresso de cada thread
				print!("\rThread {} - Chave Privada: {}", i + 1, random_hex);
				std::io::stdout().flush().expect("Falha ao escrever na saída");
			}
		});

		threads.push(thread);
	}

	// Espera todas as threads terminarem
	for thread in threads {
		thread.join().expect("Erro ao esperar a thread");
	}

	// Verifica se algum endereço foi encontrado
	let found = found_address.lock().unwrap();
	if found.is_none() {
			println!("\nNenhum endereço correspondente encontrado.");
	}
}