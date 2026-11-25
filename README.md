# P2: Técnicas de Programação em Rust

Avaliação Prática 2 (P2) da disciplina **Linguagens e Técnicas de Programação / Técnicas de Programação**, professor Bruno Gabriel.

**Dupla:** Rafael e Mahgid

Este repositório contém a resolução de dois exercícios da trilha de Rust do [Exercism](https://exercism.org/), cada um com **duas abordagens algoritmicamente distintas**, totalizando **quatro soluções**:

| Exercício | Nível | Abordagem 1 | Abordagem 2 |
|---|---|---|---|
| **Raindrops** (Gotas de Chuva) | Fácil | Imperativa, com `if` e `String` mutável | Funcional, com tabela de dados e iteradores |
| **Sublist** (Sublista) | Médio | Janelas deslizantes (`windows`) e iteradores | Comparação de tamanhos com `match` e busca manual com `for` |

---

## Sumário

1. [Estrutura do projeto](#1-estrutura-do-projeto)
2. [Instalando o Rust](#2-instalando-o-rust)
3. [Como executar o código](#3-como-executar-o-código)
4. [Exercício 1: Raindrops](#4-exercício-1-raindrops)
5. [Exercício 2: Sublist](#5-exercício-2-sublist)
6. [Sobre os testes](#6-sobre-os-testes)
7. [Escolhas idiomáticas em Rust](#7-escolhas-idiomáticas-em-rust)
8. [Solução de problemas](#8-solução-de-problemas)

---

## 1. Estrutura do projeto

Cada exercício é um projeto independente do **Cargo**, o gerenciador de projetos e compilador do Rust. Ambos foram criados como **bibliotecas** (`cargo new --lib`), ou seja, têm funções reutilizáveis, mas não têm uma função `main`.

```
P2 - 6°/
├── README.md
├── gotas de chuva/              <- projeto "raindrops"
│   ├── Cargo.toml
│   ├── src/
│   │   ├── lib.rs
│   │   ├── abordagem1.rs
│   │   └── abordagem2.rs
│   ├── tests/
│   │   └── raindrops.rs
│   └── examples/
│       └── demo.rs
└── sublista/                    <- projeto "sublist"
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

O que cada arquivo faz:

| Arquivo | Função |
|---|---|
| `Cargo.toml` | Configuração do projeto: nome, versão e edição da linguagem (`2021`). Não há dependências externas. |
| `src/lib.rs` | Raiz da biblioteca. Declara as duas abordagens e **escolhe qual delas está ativa** (para os testes e para o demo). |
| `src/abordagem1.rs` | Primeira solução do exercício, com a constante `NOME` que a identifica. |
| `src/abordagem2.rs` | Segunda solução do exercício, com a constante `NOME` que a identifica. |
| `tests/*.rs` | Testes de integração: usam a biblioteca de fora, como outro programa usaria. |
| `examples/demo.rs` | Programa de demonstração: imprime **qual abordagem está ativa** e os resultados para vários casos de entrada. |
| `target/` | Pasta criada automaticamente pelo Cargo com os arquivos compilados. Não faz parte do código-fonte. |

> **Importante:** o nome dos projetos dentro do `Cargo.toml` é `raindrops` e `sublist`. Os testes e o demo importam o código por esses nomes (`use raindrops::...` e `use sublist::...`), então não os altere. O nome das *pastas* pode ser qualquer um.

### Como o `lib.rs` escolhe a abordagem ativa

As duas abordagens são sempre compiladas (`pub mod`), e uma linha `pub use` decide qual delas é exportada com o nome que os testes e o demo esperam. A linha exporta **a função e a constante `NOME`** juntas, então ao trocar de abordagem o nome mostrado pelo demo troca junto. Exemplo do Raindrops:

```rust
pub mod abordagem1;
pub mod abordagem2;

// Para testar a abordagem 1, deixe esta linha ativa:
pub use abordagem1::{raindrops, NOME};

// Para testar a abordagem 2, comente a linha acima e ative esta:
// pub use abordagem2::{raindrops, NOME};
```

Deixe **apenas uma** das duas linhas `pub use` sem o `//`. No projeto Sublist, o `lib.rs` também define o `enum Comparison`, usado pelas duas abordagens.

---

## 2. Instalando o Rust

O Rust é instalado pelo **rustup**, o instalador oficial. Ele instala o compilador (`rustc`), o gerenciador de projetos (`cargo`) e as ferramentas `rustfmt` e `clippy`.

### 2.1 Windows (passo a passo)

1. **Baixe o instalador.** Acesse <https://rustup.rs> e baixe o `rustup-init.exe` (64 bits, na maioria dos computadores).
2. **Execute o `rustup-init.exe`.** Uma janela de terminal será aberta.
3. **Ferramentas de build do C++.** No Windows, o Rust precisa do *linker* da Microsoft. Se o instalador avisar que o **Visual Studio C++ Build Tools** não foi encontrado, aceite a instalação sugerida, ou instale manualmente:
   - Baixe o **Build Tools for Visual Studio** em <https://visualstudio.microsoft.com/visual-cpp-build-tools/>.
   - No instalador, marque a carga de trabalho **"Desenvolvimento para desktop com C++"** e instale.
   - Depois, rode o `rustup-init.exe` de novo.
4. **Escolha a instalação padrão.** Quando aparecer o menu, digite `1` (*Proceed with standard installation*) e pressione Enter. Aguarde o fim do download e da instalação.
5. **Feche e reabra o terminal** (e o VS Code, se estiver aberto). Isso é necessário para o sistema reconhecer o novo comando `cargo`.
6. **Confirme a instalação** abrindo um terminal (PowerShell) e digitando:
   ```
   rustc --version
   cargo --version
   ```
   Cada comando deve mostrar um número de versão. Se aparecer "comando não encontrado", veja a seção [Solução de problemas](#8-solução-de-problemas).

### 2.2 Linux e macOS

1. Abra um terminal e execute:
   ```
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```
2. Escolha a opção `1` (instalação padrão).
3. Feche e reabra o terminal, ou execute `source "$HOME/.cargo/env"`.
4. Confirme com `rustc --version` e `cargo --version`.

No Linux, pode ser necessário ter um compilador C instalado (por exemplo, `sudo apt install build-essential` no Ubuntu/Debian). No macOS, instale as ferramentas de linha de comando com `xcode-select --install`.

### 2.3 Ferramentas de formatação e análise

Normalmente o rustup já instala o `rustfmt` e o `clippy`. Para garantir:

```
rustup component add rustfmt clippy
```

### 2.4 Configurando o VS Code (opcional, recomendado)

1. Instale o [Visual Studio Code](https://code.visualstudio.com/).
2. Abra a aba de extensões (`Ctrl+Shift+X`) e instale a extensão **rust-analyzer**.
3. Abra a pasta de **um projeto por vez** (por exemplo, `gotas de chuva`) em **Arquivo > Abrir Pasta**. O rust-analyzer só reconhece o projeto quando o `Cargo.toml` está na raiz da pasta aberta.
4. Use o terminal integrado do VS Code (`` Ctrl+' ``), que já abre na pasta certa.

---

## 3. Como executar o código

Todos os comandos abaixo são executados **dentro da pasta de um projeto** (onde está o `Cargo.toml`). Se o caminho tiver espaços, use aspas:

```
cd "C:\caminho\para\gotas de chuva"
```

### 3.1 Escolher a abordagem

Antes de rodar, abra `src/lib.rs` e deixe ativa a linha `pub use` da abordagem desejada (a outra fica comentada com `//`). Tanto o `cargo test` quanto o demo usam a abordagem escolhida ali.

### 3.2 Rodar a demonstração

```
cargo run --example demo
```

O programa imprime **qual abordagem está ativa** e, em seguida, o resultado para vários casos de entrada. A saída é a mesma nas duas abordagens, porque ambas resolvem o mesmo problema. O que muda é a primeira linha, que identifica a abordagem.

Exemplo no projeto Raindrops, com a abordagem 2 ativa:

```
Abordagem ativa: Abordagem 2 (funcional, com tabela e iteradores)

n      | resultado
------------------------------
1      | 1
3      | Pling
5      | Plang
7      | Plong
8      | 8
15     | PlingPlang
21     | PlingPlong
28     | Plong
30     | PlingPlang
34     | 34
35     | PlangPlong
105    | PlingPlangPlong
```

Exemplo no projeto Sublist, com a abordagem 1 ativa:

```
Abordagem ativa: Abordagem 1 (windows e iteradores)

A = []   B = []   ->   Equal
A = [1, 2, 3]   B = []   ->   Superlist
A = []   B = [1, 2, 3]   ->   Sublist
A = [1, 2, 3]   B = [1, 2, 3, 4, 5]   ->   Sublist
A = [3, 4]   B = [1, 2, 3, 4, 5]   ->   Sublist
A = [1, 2, 3, 4, 5]   B = [2, 3, 4]   ->   Superlist
A = [1, 2, 4]   B = [1, 2, 3, 4, 5]   ->   Unequal
A = [1, 2, 3]   B = [1, 3, 2]   ->   Unequal
```

> Como os projetos são bibliotecas (sem `main`), o `cargo run` sozinho não funciona. Use sempre `cargo run --example demo`.

O código do demo do Raindrops (`examples/demo.rs`) é:

```rust
use raindrops::{raindrops, NOME};

fn main() {
    println!("Abordagem ativa: {}", NOME);
    println!();
    println!("{:<6} | {}", "n", "resultado");
    println!("{}", "-".repeat(30));

    for n in [1, 3, 5, 7, 8, 15, 21, 28, 30, 34, 35, 105] {
        println!("{:<6} | {}", n, raindrops(n));
    }
}
```

E o do Sublist:

```rust
use sublist::{sublist, NOME};

fn main() {
    println!("Abordagem ativa: {}", NOME);
    println!();

    let casos: [(&[i32], &[i32]); 8] = [
        (&[], &[]),
        (&[1, 2, 3], &[]),
        (&[], &[1, 2, 3]),
        (&[1, 2, 3], &[1, 2, 3, 4, 5]),
        (&[3, 4], &[1, 2, 3, 4, 5]),
        (&[1, 2, 3, 4, 5], &[2, 3, 4]),
        (&[1, 2, 4], &[1, 2, 3, 4, 5]),
        (&[1, 2, 3], &[1, 3, 2]),
    ];

    for (a, b) in casos {
        println!("A = {:?}   B = {:?}   ->   {:?}", a, b, sublist(a, b));
    }
}
```

Ambos importam do `lib.rs` apenas a função e o `NOME` da abordagem ativa. É por isso que o demo mostra uma abordagem de cada vez.

### 3.3 Rodar os testes

```
cargo test
```

O Cargo compila o projeto e executa todos os testes da abordagem ativa. O resultado esperado termina com:

```
test result: ok. 18 passed; 0 failed; 0 ignored; ...
```

> As linhas `running 0 tests` do bloco `unittests src\lib.rs` e dos `Doc-tests` são normais. Elas aparecem porque não há testes dentro do `src/` nem na documentação. Os testes de verdade estão em `tests/`.

### 3.4 Passo a passo para validar as quatro soluções

Em cada projeto, repita estes passos para cada abordagem:

1. Abra `src/lib.rs` e ative a linha `pub use` da abordagem (a outra fica comentada). Salve com `Ctrl+S`.
2. Rode `cargo run --example demo` e confira a linha `Abordagem ativa: ...` e os resultados.
3. Rode `cargo test` e confira `18 passed`.
4. Troque a linha do `lib.rs` para a outra abordagem e repita os passos 2 e 3.

### 3.5 Formatar e analisar o código

```
cargo fmt
cargo clippy
```

- `cargo fmt` formata o código no padrão oficial do Rust.
- `cargo clippy` aponta construções não idiomáticas e possíveis melhorias.

### 3.6 O que acontece ao compilar

O Rust é uma linguagem **compilada antes da execução**: o compilador `rustc` (chamado pelo Cargo) traduz o código para um executável nativo, sem máquina virtual e sem coletor de lixo. Os comandos acima usam o perfil `dev`, que não otimiza o código e inclui informações de depuração (aparece como `unoptimized + debuginfo`). Para uma versão otimizada, usa-se `--release`.

Uma característica central do compilador é verificar **propriedade (ownership) e empréstimo (borrowing)** em tempo de compilação: referências inválidas simplesmente não compilam. Por isso todas as soluções recebem os dados emprestados, sem copiá-los.

---

## 4. Exercício 1: Raindrops

### O problema

Converter um número em "sons de gotas de chuva":

| Se o número for divisível por... | Adiciona ao resultado |
|---|---|
| 3 | `Pling` |
| 5 | `Plang` |
| 7 | `Plong` |
| nenhum dos três | o próprio número, como texto |

Os sons se acumulam: `30` é divisível por 3 e por 5, então vira `"PlingPlang"`. Já `34` não é divisível por nenhum, então vira `"34"`.

**Assinatura:** `pub fn raindrops(n: u32) -> String`

- `u32` é um inteiro sem sinal de 32 bits, adequado porque o problema trata de números não negativos.
- O retorno é `String` (e não `&str`) porque o texto é construído dentro da função. Não é possível devolver uma referência a um dado local, então a função devolve a posse do valor.

### Abordagem 1: imperativa (`src/abordagem1.rs`)

```rust
/// Nome mostrado pelo demo para indicar qual abordagem está ativa.
pub const NOME: &str = "Abordagem 1 (imperativa, com if e String mutável)";

/// Abordagem 1: imperativa, com `if` independentes e `String` mutável.
pub fn raindrops(n: u32) -> String {
    let mut resultado = String::new();

    if n % 3 == 0 {
        resultado.push_str("Pling");
    }
    if n % 5 == 0 {
        resultado.push_str("Plang");
    }
    if n % 7 == 0 {
        resultado.push_str("Plong");
    }

    if resultado.is_empty() {
        n.to_string()
    } else {
        resultado
    }
}
```

**Como funciona:**

- `NOME` é uma constante de texto que identifica a abordagem. É usada apenas pelo demo e não afeta a lógica.
- `let mut resultado` cria uma `String` vazia e mutável. Em Rust as variáveis são imutáveis por padrão, e o `mut` é necessário porque o texto será modificado.
- Os três `if` são **independentes** (não são `else if`), porque mais de uma condição pode ser verdadeira ao mesmo tempo. É isso que faz o 30 receber "Pling" e "Plang".
- `n % 3 == 0` testa se o resto da divisão é zero, ou seja, se `n` é divisível por 3.
- No final, `is_empty()` indica que nenhum `if` foi atendido. Nesse caso, `n.to_string()` converte o número em texto.
- O último `if/else` não tem ponto e vírgula: ele é a **expressão final** da função e seu valor é o retorno, sem precisar de `return`.

### Abordagem 2: funcional, orientada a dados (`src/abordagem2.rs`)

```rust
/// Nome mostrado pelo demo para indicar qual abordagem está ativa.
pub const NOME: &str = "Abordagem 2 (funcional, com tabela e iteradores)";

/// Abordagem 2: funcional e orientada a dados (tabela + iteradores).
const SONS: [(u32, &str); 3] = [(3, "Pling"), (5, "Plang"), (7, "Plong")];

pub fn raindrops(n: u32) -> String {
    let resultado: String = SONS
        .iter()
        .filter(|&&(divisor, _)| n % divisor == 0)
        .map(|&(_, som)| som)
        .collect();

    if resultado.is_empty() {
        n.to_string()
    } else {
        resultado
    }
}
```

**Como funciona:**

- `SONS` é uma constante com as regras: uma lista de pares `(divisor, som)`. As regras viram **dados**, e não código.
- `iter()` percorre o array, entregando uma referência a cada par.
- `filter` mantém só os pares cujo divisor divide `n`. O padrão `|&&(divisor, _)|` desmonta as duas camadas de referência e extrai o divisor (possível porque `u32` é copiável).
- `map` transforma cada par restante apenas no seu som, com o padrão `|&(_, som)|`.
- `collect` junta todos os sons em uma `String`. O tipo do resultado é definido pela anotação `: String`.
- Iteradores em Rust são **preguiçosos**: nada é calculado até o `collect` pedir. E, depois de otimizados pelo compilador, não custam mais que um laço escrito à mão.

### Diferença entre as abordagens

| | Abordagem 1 | Abordagem 2 |
|---|---|---|
| Estilo | Imperativo, com estado mutável | Funcional e declarativo |
| Regras | Espalhadas em três `if` | Concentradas em uma tabela |
| `mut` na lógica | Sim | Não |
| Adicionar uma regra (ex.: 11) | Novo `if` | Uma linha na tabela |

---

## 5. Exercício 2: Sublist

### O problema

Dadas duas listas, `A` e `B`, determinar se:

- `A` é **igual** a `B` (mesmos valores, na mesma ordem);
- `A` é **superlista** de `B` (`A` contém `B` como uma subsequência **contígua**);
- `A` é **sublista** de `B` (`B` contém `A` como uma subsequência contígua);
- ou nenhuma das opções, e as listas são **diferentes**.

Exemplos:

| A | B | Resultado |
|---|---|---|
| `[]` | `[]` | Equal |
| `[1, 2, 3]` | `[]` | Superlist |
| `[]` | `[1, 2, 3]` | Sublist |
| `[3, 4]` | `[1, 2, 3, 4, 5]` | Sublist |
| `[1, 2, 3, 4, 5]` | `[2, 3, 4]` | Superlist |
| `[1, 2, 4]` | `[1, 2, 3, 4, 5]` | Unequal (o 3 foi pulado) |
| `[1, 2, 3]` | `[1, 3, 2]` | Unequal (a ordem importa) |

A lista vazia está contida em qualquer lista.

### Tipos e assinatura (`src/lib.rs`)

```rust
#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub mod abordagem1;
pub mod abordagem2;

// Para testar a abordagem 1, deixe esta linha ativa:
pub use abordagem1::{sublist, NOME};

// Para testar a abordagem 2, comente a linha acima e ative esta:
// pub use abordagem2::{sublist, NOME};
```

O resultado é um `enum` com quatro variantes. `Debug` permite imprimir o valor (usado pelo demo com `{:?}`) e `PartialEq` permite compará-lo com `==` (necessário para o `assert_eq!` dos testes). Usar um enum faz o compilador garantir que só existem esses quatro resultados.

A assinatura das duas abordagens é:

```rust
pub fn sublist<T: PartialEq>(primeira: &[T], segunda: &[T]) -> Comparison
```

- `&[T]` é uma *slice*: uma visão emprestada de uma sequência. A função não copia nem toma posse das listas e aceita arrays, vetores e pedaços de listas.
- `T` é um tipo **genérico**, então a função funciona para números, textos ou qualquer tipo comparável. O compilador gera uma versão para cada tipo usado, sem custo em tempo de execução.
- `T: PartialEq` é a restrição mínima necessária, já que a única operação sobre os elementos é `==`.

### Abordagem 1: janelas deslizantes e iteradores (`src/abordagem1.rs`)

```rust
use crate::Comparison;

/// Nome mostrado pelo demo para indicar qual abordagem está ativa.
pub const NOME: &str = "Abordagem 1 (windows e iteradores)";

/// Abordagem 1: janelas deslizantes (`windows`) e iteradores,
/// testando as duas direções.
pub fn sublist<T: PartialEq>(primeira: &[T], segunda: &[T]) -> Comparison {
    if primeira == segunda {
        Comparison::Equal
    } else if contem(primeira, segunda) {
        Comparison::Superlist
    } else if contem(segunda, primeira) {
        Comparison::Sublist
    } else {
        Comparison::Unequal
    }
}

fn contem<T: PartialEq>(maior: &[T], menor: &[T]) -> bool {
    // windows(0) causa panic, então a lista vazia é tratada à parte
    menor.is_empty() || maior.windows(menor.len()).any(|janela| janela == menor)
}
```

**Como funciona:**

- A **ordem** das verificações importa. A igualdade vem primeiro porque, se viesse depois, duas listas iguais seriam classificadas como superlista (uma lista sempre contém a si mesma).
- `contem` usa `windows(n)`, que gera todas as janelas consecutivas de tamanho `n`. São *slices* que apontam para a lista original, sem cópia. Por exemplo, as janelas de tamanho 2 de `[1, 2, 3, 4, 5]` são `[1,2]`, `[2,3]`, `[3,4]` e `[4,5]`.
- `any` verifica se alguma janela é igual à lista menor e **para na primeira que encontrar**.
- `windows(0)` causa *panic*. Por isso `menor.is_empty() ||` vem antes: o operador `||` só avalia o lado direito se o esquerdo for falso, então a lista vazia nunca chega ao `windows` e é considerada contida em qualquer lista.
- Se a lista menor for maior que a outra, `windows` não gera nenhuma janela e o resultado é `false`, como esperado.
- Esta abordagem testa **as duas direções**.

### Abordagem 2: comparação de tamanhos e busca manual (`src/abordagem2.rs`)

```rust
use crate::Comparison;
use std::cmp::Ordering;

/// Nome mostrado pelo demo para indicar qual abordagem está ativa.
pub const NOME: &str = "Abordagem 2 (comparação de tamanhos e busca manual)";

/// Abordagem 2: compara os tamanhos primeiro (`match` em `Ordering`),
/// decide a direção da busca e usa um laço manual com retorno antecipado.
pub fn sublist<T: PartialEq>(primeira: &[T], segunda: &[T]) -> Comparison {
    match primeira.len().cmp(&segunda.len()) {
        Ordering::Equal => {
            if primeira == segunda {
                Comparison::Equal
            } else {
                Comparison::Unequal
            }
        }
        Ordering::Greater => {
            if contem(primeira, segunda) {
                Comparison::Superlist
            } else {
                Comparison::Unequal
            }
        }
        Ordering::Less => {
            if contem(segunda, primeira) {
                Comparison::Sublist
            } else {
                Comparison::Unequal
            }
        }
    }
}

// Pré-condição: maior.len() >= menor.len()
fn contem<T: PartialEq>(maior: &[T], menor: &[T]) -> bool {
    for inicio in 0..=(maior.len() - menor.len()) {
        if maior[inicio..].starts_with(menor) {
            return true;
        }
    }
    false
}
```

**Como funciona:**

- A primeira decisão é pelo **tamanho**: `len().cmp(...)` devolve um `Ordering` (`Less`, `Equal` ou `Greater`) e o `match` trata os três casos. O compilador **obriga** a cobrir todos.
  - Tamanhos iguais: só pode ser igual ou diferente. Não há busca.
  - Primeira maior: só pode ser superlista ou diferente. Testa-se apenas se ela contém a segunda.
  - Primeira menor: só pode ser sublista ou diferente. Testa-se o contrário.
- `contem` é um laço `for` manual com **pré-condição** `maior.len() >= menor.len()` (garantida pelo `match`). Por isso a subtração nunca fica negativa, o que seria um erro com números sem sinal.
- O intervalo `0..=` é **inclusivo**: a última posição inicial possível também precisa ser testada. Com a lista maior de tamanho 5 e a menor de tamanho 3, as posições iniciais são 0, 1 e 2.
- Em cada posição, `maior[inicio..]` pega o restante da lista e `starts_with(menor)` verifica se ele começa com a lista menor. Ao encontrar, `return true` encerra imediatamente (retorno antecipado). Se o laço termina sem achar, o resultado é `false`.
- A lista vazia funciona sem caso especial: `starts_with` com uma lista vazia é verdadeiro já na primeira posição.

### Diferença entre as abordagens

| | Abordagem 1 | Abordagem 2 |
|---|---|---|
| Estilo | Funcional, com iteradores | Imperativo, com laço e retorno antecipado |
| Busca | `windows` + `any` | `for` manual + `starts_with` |
| Direção da busca | Testa as duas direções | Usa os tamanhos para testar apenas uma |
| Lista vazia | Caso especial (`is_empty`) | Funciona naturalmente |
| Complexidade (pior caso) | O(n · m) | O(n · m), evitando buscas desnecessárias |

Um algoritmo como o KMP reduziria a complexidade para O(n + m), mas não era necessário para este exercício.

---

## 6. Sobre os testes

Cada projeto tem **18 testes** em `tests/`, executados contra a abordagem ativa no `lib.rs`:

- **Raindrops:** casos como `1 -> "1"`, `3 -> "Pling"`, `15 -> "PlingPlang"`, `52 -> "52"` e `105 -> "PlingPlangPlong"`. Os testes são gerados por uma macro (`macro_rules!`) que cria uma função `#[test]` para cada caso, comparando o resultado com `assert_eq!`.
- **Sublist:** listas vazias, igualdade, sublista e superlista no início, no meio e no fim, o "falso início" (por exemplo `[1, 2, 5]` dentro de `[0, 1, 2, 3, 1, 2, 5, 6]`) e casos em que a ordem ou os dígitos mudam o resultado.

A adição da constante `NOME` e a mudança do `lib.rs` para exportá-la não alteram os testes: a função continua com o mesmo nome e o mesmo comportamento.

> **Observação:** estes arquivos de teste foram escritos seguindo o formato e os casos do Exercism, mas **não são os arquivos oficiais** da plataforma. Para validar também com os testes oficiais, é possível colar o conteúdo de `abordagem1.rs` ou `abordagem2.rs` no editor online do Exercism e clicar em **Run Tests**. Lá, o `NOME` e o `use crate::Comparison;` não são necessários (no Sublist, o enum já vem pronto no editor).

---

## 7. Escolhas idiomáticas em Rust

- **Empréstimo (borrowing):** `&[T]` em vez de `Vec<T>`, sem copiar dados. Nenhum `clone`.
- **Propriedade (ownership):** no Raindrops, a função devolve uma `String` própria, pois ela é criada dentro da função.
- **Iteradores:** `windows`, `any`, `filter`, `map` e `collect` no lugar de laços com índices, onde isso deixa o código mais claro.
- **Genéricos com restrição mínima:** apenas `T: PartialEq`.
- **Enum e `match` exaustivo:** o compilador garante que todos os casos foram tratados.
- **Constantes:** `const` para a tabela de sons e para o `NOME` de cada abordagem.
- **Expressões em vez de `return`:** o último `if/else` de cada função é o valor de retorno.
- **Sem `unwrap`:** nenhuma operação pode falhar nestes exercícios, então não é necessário `Result` nem `Option`. O tipo `Comparison` já representa todos os resultados possíveis.
- **Formatação e análise:** código formatado com `cargo fmt` e verificado com `cargo clippy`.

---

## 8. Solução de problemas

**`cargo` ou `rustc` não é reconhecido como comando**
Feche e reabra o terminal (e o VS Code) depois de instalar o Rust. Se persistir, verifique se a pasta `.cargo\bin` do seu usuário está no `PATH` ou reinstale pelo `rustup-init.exe`.

**`linker 'link.exe' not found` (Windows)**
Falta o C++ Build Tools. Instale a carga de trabalho **"Desenvolvimento para desktop com C++"** (veja o passo 3 da instalação no Windows).

**O `cargo test` mostra `running 0 tests` e não encontra os testes**
A pasta de testes precisa se chamar exatamente `tests` (minúsculo), estar **ao lado** de `src` e `Cargo.toml` (e não dentro de `src`) e conter o arquivo com a extensão `.rs`. Verifique também se o Windows não está escondendo uma extensão extra (por exemplo, `raindrops.rs.txt`).

**`could not find Cargo.toml`**
O comando está sendo executado fora da pasta do projeto. Use `cd` para entrar na pasta que contém o `Cargo.toml`.

**`error: a bin target must be available for cargo run`**
Os projetos são bibliotecas. Use `cargo run --example demo` em vez de `cargo run`.

**Erro de compilação mencionando `NOME` ou `abordagem1`/`abordagem2` no demo**
O `lib.rs` precisa exportar `NOME` junto com a função (`pub use abordagem1::{raindrops, NOME};`), e cada arquivo de abordagem precisa ter a linha `pub const NOME`. Confira também se só **uma** linha `pub use` está ativa: com as duas ativas, o compilador reclama de nomes duplicados.

**Erro 4551 / "Uma política de Controle de Aplicativo bloqueou este arquivo" (Windows)**
Não é um erro do código: o `Finished` aparece antes da mensagem, então a compilação deu certo. O Windows bloqueia por instantes executáveis recém-criados que ele ainda não reconhece. Tente, nesta ordem:

1. Rodar o mesmo comando de novo.
2. Fechar e reabrir o terminal (ou o VS Code), entrar na pasta do projeto e rodar de novo.
3. Rodar `cargo clean` (apaga só a pasta `target`, que o Cargo recria) e depois `cargo run --example demo`.
4. Mover o projeto para uma pasta simples, sem acentos e sem espaços (por exemplo, `C:\p2`), e rodar de lá.

Se continuar bloqueando, verifique em **Segurança do Windows > Controle de aplicativos e navegador** se o **Smart App Control** está ativo. Desativá-lo é uma decisão sua, pois depois de desligado ele só volta com a reinstalação do Windows. Em computadores administrados pela faculdade ou pelo trabalho, a política pode não poder ser alterada.

**Caminho com espaços ou caracteres especiais**
Coloque o caminho entre aspas no `cd`. Se ainda houver problemas, copie o projeto para uma pasta sem acentos e sem espaços.

**O rust-analyzer não reconhece o projeto no VS Code**
Abra a pasta do projeto (aquela que contém o `Cargo.toml`) diretamente, em vez de uma pasta superior com vários projetos dentro.
