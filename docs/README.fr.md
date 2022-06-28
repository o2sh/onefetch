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

<img src="../assets/screenshot-1.png" align="right" height="240px">

Onefetch est un outil en ligne de commande pour Git écrit en `Rust` qui affiche les informations d'un projet et les statistiques de codes pour un dépôt local directement dans le terminal. Cet outil est complètement hors ligne - aucun accès internet n'est requis.

Par défaut, les informations du dépôt sont affichées à côté du logo du langage dominant, mais vous pouvez configurer onefetch pour afficher une autre image à la place - sur les emulateurs le supportant -, ou du texte, ou rien du tout.

Il détecte automatiquement les licences open sources et fournit à l'utilisateur plusieurs informations comme la décomposition du code par langage, les changements en attentes, le nombres de dépendances (par le manager de paquets), les tops contributeurs (par nombre de commits), la taille sur le disque, la date de création, LOC (ligne de code), etc.

<img src="../assets/screenshot-2.png" align="right" height="240px">

Onefetch peut être configuré en passent des arguments en ligne de commande pour afficher exactement ce que vous voulez, de la manière dont vous voulez: vous pouvez personnaliser le formatage du texte au format ASCII/texte, cacher certaines lignes, ignorer des fichiers & répertoires, afficher la sortie dans plusieurs formats (Json, Yaml), etc.
Actuellement, onefetch supporte plus de 50 langages de programmation différents; si le langage de votre choix n'est pas supporté, ouvrez une issue et le support sera ajouté.

Les contributions sont les bienvenues! Rendez-vous sur la page [CONTRIBUTING](../CONTRIBUTING.md) pour plus d'informations.

### Complément: \[[Wiki](https://github.com/o2sh/onefetch/wiki)\] \[[Installation](https://github.com/o2sh/onefetch/wiki/Installation)\] \[[Premiers Pas](https://github.com/o2sh/onefetch/wiki/getting-started)\]
