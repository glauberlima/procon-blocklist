# 🚫 Procon CLI

<div align="center">

**Uma ferramenta CLI rápida e otimizada escrita em Rust para gerar listas de bloqueio de anúncios a partir do banco de dados de sites ruins do Procon-SP.**

[![Rust](https://img.shields.io/badge/rust-1.90.0-orange.svg)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![GitHub release](https://img.shields.io/github/release/glauberlima/procon-badsites.svg)](https://github.com/glauberlima/procon-badsites/releases)

[📥 Baixar Listas](#-listas-geradas) • [🚀 Instalação](#instalação) • [📖 Uso](#uso) • [🤝 Contribuição](#contribuição)

</div>

---

> **⚠️ Aviso Importante**: Este projeto não tem nenhuma ligação oficial com a Fundação Procon-SP. Ele apenas utiliza a API pública disponível no site oficial ([sistemas.procon.sp.gov.br/evitesite](https://sistemas.procon.sp.gov.br/evitesite/)) para prestar um serviço útil gerando automaticamente listas de bloqueio de anúncios em formatos práticos e acessíveis. O site oficial nunca ofereceu essa funcionalidade de forma automatizada e simples.

---

## ✨ Recursos

- 🚀 **Rápido**: Escrito em Rust para desempenho ótimo
- 📋 **Múltiplos formatos**: Gera formatos de arquivo adblock e hosts
- 🔄 **Dados em tempo real**: Busca os dados mais recentes da API do Procon-SP
- 💾 **Saída flexível**: Salva em arquivo ou envia para stdout
- 🧪 **Bem testado**: Inclui testes unitários abrangentes
- 🌍 **Multi-plataforma**: Suporte para Linux, macOS e Windows

## 📥 Listas Geradas

As listas são atualizadas automaticamente todos os dias úteis às 02:30 (horário de Brasília). Use os links abaixo para adicionar diretamente aos seus bloqueadores de anúncios:

### 🛡️ Lista AdBlock
```
https://raw.githubusercontent.com/glauberlima/procon-badsites/main/lists/adblock.txt
```

### 🖥️ Lista Hosts
```
https://raw.githubusercontent.com/glauberlima/procon-badsites/main/lists/hosts.txt
```

### 📊 Estatísticas
- **Última atualização**: Verifique o cabeçalho dos arquivos
- **Frequência**: Diariamente (segunda a sexta-feira)
- **Fonte**: [Procon-SP - Evite Esses Sites](https://sistemas.procon.sp.gov.br/evitesite/list/evitesites.php)

## Instalação

### 📦 Binários Pré-compilados

Baixe a versão mais recente da [página de releases](https://github.com/glauberlima/procon-badsites/releases):

- **Linux** (x86_64): `procon-cli-x86_64-unknown-linux-gnu.tar.gz`
- **macOS** (Intel): `procon-cli-x86_64-apple-darwin.tar.gz`
- **macOS** (Apple Silicon): `procon-cli-aarch64-apple-darwin.tar.gz`
- **Windows** (x86_64): `procon-cli-x86_64-pc-windows-gnu.zip`

### 🛠️ A partir do Código-Fonte

```bash
# Clone o repositório
git clone https://github.com/glauberlima/procon-badsites.git
cd procon-badsites/procon-cli

# Compile em modo release
cargo build --release

# O binário estará disponível em target/release/procon-cli
```

## 🚀 Uso

### 📋 Gerar Lista AdBlock (padrão)

```bash
# Saída para stdout
procon-cli generate

# Salvar em arquivo
procon-cli generate --output adblock.txt
```

### 🖥️ Gerar Arquivo Hosts

```bash
# Saída para stdout
procon-cli generate --format hosts

# Salvar em arquivo
procon-cli generate --format hosts --output hosts.txt
```

### ❓ Ajuda

```bash
# Ajuda geral
procon-cli --help

# Ajuda do comando generate
procon-cli generate --help
```

### 📱 Integração com Bloqueadores

#### uBlock Origin / AdBlock Plus
1. Abra as configurações do seu bloqueador
2. Adicione filtro personalizado
3. Cole o link RAW: `https://raw.githubusercontent.com/glauberlima/procon-badsites/main/lists/adblock.txt`

#### Hosts File (Sistema)
```bash
# Linux/macOS
sudo curl https://raw.githubusercontent.com/glauberlima/procon-badsites/main/lists/hosts.txt >> /etc/hosts

# Windows (como administrador)
curl https://raw.githubusercontent.com/glauberlima/procon-badsites/main/lists/hosts.txt >> C:\Windows\System32\drivers\etc\hosts
```

## Formatos de Saída

### Formato Adblock
```
! Title: Evite esses Sites - Procon-SP
! Expires: 1 day
! Description: Lista gerada a partir do site Evite esses Sites - https://sistemas.procon.sp.gov.br/evitesite/list/evitesites.php - Fundação Procon/SP
! Homepage: https://github.com/glauberlima/procon-badsites
! Licence: https://github.com/glauberlima/procon-badsites/blob/main/LICENSE
! Updated: 26 October 2025 11:08:53 (GMT+00:00)
||example.com^
||bad-site.org^
```

### Formato Hosts
```
# Title: Evite esses Sites - Procon-SP
# Description: Lista gerada a partir do site Evite esses Sites - https://sistemas.procon.sp.gov.br/evitesite/list/evitesites.php - Fundação Procon/SP
# Homepage: https://github.com/glauberlima/procon-badsites
# Licence: https://github.com/glauberlima/procon-badsites/blob/main/LICENSE
# Updated: 26 October 2025 11:08:53 (GMT+00:00)
0.0.0.0 example.com
0.0.0.0 bad-site.org
```

## 🏗️ Desenvolvimento

### 📋 Pré-requisitos

- Rust 1.90.0 ou posterior
- Cargo (vem com Rust)

### ⚙️ Configuração

```bash
# Clone o repositório
git clone https://github.com/glauberlima/procon-badsites.git
cd procon-badsites/procon-cli

# Instale dependências
cargo fetch
```

### 🔨 Compilação

```bash
# Debug (desenvolvimento)
cargo build

# Release (otimizado)
cargo build --release
```

### 🧪 Testes

```bash
# Executar todos os testes
cargo test

# Com output detalhado
cargo test -- --nocapture

# Testes de benchmark (se houver)
cargo bench
```

### ▶️ Execução

```bash
# Modo desenvolvimento
cargo run -- generate

# Usando binário compilado
./target/release/procon-cli generate
```

## 🏛️ Arquitetura

O CLI segue princípios de arquitetura limpa:

- **🔸 Responsabilidade Única**: Cada função tem um propósito claro
- **🔸 DRY**: Sem duplicação de código
- **🔸 KISS**: Implementação simples e direta
- **🔸 YAGNI**: Implementa apenas o necessário
- **🔸 SOLID**: Código bem estruturado e testável

### 📁 Estrutura do Projeto

```
procon-cli/
├── src/
│   ├── main.rs          # Ponto de entrada e lógica principal
│   └── lib.rs           # (Futuro) Biblioteca compartilhada
├── tests/               # Testes de integração
├── Cargo.toml           # Metadados e dependências
└── README.md            # Esta documentação
```

## 📜 Licença

Este projeto está licenciado sob a **Licença MIT** - veja o arquivo [LICENSE](../LICENSE) para detalhes.

## 🤝 Contribuição

Contribuições são bem-vindas! 🎉

1. 🍴 Faça um fork do repositório
2. 🌿 Crie uma branch de funcionalidade (`git checkout -b feature/AmazingFeature`)
3. 💾 Faça suas alterações
4. 🧪 Adicione testes para novas funcionalidades
5. ✅ Certifique-se de que todos os testes passam
6. 📝 Atualize a documentação se necessário
7. 🚀 Faça o commit das suas alterações (`git commit -m 'Add some AmazingFeature'`)
8. 📤 Envie para o repositório (`git push origin feature/AmazingFeature`)
9. 🔄 Abra um Pull Request

### 📋 Diretrizes de Contribuição

- Siga o [Conventional Commits](https://conventionalcommits.org/)
- Mantenha o código limpo e bem documentado
- Adicione testes para novas funcionalidades
- Atualize o README se necessário

## 🔗 Projetos Relacionados

- [**procon-badsites**](https://github.com/glauberlima/procon-badsites) - Repositório principal com as listas geradas
- [**Procon-SP**](https://sistemas.procon.sp.gov.br/evitesite/) - Site oficial do Procon-SP

## 🙏 Agradecimentos

- **Fundação Procon-SP** pela disponibilização dos dados
- **Comunidade Rust** pelo ecossistema incrível
- **Contribuidores** que ajudam a manter o projeto

---

<div align="center">

**Feito com ❤️ e Rust para proteger usuários brasileiros contra sites maliciosos**

[⭐ Star este repo](https://github.com/glauberlima/procon-badsites) • [🐛 Reportar bug](https://github.com/glauberlima/procon-badsites/issues) • [💡 Sugerir feature](https://github.com/glauberlima/procon-badsites/issues)

</div>