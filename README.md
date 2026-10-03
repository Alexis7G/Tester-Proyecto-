# TAVOX LCH - Proyecto base (Tauri 2)

La interfaz esta en `src/index.html`. La parte nativa (Rust) esta en `src-tauri/src/main.rs`.

## Opcion facil: crear el .exe en la nube (sin instalar nada)
1. Crea una cuenta gratis en https://github.com y un repositorio nuevo.
2. Sube todo el contenido de esta carpeta (incluida la carpeta oculta `.github`).
3. Entra en la pestana **Actions** y espera a que termine "Crear instalador de TAVOX LCH"
   (tambien puedes lanzarlo con "Run workflow"). Tarda unos 10 a 15 minutos.
4. Abre esa ejecucion y descarga **TAVOX-LCH-instalador** en la seccion Artifacts.
   Dentro esta el .exe para instalar.

## Opcion manual: compilar en tu PC
### 1. Instalar requisitos (Windows)
1. **Node.js LTS**: https://nodejs.org
2. **Microsoft C++ Build Tools**: https://visualstudio.microsoft.com/visual-cpp-build-tools/
   (marca "Desarrollo de escritorio con C++")
3. **Rust**: https://rustup.rs (descarga rustup-init.exe)
4. **WebView2**: ya viene en Windows 10/11 actualizados. Si no: https://developer.microsoft.com/microsoft-edge/webview2/

Reinicia la terminal despues de instalar.

## 2. Usar el proyecto
```
npm install
npm run dev      # abre la app en modo desarrollo
npm run build    # crea el instalador .exe
```
El instalador queda en `src-tauri/target/release/bundle/nsis/`.
La primera compilacion tarda varios minutos.

## IMPORTANTE para PCs con 2 GB de RAM
Compilar Rust consume mucha memoria. Compila en otra PC mas potente
y copia solo el instalador .exe a la PC modesta. Ejecutar la app si es ligera.

## Que esta conectado
- `system_info`: lee la RAM real del PC y ajusta la RAM recomendada del juego.

## Siguientes pasos
1. Mods y modpacks reales desde la API de Modrinth (https://docs.modrinth.com)
2. Login Microsoft y lanzar Minecraft (por ejemplo con la libreria `lyceris` o comandos propios en Rust)
3. Guardar instancias, cuentas y HUD en disco
