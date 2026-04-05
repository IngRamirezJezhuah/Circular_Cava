#!/bin/bash

pkill eww
sleep 1

PWD="$HOME/Documentos/Programacion/Proyectos-EWW/Circular_Cava/version-2.0/eww"

eww --config "$PWD" daemon &
sleeṕ 1

eww --config "$PWD" open glava2
