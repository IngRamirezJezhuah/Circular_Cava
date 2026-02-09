#!/bin/bash
# Rutas corregidas con Mayúsculas
CONFIG_DIR="$HOME/Documentos/Programacion/proyectos_personales/Circular_Cava/eww"
BIN_PATH="$HOME/Documentos/Programacion/proyectos_personales/Circular_Cava/target/release/circular_cava"

killall eww 2>/dev/null
killall circular_cava 2>/dev/null

echo "Compilando motor de audio..."
cargo build --release

# Lanzar el daemon apuntando a la carpeta correcta
eww --config "$CONFIG_DIR" daemon &
sleep 0.8

echo "Lanzando visualizador..."
eww --config "$CONFIG_DIR" open cava2
