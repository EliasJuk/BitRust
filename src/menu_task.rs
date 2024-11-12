// src/menu_task.rs

use crate::utils::utils;
use crate::modules::{wif_generate, private_to_public_key, address_generate};

use std::io;
use std::io::Write;
use colored::*;

pub fn generate_wif() {
	// Limpar a tela e exibir o menu
	utils::clear_console();
	println!("Example: b3d8c534b0d494ccda1b4dcc47cdc0aa701e39f89c326035f73558e590163dcb");

	// Pergunta para o usuário digitar a chave privada em formato hexadecimal
	print!("\nDigite a chave privada em formato hexadecimal: ");
	io::stdout().flush().unwrap();

	let mut private_key_hex = String::new();
	io::stdin()
		.read_line(&mut private_key_hex)
		.expect("Falha ao ler a chave privada");
	let private_key_hex = private_key_hex.trim(); // Remove espaços ou quebras de linha extras

	// Chama a função para gerar a chave WIF
	match wif_generate::key_to_wif(private_key_hex) {
		Ok(wif) => {
			println!("\nChave WIF gerada: {}", wif.yellow().bold());
		}
		Err(e) => {
			println!("\nErro: {}", e);
		}
	}

	utils::pause_until_enter();
}

pub fn generate_public_key() {
	utils::clear_console();

	// Pergunta pela chave privada 
	println!("Example: c875d4c6b3a2b9db1449be3a3a58d8feadfdf0be49792d7b9e79bde47bafccdb");
	print!("\nDigite a chave privada: ");
	io::stdout().flush().unwrap(); // Força o flush para garantir que a mensagem apareça

	let mut private_key_hex = String::new();
	io::stdin()
		.read_line(&mut private_key_hex)
		.expect("Falha ao ler a chave privada");

	let private_key_hex = private_key_hex.trim();

	// Chama a função para gerar a chave pública comprimida
	match private_to_public_key::private_to_public_key(private_key_hex) {
		Ok(public_key) => {
			// Imprimi a chave pública comprimida em formato hexadecimal
			println!("\nChave pública comprimida: {}", format!("{:?}", public_key).blue());
			println!("Chave pública comprimida (hex): {}", hex::encode(public_key).yellow().bold());
		}
		Err(e) => {
			println!("Erro: {}", e);
		}
	}

	utils::pause_until_enter();
}

pub fn generate_address() {
	utils::clear_console();
	println!("Example: 02bf1eed286583fe3f3718678083b0b9946ff6c9efe50efd3a274f326eb97c528f");

	// Perguntar pela chave pública comprimida
	print!("\nDigite a chave pública comprimida: ");
	io::stdout().flush().expect("Falha ao limpar o buffer");

	let mut compressed_public_key = String::new();
	io::stdin()
		.read_line(&mut compressed_public_key)
		.expect("Falha ao ler a chave pública comprimida");
	let compressed_public_key = compressed_public_key.trim();

	// Chamar a função para gerar o endereço Bitcoin
	match address_generate::address_generate(compressed_public_key) {
		Ok(address) => {
			println!("\nEndereço Bitcoin gerado: {}", address.yellow().bold());
		}
		Err(e) => {
			println!("Erro: {}", e);
		}
	}

	utils::pause_until_enter();
}