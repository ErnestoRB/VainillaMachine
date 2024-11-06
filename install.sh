#!/bin/bash

# Nombre del archivo binario (ejecutable) local y ubicación de destino
CLI_NAME="vainilla-machine" # Cambia esto al nombre de tu binario
LOCAL_BIN_PATH="./target/release/$CLI_NAME" # Ruta local del ejecutable (ajústala según corresponda)
INSTALL_PATH="/usr/local/bin/$CLI_NAME"

# Verificar si el script está siendo ejecutado con privilegios de superusuario
if [[ "$EUID" -ne 0 ]]; then
  echo "Por favor, ejecuta el script con privilegios de superusuario (sudo)."
  exit 1
fi

# Verificar si el binario local existe
if [[ ! -f "$LOCAL_BIN_PATH" ]]; then
  echo "Error: No se encontró el archivo ejecutable en $LOCAL_BIN_PATH"
  exit 1
fi

# Copiar el ejecutable a /usr/local/bin
echo "Instalando $CLI_NAME en $INSTALL_PATH..."
cp "$LOCAL_BIN_PATH" "$INSTALL_PATH"

# Verificar si la copia fue exitosa
if [[ -f "$INSTALL_PATH" ]]; then
  # Hacer el archivo ejecutable
  chmod +x "$INSTALL_PATH"
  echo "$CLI_NAME se ha instalado correctamente en $INSTALL_PATH."
else
  echo "Error: No se pudo copiar el ejecutable a $INSTALL_PATH."
  exit 1
fi

# Confirmar la instalación
echo "Verificando la instalación..."
if command -v "$CLI_NAME" >/dev/null 2>&1; then
  echo "$CLI_NAME está listo para usarse."
else
  echo "Error: $CLI_NAME no se encuentra en el PATH."
fi
