// src/main.rs

use std::io;
use std::io::Write;
use colored::*;

mod utils {
	pub mod utils;
}

fn main() {
	loop {
		// Limpar a tela e exibir o menu
		utils::utils::clear_console();

		println!("{}","╔═══════════════════════════════╗".bold().truecolor(190, 60, 190));
		println!("{}","║                               ║".bold().truecolor(190, 60, 190));
    println!("{}", format!("║           {}            ║", "BIT RUST".blue().bold()).truecolor(190, 60, 190));
    println!("{}", format!("║             {}              ║", "v0.1".yellow().bold()).truecolor(190, 60, 190));
		println!("{}","║                               ║".bold().truecolor(190, 60, 190));
		println!("{}","╚═══════════════════════════════╝".bold().truecolor(190, 60, 190));
		println!();
		println!("-------------------------------------------");
		println!(" 1 - Opção 1 ");
		println!(" 2 - Opção 2 ");
		println!(" 3 - Opção 3 ");
		println!("-------------------------------------------");
		println!(" 4 - Opção 4 ");
		println!(" 5 - Opção 5 ");
		println!(" 6 - Opção 6 ");
		println!("-------------------------------------------");
		println!(" 7 - Opção 7 ");
		println!(" 8 - Opção 8 ");
		println!(" 9 - Opção 9 ");
		println!("-------------------------------------------");
		println!(" 0 - EXIT");
		println!("-------------------------------------------");

		// Solicitar a escolha do usuário
		print!("Escolha uma opção: ");
		io::stdout().flush().unwrap();
		

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
			1 => println!("Opção 1"),
			2 => println!("Opção 2"),
			3 => println!("Opção 3"),
			4 => println!("Opção 4"),
			5 => println!("Opção 5"),
			0 => {
				println!("Saindo...");
				break;
			}
			_ => println!("Opção inválida. Tente novamente."),
		}
	}
}