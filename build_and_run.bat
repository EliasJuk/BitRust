@echo off
echo Compilando o projeto em modo release...
cargo build --release

echo.
echo Copiando binário para a pasta dist...

REM Cria a pasta dist se não existir
if not exist dist (
  mkdir dist
)

REM Copia o binário para dist
copy /Y target\release\BitRust.exe dist\BitRust.exe

echo.
echo Executando o programa...
dist\BitRust.exe

pause
