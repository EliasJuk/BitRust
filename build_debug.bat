@echo off
echo Compilando o projeto...
cargo build

echo.
echo Executando o programa...
target\debug\bitrust.exe

pause