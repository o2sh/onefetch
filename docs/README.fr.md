<h3 align="center"><img src="../assets/onefetch.svg" height="130px"></h3>

<h5 align="center">Outil Git en ligne de commande écrit en Rust</h5>

<p align="center">
	<a href="https://crates.io/crates/onefetch"><img src="https://img.shields.io/crates/v/onefetch.svg" alt="cargo"></a>
	<a href="https://github.com/o2sh/onefetch/actions"><img src="https://github.com/o2sh/onefetch/workflows/CI/badge.svg" alt="Build Status"></a>
  <a href="https://onefetch.dev"><img src="../assets/language-badge.svg"></a>
	<a href="https://github.com/o2sh/onefetch/issues?q=is%3Aissue+is%3Aopen+label%3A%22%E2%9D%93+help+wanted%22"><img src="https://img.shields.io/github/issues/o2sh/onefetch/%E2%9D%93%20help%20wanted?color=green" alt="help wanted"></a>
	<a href="../LICENSE.md"><img src="https://img.shields.io/badge/license-MIT-blue.svg"></a>
	<img src="../assets/msrv-badge.svg">
</p>

<img src="../assets/screenshot-1.png" align="right" height="250px">

Onefetch est un outil Git en ligne de commande écrit en `Rust` qui permet d'afficher les informations d'un projet et des statistiques de codes pour un dépôt local directement depuis un terminal. L'outil est complètement hors ligne - aucun accès réseau n'est requis.

Par défaut, les informations du dépôt sont affichées à côté d'une représentation ASCII du logo du langage dominant, mais vous pouvez choisir d'afficher une image à la place - sur les terminaux le supportant -, du texte, ou laisser vide.

Il détecte automatiquement les licences open sources et fournit à l'utilisateur plusieurs informations utiles comme la décomposition du code par langage, les changements en attentes, le nombres de dépendances (par gestionnaire de paquets), les principaux contributeurs (par nombre de commits), la taille du dépôt sur le disque, la date de création, le nombre de lignes de code, etc.

<img src="../assets/screenshot-2.png" align="right" height="250px">

Onefetch peut être configuré afin d'afficher exactement ce que vous souhaitez, comme vous le souhaitez : vous pouvez personnaliser le formatage du texte, cacher certaines lignes, ignorer des fichiers & répertoires, générer la sortie dans plusieurs formats (Json, Yaml), etc.

Actuellement, onefetch supporte plus de [100 langages de programmation](https://onefetch.dev); si le langage de votre choix n'est pas supporté, créez une issue et le support sera ajouté.

Les contributions sont plus que bienvenues! Rendez-vous sur la page [CONTRIBUTING](../CONTRIBUTING.md) pour plus d'informations.

### Compléments: \[[Wiki](https://github.com/o2sh/onefetch/wiki)\] \[[Installation](https://github.com/o2sh/onefetch/wiki/Installation)\] \[[Premiers Pas](https://github.com/o2sh/onefetch/wiki/getting-started)\]
