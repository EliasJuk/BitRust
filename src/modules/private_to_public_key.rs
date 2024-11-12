// src/modules/privateToPublicKey.rs

use secp256k1::{Secp256k1, SecretKey, PublicKey};
use hex::decode;

/// Função para converter chave privada em chave pública comprimida
pub fn private_to_public_key(private_key_hex: &str) -> Result<Vec<u8>, String> {
	// Converte a chave privada de hexadecimal para bytes
	let priv_key_bytes = decode(private_key_hex).map_err(|e| e.to_string())?;

	// Cria uma chave secreta a partir dos bytes para a curva elíptica secp256k1
	let secret_key = SecretKey::from_slice(&priv_key_bytes).map_err(|e| e.to_string())?;

	// Curva elíptica secp256k1
	let secp = Secp256k1::new();

	// Gera a chave pública a partir da chave privada
	let public_key = PublicKey::from_secret_key(&secp, &secret_key);

	// Gera a chave pública comprimida
	let compressed_pub_key = public_key.serialize(); // Usa o método serialize() para obter a chave comprimida

	// Verifica se o primeiro byte é 0x02 ou 0x03, que indicam uma chave pública comprimida
	if compressed_pub_key[0] == 0x02 || compressed_pub_key[0] == 0x03 {
		// Retorna a chave pública comprimida como um vetor de bytes
		Ok(compressed_pub_key.to_vec())
	} else {
		Err("Chave pública não comprimida".to_string())
	}
}