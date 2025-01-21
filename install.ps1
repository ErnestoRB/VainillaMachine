# Comprobar si el script está ejecutándose como administrador
$runAsAdmin = (New-Object Security.Principal.WindowsPrincipal([Security.Principal.WindowsIdentity]::GetCurrent())).IsInRole([Security.Principal.WindowsBuiltInRole]::Administrator)

if (-not $runAsAdmin) {
    Write-Host "Este script necesita ser ejecutado como administrador. Por favor, reinicia PowerShell como administrador."
    Exit
}

# Ruta donde se copiará el binario
$binPath = 'C:\Program Files\VainillaMachine\bin'

# Ruta del binario que quieres instalar
$binaryFile = ".\target\release\vainilla-machine.exe"

# Verificar si la ruta de destino existe, si no, crearla
if (-not (Test-Path $binPath)) {
    Write-Host "Creando el directorio: $binPath"
    New-Item -Path $binPath -ItemType Directory
}

# Copiar el binario a la ruta del sistema
Write-Host "Copiando el binario a $binPath"
Copy-Item $binaryFile -Destination $binPath

# Obtener el valor actual de la variable PATH a nivel de sistema
$currentPath = [System.Environment]::GetEnvironmentVariable("PATH", [System.EnvironmentVariableTarget]::Machine)

# Verificar si la ruta ya está en el PATH
if ($currentPath -notlike "*$binPath*") {
    # Agregar la nueva ruta al PATH global
    $newPathValue = "$currentPath;$binPath"
    [System.Environment]::SetEnvironmentVariable("PATH", $newPathValue, [System.EnvironmentVariableTarget]::Machine)
    Write-Host "Ruta $binPath agregada al PATH global."
} else {
    Write-Host "La ruta ya está en el PATH global."
}

# Mostrar mensaje final
Write-Host "Instalación y configuración completadas. Ahora podrás ejecutar vainilla-machine.exe desde cualquier consola."
