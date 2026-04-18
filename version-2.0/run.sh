#!/bin/bash

# 1. Matar procesos anteriores si existen
pkill circular_cava
eww --config ./eww/ close glava2

# 2. Abrir la decoración (Eww)
eww --config ./eww/ open glava2 &

# 3. Ejecutar el motor (Rust)
cargo run --release
