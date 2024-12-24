<h3 align="center"><img src="../assets/onefetch.svg" height="130px"></h3>

<h5 align="center">Uno strumento a riga di comando per informazioni su Git scritto in Rust</h5>

<p align="center">
	<a href="https://crates.io/crates/onefetch"><img src="https://img.shields.io/crates/v/onefetch.svg" alt="cargo"></a>
	<a href="https://github.com/o2sh/onefetch/actions"><img src="https://github.com/o2sh/onefetch/workflows/CI/badge.svg" alt="Build Status"></a>
  <a href="https://github.com/o2sh/onefetch/issues?q=is%3Aissue+is%3Aopen+label%3A%22help+wanted%22"><img src="https://img.shields.io/github/issues/o2sh/onefetch/help%20wanted?color=green" alt="help wanted"></a>
	<a href="../LICENSE.md"><img src="https://img.shields.io/badge/license-MIT-blue.svg"></a>
	<img src="../assets/msrv-badge.svg">
</p>

<img src="../assets/screenshot-1.png" align="right" height="250px">

Onefetch è uno strumento a riga di comando per ottenere informazioni su Git ed è scritto in `Rust`. Il programma mostra le informazioni e le statistiche del codice di una repository locale direttamente nel terminale, senza necessitare di una connessione ad Internet.

Di default, le informazioni della repo vengono mostrate accanto al logo del linguaggio più utilizzato, ma è possibile configurare Onefetch in modo che mostri un'immagine (se il terminale lo supporta), del testo fornito come input oppure nulla.

Rileva automaticamente le licenze open source e fornisce all'utente informazioni utili riguardo alla distribuzione del codice, ai cambiamenti in attesa, al numero di dipendenze (divise per package manager), ai maggiori contributori (per numero di commit), alla dimensione sul disco, alla data di creazione, alle linee di codice, ecc.

<img src="../assets/screenshot-2.png" align="right" height="250px">

Onefetch può essere configurato con delle flag per mostrare esattamente ciò che desideri, nel modo che preferisci: puoi personalizzare l'ASCII e la formattazione del testo, disabilitare informazioni specifiche, ignorare file e cartelle e formattare il testo in formati come JSON e YAML, ecc.

Al momento onefetch supporta più di [100 diversi linguaggi di programmazione](https://onefetch.dev); Se il linguaggio che hai scelto non è supportato apri una issue e lo supporteremo.

I contributi sono molto graditi! Vedi [CONTRIBUTING](../CONTRIBUTING.md) per avere più informazioni.

### Altro: \[[Wiki](https://github.com/o2sh/onefetch/wiki)\] \[[Installazione](https://github.com/o2sh/onefetch/wiki/Installation)\] \[[Per iniziare](https://github.com/o2sh/onefetch/wiki/getting-started)\]
