// src/routes.rs

use crate::utils::utils;
use crate::modules::wif_generate;

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