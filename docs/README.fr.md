<h3 align="center"><img src="../assets/onefetch.svg" height="130px"></h3>

<h5 align="center">Un outil en ligne de commande affichant les informations Git écrit en Rust</h5>

<p align="center">
	<a href="https://crates.io/crates/onefetch"><img src="https://img.shields.io/crates/v/onefetch.svg" alt="cargo"></a>
	<a href="https://github.com/o2sh/onefetch/actions"><img src="https://github.com/o2sh/onefetch/workflows/CI/badge.svg" alt="Build Status"></a>
  <a href="https://github.com/o2sh/onefetch/wiki/language-Support"><img src="../assets/language-badge.svg"></a>
	<a href="https://github.com/o2sh/onefetch/issues?q=is%3Aissue+is%3Aopen+label%3A%22help+wanted%22"><img src="https://img.shields.io/github/issues/o2sh/onefetch/help%20wanted?color=green" alt="help wanted"></a>
	<a href="../LICENSE.md"><img src="https://img.shields.io/badge/license-MIT-blue.svg"></a>
	<img src="../assets/msrv-badge.svg">
</p>

<p align="center">
  <a href="docs/README.ja.md">日本語</a> | <a href="docs/README.fa.md">فارسی</a> |
  <a href="docs/README.cn.md">简体中文</a> | <a href="docs/README.ru.md">Русский</a> |
  <a href="docs/README.es.md">Español</a> | <a href="docs/README.fr.md">Français</a>
</p>

<img src="../assets/screenshot-1.png" align="right" height="240px">

Onefetch est un outil en ligne de commande pour Git écrit en `Rust` qui affiche les informations d'un projet et les statistiques de codes pour un dépôt local directement dans le terminal. Cet outil est complètement hors ligne - aucun accès internet n'est recquis.

Par défaut, les informations du dépôt sont affichées à côté du logo du langage dominant, mais vous pouvez configurer onfetch pour afficher une autre image de votre choix - sur les emulateurs le supportant -, ou du text, ou rien du tout.

Il détecte automatiquement les licences open sources depuis le text et fournit à l'utilisateur plusieurs informations comme la distribution du code, les changements en attentes, le nombres de dépendances (par le manager de paquets), les tops contributeurs (par nombre de commits), la taille sur le disque, la date de création, LOC (ligne de code), etc.

<img src="../assets/screenshot-2.png" align="right" height="240px">

Onefetch peut être configuré en passent des arguments en ligne de commande pour afficher exactement ce que vous voulez, de la manière dont vous voulez: vous pouvez personnaliser du text au format ASCII/text, désactiver les informations sur les lignes, ignorer des fichiers & répertoires, afficher la sortie dans plusieurs formats (Json, Yaml), etc.
Actuellement, onefetch supporte plus de 50 langages de programmation différents; si le langage de votre choix n'est pas supporté, ouvrez une issue et le support sera ajouté.

Les contributions sont les bienvenues! Rendez-vous sur la page [CONTRIBUTING](../CONTRIBUTING.md) pour plus d'informations.

### More: \[[Wiki](https://github.com/o2sh/onefetch/wiki)\] \[[Installation](https://github.com/o2sh/onefetch/wiki/Installation)\] \[[Premiers Pas](https://github.com/o2sh/onefetch/wiki/getting-started)\]
