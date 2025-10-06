# Curso Polkadot

Este projeto utiliza um ambiente de desenvolvimento baseado em **Dev Container** para facilitar a configuração e o uso das ferramentas necessárias.

> **Nota:** Este material foi desenvolvido para fins de estudo durante o curso [Formação Blockchain Polkadot SDK](https://github.com/curso-polkadot-sdk/formacao-blockchain-polkadot-sdk).

## Sumário

- [Como rodar o ambiente com Dev Container](#como-rodar-o-ambiente-com-dev-container)
- [Como rodar o ambiente no GitHub Codespaces](#como-rodar-o-ambiente-no-github-codespaces)
- [Dependências já presentes no projeto](#dependências-já-presentes-no-projeto)
- [Referências](#referências)

## Como rodar o ambiente com Dev Container

1. **Pré-requisitos**

   - [Docker](https://www.docker.com/) **ou** [Rancher Desktop](https://rancherdesktop.io/) instalado
   - [Visual Studio Code](https://code.visualstudio.com/) com a extensão [Dev Containers](https://marketplace.visualstudio.com/items?itemName=ms-vscode-remote.remote-containers)

2. **Abrindo o projeto no Dev Container**

   - Abra o VS Code.
   - Selecione `File > Open Folder` e escolha o diretório do projeto.
   - Clique em `Reopen in Container` quando solicitado, ou use o comando `Dev Containers: Reopen in Container` pela paleta de comandos (`Ctrl+Shift+P`).

3. **Ambiente pré-configurado**  
   O Dev Container já inclui:

   - Git (versão atualizada)
   - Rust e utilitários comuns
   - Dependências necessárias para desenvolvimento Polkadot
   - Ferramentas de linha de comando úteis

4. **Utilizando o terminal**  
   Após abrir o projeto no Dev Container, utilize o terminal integrado do VS Code para executar comandos como `git`, `cargo`, entre outros.

## Como rodar o ambiente no GitHub Codespaces

1. Certifique-se de que o repositório está hospedado no GitHub.
2. No repositório, clique no botão verde “<> Code” e selecione a aba “Codespaces”.
3. Clique em “Create codespace on main” (ou no branch desejado).
4. Aguarde o ambiente ser provisionado automaticamente.
5. O Codespaces utilizará a configuração do Dev Container já presente no projeto.

Você poderá programar diretamente do navegador ou abrir o Codespace no VS Code.

## Dependências já presentes no projeto

O ambiente do Dev Container já inclui as seguintes dependências e ferramentas instaladas e disponíveis no `PATH`:

- **Git** (compilado a partir do código-fonte, versão atualizada)
- **Rust** (toolchain completo, incluindo `cargo`)
- **Utilitários Rust**: `rustfmt`, `clippy`, entre outros
- **Ferramentas de linha de comando**: `apt`, `dpkg`, `curl`, `wget`, `ssh`, `scp`, `rsync`, `gpg`, `ps`, `lsof`, `netstat`, `top`, `tree`, `find`, `grep`, `zip`, `unzip`, `tar`, `gzip`, `bzip2`, `xz`
- **Dependências para desenvolvimento Polkadot** (pré-instaladas conforme necessário)

Essas dependências garantem um ambiente pronto para desenvolvimento com Rust e Polkadot, sem necessidade de configurações adicionais.


## Como executar os projetos

Consulte o arquivo [`COMO_EXECUTAR.md`](./COMO_EXECUTAR.md) para instruções detalhadas de execução de cada projeto deste repositório.

## Referências

- [Documentação Dev Containers](https://code.visualstudio.com/docs/devcontainers/containers)
- [GitHub Codespaces](https://docs.github.com/en/codespaces)
- [Rust](https://www.rust-lang.org/)
- [Polkadot](https://polkadot.network/)
