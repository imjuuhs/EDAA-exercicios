# Atividade — Reescrita de Algoritmos em Rust

**Estudante:** Júlia Starling Negrini Fudoli — RA 124222027  
**Disciplina:** Estruturas de Dados e Análise de Algoritmos

## Como rodar

```bash
cargo run    # executa os exemplos de todos os exercícios
cargo test   # roda todos os testes
```

---

## Análise de Complexidade

### Exercício 1 — Verificar Primeiro

**Complexidade:** O(1)

Acessa direto o primeiro elemento sem percorrer nada. Não importa se a lista tem 10 ou 10 milhões de elementos, a operação é sempre a mesma.

---

### Exercício 2 — Somar Lista

**Complexidade:** O(n)

Um loop que passa por cada elemento uma vez. Se dobrar o tamanho da lista, dobra o tempo.

---

### Exercício 3 — Busca Binária

**Complexidade:** O(log n)

A cada iteração descartamos metade dos elementos restantes. Para uma lista de 1 milhão de elementos, no pior caso fazemos ~20 comparações. Muito mais eficiente que busca linear, mas exige que a lista esteja ordenada.

---

### Exercício 4 — Pares com Soma

**Complexidade:** O(n²)

Dois loops aninhados: o externo vai de 0 até n, o interno vai de i+1 até n. Para n=1000, fazemos ~500.000 comparações.

---

### Exercício 5 — Imprimir Pares e Pares

**Complexidade:** O(n²)

Tem dois blocos: o primeiro é O(n) e o segundo é O(n²). Pela regra da soma, o que domina é o maior, então O(n) + O(n²) = O(n²). A diferença desse exercício pro 4 é que aqui os dois loops começam do zero (gerando inclusive pares (i, i)), enquanto no 4 o segundo loop começa em i+1.

---

### Exercício 6 — Potências de Dois

**Complexidade:** O(log n)

O valor de `i` dobra a cada iteração (1 → 2 → 4 → 8 → ...), então o número de iterações é log₂(n). Para n=1024, são apenas 10 iterações.

---

### Exercício 7 — Fibonacci Recursivo

**Complexidade:** O(2^n)

Cada chamada faz duas chamadas recursivas. Para calcular fib(5) precisamos calcular fib(4) e fib(3), e fib(4) calcula fib(3) de novo. Fica um desperdício enorme conforme n cresce. Não testar com n > 40.

---

### Exercício 8 — Ordenação Bolha (Bubble Sort)

**Complexidade:** O(n²)

O loop externo roda n vezes, o interno vai até n-i-1. A cada passagem completa o maior elemento "borbulha" para o final. Total de comparações ≈ n²/2, que é O(n²).

---

### Exercício 9 — Produto de Matrizes

**Complexidade:** O(n³)

Três loops aninhados, cada um de 0 a n. Para matrizes 10x10 são 1.000 multiplicações, para 100x100 são 1.000.000. Cresce muito rápido.

---

### Exercício 10 — Merge Sort

**Complexidade:** O(n log n)

Divide a lista recursivamente (log n níveis) e na fusão percorre todos os n elementos em cada nível. É muito mais eficiente que bubble sort para listas grandes. Para n=1.000.000: bubble sort faz ~10¹² operações, merge sort faz ~20.000.000.
