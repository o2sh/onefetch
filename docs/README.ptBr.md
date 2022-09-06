<h3 align="center"><img src="assets/onefetch.svg" height="130px"></h3>

<h5 align="center"> Uma ferramenta de informações Git para linha de comando escrito em Rust </h5>

<p align="center">
	<a href="https://crates.io/crates/onefetch"><img src="https://img.shields.io/crates/v/onefetch.svg" alt="cargo"></a>
	<a href="https://github.com/o2sh/onefetch/actions"><img src="https://github.com/o2sh/onefetch/workflows/CI/badge.svg" alt="Build Status"></a>
  <a href="https://onefetch.dev"><img src="assets/language-badge.svg"></a>
	<a href="https://github.com/o2sh/onefetch/issues?q=is%3Aissue+is%3Aopen+label%3A%22help+wanted%22"><img src="https://img.shields.io/github/issues/o2sh/onefetch/help%20wanted?color=green" alt="help wanted"></a>
	<a href="./LICENSE.md"><img src="https://img.shields.io/badge/license-MIT-blue.svg"></a>
	<img src="assets/msrv-badge.svg">
</p>

<img src="assets/screenshot-1.png" align="right" height="240px">

Onefetch é uma ferramenta de informações Git para linha de comando escrita em `Rust`, que exibe informações do projeto e estatísticas de código para um repositório local Git direto no seu terminal. A ferramenta está completamente offline - não é necessário acesso à rede.

Por padrão, as informações do repositório são exibidas ao lado do logotipo do idioma dominante, mas você pode configurar o onefetch para usar uma imagem - em terminais suportados -, uma entrada de texto ou não ter exebição.

Ele detecta automaticamente licenças de código aberto a partir de textos e fornece ao usuário informações valiosas como linguagem do código, alterações pendentes, número de dependências (por gerenciador de pacotes), principais contribuidores (por número de commits), tamanho no disco, data de criação, LOC (linhas de código), etc.
<img src="assets/screenshot-2.png" align="right" height="240px">

Onefetch pode ser configurado por meio de sinalizadores de linha de comando para exibir exatamente o que você deseja, do jeito que você queira: você pode personalizar a formatação ASCII/Texto, desabilitar linhas de informação, ignorar arquivos e diretórios e saída de vários formatos (Json, Yaml), etc.

A partir de agora, o onefetch suporta mais de [50 linguagens de programação diferentes](https://onefetch.dev); se o idioma de sua escolha não for suportado: relate um problema com a linguagen desejada e o suporte irá adicionar.

Contribuições são muito bem-vindas! Consulte [CONTRIBUTING](CONTRIBUTING.md) para mais informações.

### Mais: \[[Wiki](https://github.com/o2sh/onefetch/wiki)\] \[[Instalação](https://github.com/o2sh/onefetch/wiki/Installation)\] \[[Começando](https://github.com/o2sh/onefetch/wiki/getting-started)\]
