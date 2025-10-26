# 🚫 Lista de Bloqueio Procon-SP

<div align="center">

**Listas de bloqueio, atualizadas automaticamente, baseadas na relação de sites não confiáveis da [Fundação Procon-SP](https://sistemas.procon.sp.gov.br/evitesite/list/evitesites.php).**

[![Rust](https://img.shields.io/badge/rust-1.90.0-orange.svg)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![GitHub release](https://img.shields.io/github/release/glauberlima/procon-blocklist.svg)](https://github.com/glauberlima/procon-blocklist/releases)

</div>

---

> **⚠️ Aviso Importante**: Este é um projeto comunitário e não possui nenhuma ligação oficial com a Fundação Procon-SP. A iniciativa apenas utiliza a API pública disponível no [site oficial](https://sistemas.procon.sp.gov.br/evitesite/) para gerar listas de bloqueio de forma automatizada, visando proteger os consumidores brasileiros.

---

## 📥 Listas de Bloqueio Prontas para Uso

As listas são atualizadas automaticamente de segunda a sexta-feira. Para se proteger, basta adicionar um dos links abaixo ao seu bloqueador de anúncios preferido.

| Formato | Descrição | Link |
|---------|-----------|------|
| **AdBlock** (recomendado) | Lista de filtros para bloqueadores como uBlock Origin, AdGuard | [Download](https://raw.githubusercontent.com/glauberlima/procon-blocklist/main/lists/adblock.txt) |
| **Hosts** | Arquivo hosts para bloqueio em nível de sistema | [Download](https://raw.githubusercontent.com/glauberlima/procon-blocklist/main/lists/hosts.txt) |

### ⚙️ Como Adicionar a Lista ao seu Bloqueador

#### uBlock Origin, AdGuard, Adblock Plus, etc.

1.  Copie o link do formato AdBlock acima.
2.  Abra as configurações do seu bloqueador de anúncios.
3.  Procure a seção "Listas de Filtros", "Minhas Listas" ou similar.
4.  Escolha a opção para adicionar uma nova lista a partir de uma URL e cole o link.

#### Arquivo de Hosts (Avançado)

Este método bloqueia os sites em todo o sistema operacional.

- **Linux / macOS:**
  ```bash
  sudo curl https://raw.githubusercontent.com/glauberlima/procon-blocklist/main/lists/hosts.txt >> /etc/hosts
  ```
- **Windows (usando PowerShell como Administrador):**
  ```powershell
  curl https://raw.githubusercontent.com/glauberlima/procon-blocklist/main/lists/hosts.txt >> C:\Windows\System32\drivers\etc\hosts
  ```

## ✨ Sobre o Projeto

- 🚀 **Rápido**: Desempenho otimizado com Rust.
- 📋 **Múltiplos Formatos**: Gera listas para Adblock e formato Hosts.
- 🔄 **Dados em Tempo Real**: Busca os dados mais recentes da API da Fundação Procon-SP.
- 🌍 **Multiplataforma**: Suporte para Linux, macOS e Windows.
- 📖 **Código Aberto**: Totalmente transparente e aberto a contribuições.

---

## 🛠️ Para Usuários Avançados e Desenvolvedores

Esta seção contém informações para quem deseja executar a ferramenta manualmente ou contribuir com o projeto.

### Instalação e Uso do CLI

A ferramenta `procon-cli` permite gerar as listas localmente.

1.  **Instalação:** Baixe o executável para o seu sistema operacional na [**página de Releases**](https://github.com/glauberlima/procon-blocklist/releases) ou, se tiver o [Rust instalado](https://www.rust-lang.org/tools/install), compile a partir do código-fonte:
    ```bash
    git clone https://github.com/glauberlima/procon-blocklist.git
    cd procon-blocklist
    cargo build --release
    ```

2.  **Uso:**
    ```bash
    # Gerar lista AdBlock e exibir no terminal
    ./target/release/procon-cli generate

    # Gerar lista Hosts e salvar em um arquivo
    ./target/release/procon-cli generate --format hosts --output hosts.txt
    ```

### Contribuição

Contribuições são muito bem-vindas!

1.  Faça um **Fork** do repositório.
2.  Crie uma **Branch** para sua alteração.
3.  Após implementar, certifique-se que os testes passam com `cargo test`.
4.  Abra um **Pull Request**.

## 📜 Licença

Este projeto está licenciado sob a **Licença MIT**.

## 🙏 Agradecimentos

- **Fundação Procon-SP** pela disponibilização dos dados.
- **Comunidade Rust** pelo ecossistema incrível.

---
Feito com ❤️, ☕, Rust e IA generativa
