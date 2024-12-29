<h3 align="center"><img src="../assets/onefetch.svg" height="130px"></h3>

<h5 align="center">Narzędzie informacyjne Git CLI napisane w Rust</h5>

<p align="center">
	<a href="https://crates.io/crates/onefetch"><img src="https://img.shields.io/crates/v/onefetch.svg" alt="cargo"></a>
	<a href="https://github.com/o2sh/onefetch/actions"><img src="https://github.com/o2sh/onefetch/workflows/CI/badge.svg" alt="Build Status"></a>
  <a href="https://github.com/o2sh/onefetch/issues?q=is%3Aissue+is%3Aopen+label%3A%22help+wanted%22"><img src="https://img.shields.io/github/issues/o2sh/onefetch/help%20wanted?color=green" alt="help wanted"></a>
	<a href="../LICENSE.md"><img src="https://img.shields.io/badge/license-MIT-blue.svg"></a>
	<img src="../assets/msrv-badge.svg">
</p>

<img src="../assets/screenshot-1.png" align="right" height="250px">

Onefech jest narzędziem informacyjnym Git CLI napisanym w `Rust`. Wyświetla ono informacje o projekcie i statystyki kodu lokalnego repozytorium bezpośrednio w terminalu. Jest także w pełni offline - dostęp do internetu nie jest wymagany.

Domyślnie, informacje o repozytorium są wyświetlane obok logo dominującego języka, ale można skonfigurować Onefetch aby zamiast tego wyświetlał zdjęcie (we wspieranych terminalach), tekst lub nic.

Narzędzie automatycznie wykrywa licencje open source i zapewnia użytkownikowi cenne informacje, takie jak dystrybucja kodu, oczekujące zmiany, liczba zależności (według menedżera pakietów), top contributorzy (według liczby commitów), rozmiar na dysku, data utworzenia, number linii kodu, itp.

<img src="../assets/screenshot-2.png" align="right" height="250px">

Onefetch może być konfigurowany poprzez flagi CLI tak, aby wyświetlał dokładnie to, czego chcesz, tak, jak chcesz: można dostosować formatowanie ASCII/tekstu, wyłączyć wiersze informacyjne, ignorować pliki i katalogi, a nawet outputować dane w różnych formatach (JSON, YAML) itd.

Obecnie onefetch obsługuje ponad [100 różnych języków programowania](https://onefetch.dev). Jeśli wybrany przez Ciebie język nie jest obsługiwany: zgłoś Issue, a wsparcie zostanie dodane.

Wkłady są mile widziane! Więcej informacji znajdziesz w [CONTRIBUTING](../CONTRIBUTING.md).

### Więcej: \[[Wiki](https://github.com/o2sh/onefetch/wiki)\] \[[Instalacja](https://github.com/o2sh/onefetch/wiki/Installation)\] \[[Jak Zacząć](https://github.com/o2sh/onefetch/wiki/getting-started)\]
