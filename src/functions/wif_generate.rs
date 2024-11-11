// src/functions/wif_generate.rs

use hex::decode;
use sha2::{Digest, Sha256};
use base58::ToBase58;

pub fn key_to_wif(private_key_hex: &str) -> Result<String, String> {
	// Converter a chave privada hexadecimal para bytes
	let private_key_bytes = match decode(private_key_hex) {
		Ok(bytes) => bytes,
		Err(_) => return Err("Erro ao decodificar chave privada hexadecimal".to_string()),
	};

	// Prefixo 0x80 para Bitcoin mainnet
	let mut extended_key = vec![0x80];
	extended_key.extend(private_key_bytes);

	// Adicionar o sufixo 0x01
	extended_key.push(0x01);

	// Faz o hash duplo SHA-256 para gerar o checksum
	let checksum = {
		let hash1 = Sha256::digest(&extended_key);
		let hash2 = Sha256::digest(&hash1);
		hash2[0..4].to_vec()
	};

	// Adicionar o checksum ao final da chave estendida
	extended_key.extend(checksum);

	// Converter o resultado final para Base58
	let wif = extended_key.to_base58();
	Ok(wif)
}