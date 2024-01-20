<h3 align="center"><img src="assets/onefetch.svg" height="130px"></h3>

<h5 align="center">Ferramenta de informações Git de linha de comando escrita em Rust</h5>


<p align="center">
	<a href="https://crates.io/crates/onefetch"><img src="https://img.shields.io/crates/v/onefetch.svg" alt="cargo"></a>
	<a href="https://github.com/o2sh/onefetch/actions"><img src="https://github.com/o2sh/onefetch/workflows/CI/badge.svg" alt="Build Status"></a>
  <a href="https://onefetch.dev"><img src="assets/language-badge.svg"></a>
  <a href="https://github.com/o2sh/onefetch/issues?q=is%3Aissue+is%3Aopen+label%3A%22help+wanted%22"><img src="https://img.shields.io/github/issues/o2sh/onefetch/help%20wanted?color=green" alt="help wanted"></a>
	<a href="./LICENSE.md"><img src="https://img.shields.io/badge/license-MIT-blue.svg"></a>
	<img src="assets/msrv-badge.svg">
</p>

<img src="assets/screenshot-1.png" align="right" height="250px">

Onefetch é uma ferramenta de linha de comando de informações Git escrita em `Rust` que exibe informações do projeto e estatísticas de código para um repositório Git local diretamente em seu terminal. A ferramenta está totalmente offline - não é necessário acesso à rede.

Por padrão, as informações do repositório são exibidas ao lado do logotipo do idioma dominante, mas você pode configurar ainda mais o onefetch para usar uma imagem - em terminais suportados -, uma entrada de texto ou nada.

Ele detecta automaticamente licenças de código aberto a partir de textos e fornece ao usuário informações valiosas como distribuição de código, alterações pendentes, número de dependências (por gerenciador de pacotes), principais contribuidores (por número de commits), tamanho no disco, data de criação, LOC (linhas de código), etc.

<img src="assets/screenshot-2.png" align="right" height="250px">

Onefetch pode ser configurado por meio de flags de linha de comando para exibir exatamente o que você deseja, da maneira que desejar: você pode personalizar a formatação ASCII/Texto, desativar linhas de informações, ignorar arquivos e diretórios, gerar saída em vários formatos (Json, Yaml), etc.

A partir de agora, onefetch oferece suporte a mais de [100 linguagens de programação diferentes](https://onefetch.dev); se a linguagem não estiver disponível: abra um issue e o suporte para a linguagem será adicionado.

Contribuições são muito bem-vindas! Consulte [CONTRIBUTING](CONTRIBUTING.md) para obter mais informações.

### More: \[[Wiki](https://github.com/o2sh/onefetch/wiki)\] \[[Installation](https://github.com/o2sh/onefetch/wiki/Installation)\] \[[Getting Started](https://github.com/o2sh/onefetch/wiki/getting-started)\]
