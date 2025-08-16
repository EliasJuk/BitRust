#!/bin/bash

echo "🔧 Compilando o projeto em modo release..."
cargo build --release

echo ""
echo "📁 Preparando pasta dist..."
mkdir -p dist

echo ""
echo "📦 Movendo binário para dist..."
cp target/release/bitrust dist/bitrust

echo ""
echo "🚀 Executando o programa..."
./dist/bitrust

#chmod +x build-and-run.sh
#./build-and-run.sh