#!/bin/bash

# Rutas
CONFIG_DIR="$HOME/Documentos/Programacion/proyectos_personales/circular_cava/eww"
BIN_PATH="$HOME/Documentos/Programacion/proyectos_personales/circular_cava/target/release/circular_cava"

# 1. Matar eww y el proceso de rust si ya están corriendo
killall eww 2>/dev/null
killall circular_cava 2>/dev/null

# 2. Compilar el proyecto por si hubo cambios (opcional, pero recomendado)
echo "Compilando motor de audio..."
cargo build --release

# 3. Lanzar el daemon de eww con la ruta correcta
eww --config "$CONFIG_DIR" daemon &

# Esperar un momento a que el daemon despierte
sleep 0.5

# 4. Abrir la ventana
echo "Lanzando visualizador..."
eww --config "$CONFIG_DIR" open cava2
