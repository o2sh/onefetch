<h3 align="center"><img src="../assets/onefetch.svg" height="130px"></h3>

<h5 align="center">Терминальная утилита для вывода информации о Git
репозиториях, написанная на Rust</h5>

<p align="center">
	<a href="https://crates.io/crates/onefetch"><img src="https://img.shields.io/crates/v/onefetch.svg" alt="cargo"></a>
	<a href="https://github.com/o2sh/onefetch/actions"><img src="https://github.com/o2sh/onefetch/workflows/CI/badge.svg" alt="Build Status"></a>
  <a href="https://onefetch.dev"><img src="../assets/language-badge.svg"></a>
	<a href="https://github.com/o2sh/onefetch/issues?q=is%3Aissue+is%3Aopen+label%3A%22%E2%9D%93+help+wanted%22"><img src="https://img.shields.io/github/issues/o2sh/onefetch/%E2%9D%93%20help%20wanted?color=green" alt="help wanted"></a>
	<a href="./LICENSE.md"><img src="https://img.shields.io/badge/license-MIT-blue.svg"></a>
	<img src="../assets/msrv-badge.svg">
</p>

<img src="../assets/screenshot-1.png" align="right" height="250px">

Onefetch выводит информацию о локальном Git репозитории прямо в терминал.
Утилита не нуждается в подключении к интернету.

По умолчанию информация о репозитории отображается вместе с логотипом главного
языка программирования, но вы можете сделать так, чтобы Onefetch использовал
изображение (если терминал это позволяет), читал ваш собственный ASCII-рисунок
или вообще не выводил его.

Утилита автоматически обнаруживает OpenSource лицензии по их тексту и предоставляет
различную информацию, вроде языков программирования, из которых состоит
репозиторий, изменений, ожидающих подтверждения, числа зависимостей (по
пакетному менеджеру), главных контрибьюторов (по числу коммитов),
размера занимаемого места на диске, даты создания, количества строк и т.п.

<img src="../assets/screenshot-2.png" align="right" height="250px">

Onefetch конфигурируется через флаги, позволяющие отображать только нужную вам
информацию, в нужном вам виде: вы можете менять форматирование текста,
выключать отображение определенных полей, игнорировать определенные файлы и директории,
выводить информацию в различных форматах (Json, Yaml) и т.д.

На данный момент Onefetch поддерживает более [100 языков программирования](https://onefetch.dev). Если
нужный вам язык не поддерживается, дайте нам об этом знать, мы добавим
его.

Вклады всегда приветствуются! Прочтите [CONTRIBUTING](../CONTRIBUTING.md),
чтобы узнать больше.

### Больше информации: \[[Wiki](https://github.com/o2sh/onefetch/wiki)\] \[[Установка](https://github.com/o2sh/onefetch/wiki/Installation)\] \[[Основы](https://github.com/o2sh/onefetch/wiki/getting-started)\]
