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
| `src/lib.rs` | Raiz da biblioteca. Declara as duas abordagens e **escolhe qual delas está ativa** para os testes. |
| `src/abordagem1.rs` | Primeira solução do exercício. |
| `src/abordagem2.rs` | Segunda solução do exercício. |
| `tests/*.rs` | Testes de integração: usam a biblioteca de fora, como outro programa usaria. |
| `examples/demo.rs` | Programa de demonstração que executa **as duas abordagens lado a lado** e compara os resultados. |
| `target/` | Pasta criada automaticamente pelo Cargo com os arquivos compilados. Não faz parte do código-fonte. |

> **Importante:** o nome dos projetos dentro do `Cargo.toml` é `raindrops` e `sublist`. Os testes importam o código por esses nomes (`use raindrops::...` e `use sublist::...`), então não os altere. O nome das *pastas* pode ser qualquer um.

### Como o `lib.rs` alterna entre as abordagens

As duas abordagens são sempre compiladas (`pub mod`), e uma linha `pub use` decide qual função é exportada com o nome que os testes esperam:

```rust
pub mod abordagem1;
pub mod abordagem2;

// Abordagem 1 ativa:
pub use abordagem1::raindrops;

// Para usar a abordagem 2, comente a linha acima e ative esta:
// pub use abordagem2::raindrops;
```

No projeto Sublist, o `lib.rs` também define o `enum Comparison`, usado pelas duas abordagens.

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
cd "C:\caminho\para\P2 - 6°\gotas de chuva"
```

### 3.1 Rodar os testes

```
cargo test
```

O Cargo compila o projeto e executa todos os testes. O resultado esperado termina com:

```
test result: ok. 18 passed; 0 failed; 0 ignored; ...
```

> As linhas `running 0 tests` do bloco `unittests src\lib.rs` e dos `Doc-tests` são normais. Elas aparecem porque não há testes dentro do `src/` nem na documentação. Os testes de verdade estão em `tests/`.

### 3.2 Rodar a demonstração

```
cargo run --example demo
```

O programa imprime, para vários casos de entrada, o resultado das **duas abordagens** e se elas concordam (`OK`) ou não (`DIFERENTE`).

> Como os projetos são bibliotecas (sem `main`), o `cargo run` sozinho não funciona. Use sempre `cargo run --example demo`.

Exemplo de saída no projeto Raindrops:

```
n      | abordagem 1      | abordagem 2      | iguais?
-------------------------------------------------------
3      | Pling            | Pling            | OK
30     | PlingPlang       | PlingPlang       | OK
34     | 34               | 34               | OK
105    | PlingPlangPlong  | PlingPlangPlong  | OK
```

### 3.3 Alternar entre as abordagens nos testes

O `cargo test` testa a abordagem que estiver ativa no `src/lib.rs`. Para testar a outra:

1. Abra `src/lib.rs`.
2. Comente a linha `pub use abordagemX::...` ativa (colocando `//` na frente) e descomente a outra.
3. Salve o arquivo e rode `cargo test` novamente.

(O `cargo run --example demo` não depende dessa troca, pois sempre chama as duas abordagens diretamente.)

### 3.4 Formatar e analisar o código

```
cargo fmt
cargo clippy
```

- `cargo fmt` formata o código no padrão oficial do Rust.
- `cargo clippy` aponta construções não idiomáticas e possíveis melhorias.

### 3.5 O que acontece ao compilar

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

- `let mut resultado` cria uma `String` vazia e mutável. Em Rust as variáveis são imutáveis por padrão, e o `mut` é necessário porque o texto será modificado.
- Os três `if` são **independentes** (não são `else if`), porque mais de uma condição pode ser verdadeira ao mesmo tempo. É isso que faz o 30 receber "Pling" e "Plang".
- `n % 3 == 0` testa se o resto da divisão é zero, ou seja, se `n` é divisível por 3.
- No final, `is_empty()` indica que nenhum `if` foi atendido. Nesse caso, `n.to_string()` converte o número em texto.
- O último `if/else` não tem ponto e vírgula: ele é a **expressão final** da função e seu valor é o retorno, sem precisar de `return`.

### Abordagem 2: funcional, orientada a dados (`src/abordagem2.rs`)

```rust
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
```

O resultado é um `enum` com quatro variantes. `Debug` permite imprimir o valor e `PartialEq` permite compará-lo com `==` (necessário para o `assert_eq!` dos testes). Usar um enum faz o compilador garantir que só existem esses quatro resultados.

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

Cada projeto tem **18 testes** em `tests/`:

- **Raindrops:** casos como `1 -> "1"`, `3 -> "Pling"`, `15 -> "PlingPlang"`, `52 -> "52"` e `105 -> "PlingPlangPlong"`. Os testes são gerados por uma macro (`macro_rules!`) que cria uma função `#[test]` para cada caso, comparando o resultado com `assert_eq!`.
- **Sublist:** listas vazias, igualdade, sublista e superlista no início, no meio e no fim, o "falso início" (por exemplo `[1, 2, 5]` dentro de `[0, 1, 2, 3, 1, 2, 5, 6]`) e casos em que a ordem ou os dígitos mudam o resultado.

> **Observação:** estes arquivos de teste foram escritos seguindo o formato e os casos do Exercism, mas **não são os arquivos oficiais** da plataforma. Para validar também com os testes oficiais, é possível colar o conteúdo de `abordagem1.rs` ou `abordagem2.rs` no editor online do Exercism e clicar em **Run Tests** (no Sublist, sem a linha `use crate::Comparison;`, pois lá o enum já vem pronto).

---

## 7. Escolhas idiomáticas em Rust

- **Empréstimo (borrowing):** `&[T]` em vez de `Vec<T>`, sem copiar dados. Nenhum `clone`.
- **Propriedade (ownership):** no Raindrops, a função devolve uma `String` própria, pois ela é criada dentro da função.
- **Iteradores:** `windows`, `any`, `filter`, `map` e `collect` no lugar de laços com índices, onde isso deixa o código mais claro.
- **Genéricos com restrição mínima:** apenas `T: PartialEq`.
- **Enum e `match` exaustivo:** o compilador garante que todos os casos foram tratados.
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

**Erro 4551 / "Uma política de Controle de Aplicativo bloqueou este arquivo" (Windows)**
O Windows às vezes bloqueia por instantes executáveis recém-criados. Rode o comando novamente. Se continuar, mova o projeto para uma pasta simples (por exemplo, `C:\p2`) e execute de lá.

**Caminho com espaços ou caracteres especiais**
Coloque o caminho entre aspas no `cd`. Se ainda houver problemas, copie o projeto para uma pasta sem acentos e sem espaços.

**O rust-analyzer não reconhece o projeto no VS Code**
Abra a pasta do projeto (aquela que contém o `Cargo.toml`) diretamente, em vez de uma pasta superior com vários projetos dentro.
