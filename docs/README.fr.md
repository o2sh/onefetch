<h3 align="center"><img src="../assets/onefetch.svg" height="130px"></h3>

<h5 align="center">Un outil Git en ligne de commande écrit en Rust</h5>

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

Par défaut, les informations du dépôt sont affichées à côté du logo du langage dominant, mais vous pouvez configurer onefetch pour afficher image à la place - sur les terminaux le supportant -, ou du texte, ou rien du tout.

Il détecte automatiquement les licences open sources et fournit à l'utilisateur plusieurs informations comme la décomposition du code par langage, les changements en attentes, le nombres de dépendances (par le gestionnaire de paquets), les tops contributeurs (par nombre de commits), la taille du dépôt le disque, la date de création, LOC (ligne de code), etc.

<img src="../assets/screenshot-2.png" align="right" height="240px">

Onefetch peut être configuré afin d'afficher exactement ce que vous souhaitez, comme vous le souhaitez : vous pouvez personnaliser le formatage du texte, cacher certaines lignes, ignorer des fichiers & répertoires, générer la sortie dans plusieurs formats (Json, Yaml), etc.
Actuellement, onefetch supporte plus de 50 langages de programmation différents; si le langage de votre choix n'est pas supporté, ouvrez une issue et le support sera ajouté.

Les contributions sont plus que bienvenues! Rendez-vous sur la page [CONTRIBUTING](../CONTRIBUTING.md) pour plus d'informations.

### Complément: \[[Wiki](https://github.com/o2sh/onefetch/wiki)\] \[[Installation](https://github.com/o2sh/onefetch/wiki/Installation)\] \[[Premiers Pas](https://github.com/o2sh/onefetch/wiki/getting-started)\]
