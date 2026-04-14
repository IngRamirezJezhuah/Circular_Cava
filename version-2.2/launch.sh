#!/bin/bash

echo "Compilando motor de audio..."

UPWD="$HOME/Documentos/Programacion/Proyectos-EWW/Circular_Cava/version-2.2/eww"

cargo build --release

pkill eww
sleep 0.5

echo "Lanzando Circular cava..."
eww --config "$PWD" daemon &

eww --config "$PWD" open glava2

#este comando es para ejecutar la copilacion de rust 
#cargo build --release 
