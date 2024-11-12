// src/utils/utils.rs

use std::io::{self, Write};
use num_bigint::BigUint;
use num_traits::One;
use std::fs::OpenOptions;
use std::env;


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

/// Função para calcular o número de combinações entre o StartHash e o EndHash
pub fn calcular_combinacoes(start_hash: &str, end_hash: &str) -> Result<BigUint, String> {
	// Convertendo as strings hexadecimais para BigInt
	let start = match hex_to_bigint(start_hash) {
		Ok(value) => value,
		Err(e) => return Err(format!("Erro no StartHash: {}", e)),
	};

	let end = match hex_to_bigint(end_hash) {
		Ok(value) => value,
		Err(e) => return Err(format!("Erro no EndHash: {}", e)),
	};

	// Calcula a diferença entre os dois valores
	let diff = if end > start {
		end - start
	} else {
		start - end
	};

	Ok(diff + BigUint::one())
}

/// Função para converter de hexadecimal para BigInt
pub fn hex_to_bigint(hex: &str) -> Result<BigUint, String> {
	let hex = hex.trim().to_lowercase(); // Remove espaços e converte para minúsculas

	// Verifica se o valor é "null" ou está vazio
	if hex.is_empty() || hex == "null" {
		return Err("Valor inválido: 'null' ou vazio.".to_string());
	}

	// Verifica se a string contém apenas caracteres hexadecimais
	if hex.chars().all(|c| c.is_digit(16)) {
		match BigUint::parse_bytes(hex.as_bytes(), 16) {
			Some(value) => Ok(value),
			None => Err("Erro na conversão do valor hexadecimal para BigInt.".to_string()),
		}
	} else {
		Err("A string contém caracteres não hexadecimais.".to_string())
	}
}

/// Função para salvar o resultado no arquivo resultado.txt
pub fn salvar_resultado(endereco: &str, chave_privada: &str) {
	// Obtem o diretório do executável
	let exe_dir = env::current_exe()
		.expect("Falha ao obter o diretório do executável")
		.parent()  // Obtém o diretório pai
		.expect("Falha ao obter o diretório pai do executável")
		.to_path_buf();
	
	// Concatena o nome do arquivo com o diretório do executável
	let arquivo_path = exe_dir.join("resultado.txt");
	
	// Abre o arquivo (cria se não existir, apende caso já exista)
	let mut file = OpenOptions::new()
		.create(true)
		.append(true)
		.open(arquivo_path)
		.expect("Não foi possível abrir o arquivo");
	
	// Escreve no arquivo
	writeln!(file, "Endereço encontrado: {}, Chave Privada: {}", endereco, chave_privada)
		.expect("Falha ao escrever no arquivo");
}