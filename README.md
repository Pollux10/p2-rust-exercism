# P2: Técnicas de Programação (Rust)

Avaliação Prática 2 da disciplina **Linguagens e Técnicas de Programação / Técnicas de Programação**, orientada pelo professor **Bruno Gabriel**.

**Dupla:**
- Rafael
- Mahgid

**Curso:** Engenharia da Computação - 6° período

## Sobre o trabalho

O trabalho consiste em resolver dois exercícios da trilha de Rust do [Exercism](https://exercism.org/), um de nível fácil e um de nível médio, cada um com **duas abordagens algoritmicamente distintas**. No total são quatro soluções, apresentadas em vídeo.

| Exercício | Nível | Abordagem 1 | Abordagem 2 |
|---|---|---|---|
| **Raindrops** (Gotas de Chuva) | Fácil | Imperativa: `String` mutável e três `if`s independentes | Funcional: tabela de regras (`const`) com iteradores (`iter`, `filter`, `map`, `collect`) |
| **Sublist** (Sublista) | Médio | Iteradores: `windows` + `any`, testando as duas direções | Comparação de tamanhos com `match` em `Ordering` e busca manual com `for` e `starts_with` |

### Raindrops

Converte um número em sons de gotas de chuva:

- divisível por 3 adiciona `Pling`;
- divisível por 5 adiciona `Plang`;
- divisível por 7 adiciona `Plong`;
- se não for divisível por nenhum dos três, o resultado é o próprio número como texto.

Exemplos: `28` vira `Plong`, `30` vira `PlingPlang`, `34` vira `34`.

### Sublist

Dadas duas listas `A` e `B`, determina se são iguais (`Equal`), se `A` contém `B` (`Superlist`), se `A` está contida em `B` (`Sublist`) ou se são diferentes (`Unequal`). A subsequência precisa ser **contígua**.

## Estrutura do repositório

```
P2 - 6°/
├── README.md
├── gotas de chuva/          # projeto "raindrops"
│   ├── Cargo.toml
│   ├── src/
│   │   ├── lib.rs           # escolhe qual abordagem é exportada
│   │   ├── abordagem1.rs
│   │   └── abordagem2.rs
│   ├── tests/
│   │   └── raindrops.rs
│   └── examples/
│       └── demo.rs
└── sublista/                # projeto "sublist"
    ├── Cargo.toml
    ├── src/
    │   ├── lib.rs
    │   ├── abordagem1.rs
    │   └── abordagem2.rs
    ├── tests/
    │   └── sublist.rs
    └── examples/
        └── demo.rs
```

Cada pasta é um projeto Cargo independente, criado como biblioteca (`cargo new --lib`). O nome do pacote no `Cargo.toml` (`raindrops` e `sublist`) é o que os testes usam para importar o código.

## Como executar

### Requisitos

- [Rust](https://rustup.rs) instalado (inclui o `cargo`).
- Opcional: VS Code com a extensão **rust-analyzer**.

### Escolher a abordagem

Em `src/lib.rs` de cada projeto, as duas abordagens são declaradas com `pub mod`, e uma linha `pub use` define qual delas os testes enxergam. Para alternar, comente uma linha e descomente a outra:

```rust
// Abordagem 1 ativa:
pub use abordagem1::raindrops;

// Abordagem 2 ativa:
// pub use abordagem2::raindrops;
```

No projeto `sublista`, a linha equivalente exporta a função `sublist`.

### Rodar os testes

Dentro da pasta de um dos projetos:

```bash
cargo test
```

Cada projeto tem 18 testes. A saída esperada termina com `test result: ok. 18 passed`.

### Rodar o demo

O demo chama **as duas abordagens** com vários casos e mostra os resultados lado a lado, indicando `OK` quando coincidem:

```bash
cargo run --example demo
```

Como os projetos são bibliotecas (não têm `main`), o comando é `cargo run --example demo` e não apenas `cargo run`.

### Formatação e lint

```bash
cargo fmt
cargo clippy
```

## Sobre os testes

Os arquivos em `tests/` foram escritos pela dupla seguindo o formato e os casos do Exercism. Eles **não são os arquivos oficiais** da plataforma. Os mesmos códigos podem ser colados no editor online do Exercism para rodar a suíte oficial.

## Decisões de implementação

- **Empréstimo em vez de cópia:** o Sublist usa `&[T]` (slices), sem `Vec` nem `clone`.
- **Genéricos com restrição mínima:** `T: PartialEq`, que é só o que as funções precisam para comparar elementos.
- **Enum e `match` exaustivo:** o resultado do Sublist é o `enum Comparison`, e o `match` em `Ordering` obriga o compilador a verificar todos os casos.
- **Iteradores onde deixam o código mais claro:** `windows`, `any`, `filter`, `map` e `collect`.
- **Lista vazia:** tratada de forma explícita na abordagem 1 do Sublist, porque `windows(0)` causa *panic*. Na abordagem 2, `starts_with` com lista vazia já devolve verdadeiro.
- **Sem `unwrap`:** nenhuma das operações pode falhar, então não há `Result` nem `Option`.
- **Perfil de compilação:** os comandos acima usam o perfil `dev` (sem otimização, com verificações de estouro aritmético). Para um executável otimizado, use `--release`.
