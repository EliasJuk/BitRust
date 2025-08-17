#!/bin/bash

# Nome do pacote definido no Cargo.toml
BINARY_NAME="BitRust"

echo "🔧 Compilando o projeto em modo release..."
cargo build --release

# Verifica se o build foi bem-sucedido
if [ ! -f "target/release/$BINARY_NAME" ]; then
    echo "❌ Erro: binário '$BINARY_NAME' não encontrado!"
    exit 1
fi

echo ""
echo "📁 Preparando pasta dist..."
mkdir -p dist

echo ""
echo "📦 Movendo binário para dist..."
cp "target/release/$BINARY_NAME" dist/bitrust

echo ""
echo "🚀 Executando o programa..."
./dist/bitrust