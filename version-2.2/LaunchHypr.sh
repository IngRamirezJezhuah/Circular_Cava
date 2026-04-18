#!/bin/bash
#========================================================================
# Dj al habla, hice esta segunda version de el de la musica             |
# por que me es mas facil hcaerlo con los colores segun wallpaper para  |
# aprovechar los colores por defecto que hice asi puede quedar mas      |
# guapeton si no funciona asi mejor usar el launch.sh normal            |
#========================================================================
PWD="$HOME/Documentos/Programacion/Proyectos-EWW/Circular_Cava/version-2.0/eww"
PROJECT_DIR="$(dirname "$0")"

grep "=" ~/.config/hypr/wallust/wallust-hyprland.conf | sed 's\ = rgb(\(.*\)): #\1;\g' | tr -d '\r' > "$PROYECT_DIR/eww/colors.scss"  

killall eww 2>/dev/null

if ! pidof eww > /dev/null; then
   # ↳ esta madre lo que hace es que revisa si el eww esta chambeando
   playerctld daemon & 
   # ↳ entonces si esta chambeando pues pide al reproductor que busque el anterior que ya estaba haciendo chamba de musica y lo pone para reproducir
   eww --config "$CFG_DIR" daemon &
   #↳ ahora si ya prende el cacharro a chambear
   sleep 0.5
   #↳hay que darle tiempo a que se muestre segun un profe eso es bueno como experiencia de user algo asi decia
fi

eww --config "$PWD" daemon &
sleep 1

eww --config "$PWD" open glava2
