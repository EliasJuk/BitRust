// src/modules/address_generate.rs

use sha2::{Sha256, Digest as ShaDigest};
use ripemd160::{Ripemd160, Digest as RipemdDigest};
use base58::ToBase58;
use hex::decode;

/// Função para gerar o endereço Bitcoin a partir de uma chave pública comprimida
pub fn address_generate(compressed_public_key: &str) -> Result<String, String> {
	// Valida o comprimento da chave pública comprimida
	if compressed_public_key.len() != 66 {
		return Err("Chave pública comprimida deve ter 66 caracteres.".to_string());
	}

	// Converte a chave pública comprimida para bytes
	let public_key_bytes = match decode(compressed_public_key) {
		Ok(bytes) => bytes,
		Err(_) => return Err("Falha ao decodificar a chave pública.".to_string()),
	};

	// Aplica SHA-256 à chave pública
	let sha256_hash = sha256(&public_key_bytes)?;

	// Aplica RIPEMD-160 ao hash SHA-256
	let ripemd160_hash = ripemd160(&sha256_hash)?;

	// Adiciona byte de versão 0x00
	let mut versioned_hash = vec![0x00];
	versioned_hash.extend_from_slice(&ripemd160_hash);

	// Calcula o checksum (SHA-256 do SHA-256)
	let checksum = sha256(&sha256(&versioned_hash)?)?;
	let checksum = &checksum[0..4]; // Pegando os primeiros 4 bytes

	// Concatena o hash versionado com o checksum
	let mut binary_address = versioned_hash;
	binary_address.extend_from_slice(checksum);

	// Codifica o endereço em Base58
	let base58_address = binary_address.to_base58();

	Ok(base58_address)
}

/// SHA256
fn sha256(data: &[u8]) -> Result<Vec<u8>, String> {
	let mut hasher = Sha256::new();
	hasher.update(data);
	Ok(hasher.finalize().to_vec())
}

/// RIPEMD160
fn ripemd160(data: &[u8]) -> Result<Vec<u8>, String> {
	let mut hasher = Ripemd160::new();
	hasher.update(data);
	Ok(hasher.finalize().to_vec())
}
