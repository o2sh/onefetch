<h3 align="center"><img src="../assets/onefetch.svg" height="130px"></h3>

<h5 align="center">Una herramienta de información de Git de línea de comandos escrita en Rust</h5>

<p align="center">
	<a href="https://crates.io/crates/onefetch"><img src="https://img.shields.io/crates/v/onefetch.svg" alt="cargo"></a>
	<a href="https://github.com/o2sh/onefetch/actions"><img src="https://github.com/o2sh/onefetch/workflows/CI/badge.svg" alt="Build Status"></a>
  <a href="https://onefetch.dev"><img src="../assets/language-badge.svg"></a>
	<a href="https://github.com/o2sh/onefetch/issues?q=is%3Aissue+is%3Aopen+label%3A%22%E2%9D%93+help+wanted%22"><img src="https://img.shields.io/github/issues/o2sh/onefetch/%E2%9D%93%20help%20wanted?color=green" alt="help wanted"></a>
	<a href="./LICENSE.md"><img src="https://img.shields.io/badge/license-MIT-blue.svg"></a>
	<img src="../assets/msrv-badge.svg">
</p>

<img src="../assets/screenshot-1.png" align="right" height="250px">

Onefetch es una herramienta de información Git de línea de comandos escrita en `Rust` que muestra información del proyecto y estadísticas de código para un repositorio Git local directamente en su terminal. La herramienta funciona sin conexión a internet.

De manera predeterminada, la información del repositorio se muestra junto con el logotipo del lenguaje dominante, pero puede configurar onefetch para usar una imagen, en terminales compatibles, una entrada de texto o nada en absoluto.

Detecta automáticamente las licencias de código abierto de los textos y proporciona al usuario información valiosa como distribución de código, cambios pendientes, número de dependencias (por administrador de paquetes), principales contribuyentes (por número de confirmaciones), tamaño en disco, fecha de creación, LOC (líneas de código), etc.

<img src="../assets/screenshot-2.png" align="right" height="250px">

Onefetch se puede configurar a través de indicadores de línea de comandos para mostrar exactamente lo que desea, de la manera que desea: puede personalizar el formato de texto/ASCII, deshabilitar líneas de información, ignorar archivos y directorios, salida en múltiples formatos (Json, Yaml), etc.

A partir de ahora, onefetch admite más de 100 lenguajes de programación diferentes;si el lenguaje de su elección no es compatible: abra una propuesta (issue) y se agregará soporte.

¡Las contribuciones son muy bienvenidas! Consulte [CONTRIBUTING](../CONTRIBUTING.md) para obtener más información.

### Más: \[[Wiki](https://github.com/o2sh/onefetch/wiki)\] \[[Instalación](https://github.com/o2sh/onefetch/wiki/Installation)\] \[[Para Empezar](https://github.com/o2sh/onefetch/wiki/getting-started)\]
