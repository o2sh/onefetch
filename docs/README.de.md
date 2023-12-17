<h3 align="center"><img src="../assets/onefetch.svg" height="130px"></h3>

<h5 align="center">Kommandozeilen-Git-Informationstool geschrieben in Rust</h5>

<p align="center">
	<a href="https://crates.io/crates/onefetch"><img src="https://img.shields.io/crates/v/onefetch.svg" alt="cargo"></a>
	<a href="https://github.com/o2sh/onefetch/actions"><img src="https://github.com/o2sh/onefetch/workflows/CI/badge.svg" alt="Build Status"></a>
  <a href="https://onefetch.dev"><img src="assets/language-badge.svg"></a>
  <a href="https://github.com/o2sh/onefetch/issues?q=is%3Aissue+ist%3Aoffen+Label%3A%22Hilfe+gewünscht%22"><img src="https://img.shields.io/github/issues/o2sh/onefetch/help%20wanted?color=green" alt="Hilfe gesucht"></a>
	<a href="./LICENSE.md"><img src="https://img.shields.io/badge/license-MIT-blue.svg"></a>
	<img src="../assets/msrv-badge.svg">
</p>

<img src="../assets/screenshot-1.png" align="right" height="250px">

Onefetch ist ein in `Rust` geschriebenes Kommandozeilen-Git-Informationstool, das Projektinformationen und Codestatistiken für ein lokales Git-Repository direkt in Ihrem Terminal anzeigt. Das Tool ist komplett offline - es ist kein Netzwerkzugang erforderlich.

Standardmäßig werden die Repository-Informationen zusammen mit dem Logo der vorherrschenden Sprache angezeigt, aber du kannst onefetch auch so konfigurieren, dass es stattdessen ein Bild - auf unterstützten Terminals -, eine Texteingabe oder gar nichts anzeigt.

Es erkennt automatisch Open-Source-Lizenzen aus Texten und versorgt den Benutzer mit wertvollen Informationen wie Code-Verteilung, ausstehende Änderungen, Anzahl der Abhängigkeiten (nach Paketmanager), Top-Beitragende (nach Anzahl der Commits), Größe auf der Festplatte, Erstellungsdatum, LOC (Lines of Code), etc.

<img src="../assets/screenshot-2.png" align="right" height="250px">

Onefetch kann über Befehlszeilen-Flags so konfiguriert werden, dass es genau das anzeigt, was du möchtest, und zwar so, wie du es willst: du kannst die ASCII-/Text-Formatierung anpassen, Infozeilen deaktivieren, Dateien und Verzeichnisse ignorieren, aus verschiedenen Ausgabeformaten (Json, Yaml) wählen usw.

Ab sofort unterstützt onefetch mehr als [100 verschiedene Programmiersprachen](https://onefetch.dev); wenn die Sprache Ihrer Wahl nicht unterstützt wird: Erstelle ein neues Issue und die Übersetzung wird hinzugefügt.

Beiträge sind sehr willkommen! Siehe [CONTRIBUTING](../CONTRIBUTING.md) für weitere Informationen.

### Mehr: \[[Wiki](https://github.com/o2sh/onefetch/wiki)\] \[[Installation](https://github.com/o2sh/onefetch/wiki/Installation)\] \[[Erste Schritte](https://github.com/o2sh/onefetch/wiki/getting-started)\]
