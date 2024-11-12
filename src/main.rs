// src/main.rs

use std::io;
use std::io::Write;
use colored::*;

mod utils {
	pub mod utils;
}

mod modules {
	pub mod wif_generate;
	pub mod desafio_info;
	pub mod private_to_public_key;
	pub mod address_generate;
	pub mod puzzle_wallet;
}

mod data {
	pub mod ranges;
}

mod menu_task;

fn main() {
	loop {
		// Limpar a tela e exibir o menu
		utils::utils::clear_console();

		println!("{}","╔═══════════════════════════════╗".bold().truecolor(190, 60, 190));
		println!("{}","║                               ║".bold().truecolor(190, 60, 190));
    println!("{}", format!("║           {}            ║", "BIT RUST".blue().bold()).truecolor(190, 60, 190));
    println!("{}", format!("║             {}              ║", "v0.1".yellow().bold()).truecolor(190, 60, 190));
		println!("{}","║                               ║".bold().truecolor(190, 60, 190));
		println!("{}","╚═by CaptainJuk═════════════════╝".bold().truecolor(190, 60, 190));
		println!();
		println!("-------------------------------------------");
		println!(" 1 - CONVERT PRIVATE KEY TO WIF");
		println!(" 2 - GENERATE PUBLIC KEY");
		println!(" 3 - GENERATE BTC ADDRESS");
		println!("-------------------------------------------");
		println!(" 4 -  ");
		println!(" 5 -  ");
		println!(" 6 -  ");
		println!("-------------------------------------------");
		println!(" 7 - DESAFIO INFO");
		println!(" 8 -  ");
		println!(" 9 -  ");
		println!("-------------------------------------------");
		println!(" 0 - EXIT");
		println!("-------------------------------------------");

		// Solicitar a escolha do usuário
		print!("Escolha uma opção: ");
		io::stdout().flush().unwrap(); // Forçar o flush para garantir que a mensagem apareça
		

		let mut choice = String::new();
		io::stdin()
			.read_line(&mut choice)
			.expect("Falha ao ler a entrada");
		let choice: i32 = match choice.trim().parse() {
			Ok(num) => num,
			Err(_) => {
				println!("Opção inválida. Tente novamente.");
				continue;
			}
		};

		// Processar a escolha do usuário
		match choice {
			1 => menu_task::generate_wif(),
			2 => menu_task::generate_public_key(),
			3 => menu_task::generate_address(),
			4 => menu_task::puzzle_wallet_challenge(),
			5 => println!("Opção 5"),
			7 => modules::desafio_info::desafio_info(),
			0 => {
				println!("Saindo...");
				break;
			}
			_ => println!("Opção inválida. Tente novamente."),
		}
	}
}