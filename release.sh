#!/bin/bash

# Script para criar uma build de release do hubstry_iso_code.
# Este script garante que o código esteja formatado, que os testes passem,
# e então compila o binário otimizado para produção.

# Parar a execução se qualquer comando falhar
set -e

echo "🚀 Iniciando o processo de build de release..."

# 1. Verificar a formatação do código
echo " lint: Verificando a formatação do código com 'cargo fmt'..."
cargo fmt -- --check

# 2. Executar os testes
echo " test: Executando todos os testes..."
cargo test --all-features

# 3. Compilar o binário em modo de release
echo " build: Compilando o binário em modo de release (otimizado)..."
cargo build --release

# 4. Exibir mensagem de sucesso
echo ""
echo "✅ Build de release concluído com sucesso!"
echo "O binário otimizado está localizado em: target/release/hubstry_iso_code"
echo "Você pode copiá-lo para /usr/local/bin ou outro diretório em seu PATH."