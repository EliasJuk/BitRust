// src/utils/utils.rs

use std::io::{self, Write};

/// Função para limpar a tela do terminal
pub fn clear_console() {
	print!("\x1b[2J\x1b[1;1H");
	io::stdout().flush().unwrap();
}