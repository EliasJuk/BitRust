// src/utils/utils.rs

use std::io::{self, Write};

/// Função para limpar a tela do terminal
pub fn clear_console() {
	print!("\x1b[2J\x1b[1;1H");
	io::stdout().flush().unwrap();
}

/// Função para pausar a execução até o usuário pressionar Enter
pub fn pause_until_enter() {
	print!("\nPressione Enter para voltar ao menu... ");
	io::stdout().flush().expect("Falha ao exibir a mensagem");

	let mut _dummy = String::new();
	io::stdin().read_line(&mut _dummy).expect("Falha ao esperar entrada");
}