# EDAA — Estruturas de Dados e Análise de Algoritmos

**Estudante:** Júlia Starling N. Fudoli  
**RA:** 124222027  
**Professor:** Alexandre de Oliveira  
**Disciplina:** Estruturas de Dados e Análise de Algoritmos

---

## Conteúdo do Repositório

### 1. Atividade — Reescrita de Algoritmos em Rust ([`atividade-rust/`](atividade-rust/))

Implementação em Rust de 10 algoritmos com análise de complexidade Big-O.

| Arquivo | Algoritmo | Complexidade |
|---|---|---|
| `ex01_verificar_primeiro.rs` | Verificar Primeiro | O(1) |
| `ex02_somar_lista.rs` | Somar Lista | O(n) |
| `ex03_busca_binaria.rs` | Busca Binária | O(log n) |
| `ex04_pares_com_soma.rs` | Pares com Soma | O(n²) |
| `ex05_imprimir_pares_e_pares.rs` | Imprimir Pares e Pares | O(n²) |
| `ex06_potencias_de_dois.rs` | Potências de Dois | O(log n) |
| `ex07_fibonacci_recursivo.rs` | Fibonacci Recursivo | O(2ⁿ) |
| `ex08_ordenacao_bolha.rs` | Ordenação Bolha (Bubble Sort) | O(n²) |
| `ex09_produto_de_matrizes.rs` | Produto de Matrizes | O(n³) |
| `ex10_merge_sort.rs` | Merge Sort | O(n log n) |

**Como executar:**
```bash
cd atividade-rust
cargo run        # executa todos os exemplos
cargo test       # roda os 34 testes automatizados
```

Evidências de execução disponíveis em [`atividade-rust/prints_execucao/`](atividade-rust/prints_execucao/).  
Análise de complexidade detalhada em [`atividade-rust/README.md`](atividade-rust/README.md).

---

### 2. Texto Dissertativo — Algoritmos, Dados e Estruturas de Dados ([`texto1_algoritmos.md`](texto1_algoritmos.md))

Texto dissertativo abordando:
- O que é um algoritmo e exemplos práticos
- Diferença entre dado, informação e estrutura de dados
- Situações reais onde a organização dos dados impacta o desempenho

---

### 3. Resenha Crítica — Podcast "C Versus Python no Aprendizado de Algoritmos" ([`resenha_podcast.md`](resenha_podcast.md))

Resenha crítica do podcast *DeDebate* sobre a escolha de linguagem no ensino de algoritmos, analisando os argumentos apresentados em favor de C e Python e desenvolvendo uma perspectiva crítica própria.

---

### 4. Tutorial — Algoritmos de Busca em Rust ([`tutorial-busca/`](tutorial-busca/))

Implementação do experimento prático da Aula 02: comparação entre busca sequencial simples e busca sequencial com interrupção antecipada, com medição de operações e tempo de execução.

**Como executar:**
```bash
cd tutorial-busca
cargo run --release   # executa os experimentos comparativos
cargo test            # roda os 3 testes automatizados
```
