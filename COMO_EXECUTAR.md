# Execução dos Projetos

Este repositório contém múltiplos projetos Rust, cada um em seu próprio diretório. Abaixo estão as instruções para executar cada um deles.

## Pré-requisitos
- Docker (para uso com Dev Container) OU
- Instalação local do [Rust](https://www.rust-lang.org/tools/install)

## Usando o Dev Container
Se estiver usando o Dev Container, basta abrir o terminal integrado do VS Code e seguir os comandos abaixo para cada projeto.

## Projetos Disponíveis

- `constantes`
- `escopo`
- `funcoes_basicas`
- `seguranca_memoria`
- `shadowing`
- `tipos_basicos`

Cada projeto possui um arquivo `Cargo.toml` e um diretório `src/` com o código-fonte.

## Como executar cada projeto

1. Abra o terminal na raiz do repositório.
2. Acesse o diretório do projeto desejado. Exemplo:

   ```bash
   cd constantes
   ```

3. Execute o projeto com o comando:

   ```bash
   cargo run
   ```

4. Para compilar sem executar:

   ```bash
   cargo build
   ```

5. Para rodar testes (se houver):

   ```bash
   cargo test
   ```

Repita o processo para cada diretório de projeto.

## Observações
- O comando `cargo run` irá compilar e executar o projeto principal definido em `src/main.rs`.
- Certifique-se de estar no diretório correto antes de rodar os comandos.

---

Dúvidas ou problemas? Abra uma issue ou consulte a documentação oficial do Rust.