# Atividade — Reescrita de Algoritmos em Rust

**Disciplina:** Estruturas de Dados e Análise de Algoritmos  
**Estudante:** Júlia Starling Negrini Fudoli — RA 124222027  
**Professor:** Alexandre de Oliveira

## Como executar

```bash
# Compilar e rodar todos os exemplos
cargo run

# Rodar todos os testes
cargo test
```

---

## Exercício 1 — Verificar Primeiro

**Complexidade:** O(1)

**Lógica do algoritmo:**
Retorna o primeiro elemento de uma lista, se ela não estiver vazia. Não há iteração — o acesso é direto ao índice zero.

**Justificativa da complexidade:**
Independentemente do tamanho da lista, a operação é sempre a mesma: verificar se está vazia e acessar `lista[0]`. O número de operações não cresce com `n`, portanto é O(1).

---

## Exercício 2 — Somar Lista

**Complexidade:** O(n)

**Lógica do algoritmo:**
Percorre cada elemento da lista uma vez, acumulando a soma em uma variável `total`, que é retornada ao final.

**Justificativa da complexidade:**
Há um único loop que visita cada um dos `n` elementos exatamente uma vez. O tempo de execução cresce linearmente com o tamanho da entrada: O(n).

---

## Exercício 3 — Busca Binária

**Complexidade:** O(log n)

**Lógica do algoritmo:**
Divide o intervalo de busca ao meio a cada iteração, descartando metade dos elementos restantes até encontrar o alvo ou esgotar a lista. Requer que a lista esteja ordenada.

**Justificativa da complexidade:**
A cada passo o espaço de busca é cortado pela metade: n → n/2 → n/4 → ... → 1. São necessários log₂(n) passos para reduzir o espaço a 1 elemento. Para n = 1.000.000, isso representa apenas ~20 iterações.

---

## Exercício 4 — Pares com Soma

**Complexidade:** O(n²)

**Lógica do algoritmo:**
Usa dois loops aninhados para comparar todos os pares possíveis de elementos da lista, imprimindo (e retornando) aqueles cuja soma é igual ao alvo.

**Justificativa da complexidade:**
O loop externo executa n vezes. Para cada iteração, o loop interno executa até n-1 vezes. O total de comparações é proporcional a n×(n-1)/2 ≈ n², portanto O(n²).

---

## Exercício 5 — Imprimir Pares e Pares

**Complexidade:** O(n²)

**Lógica do algoritmo:**
Possui dois blocos sequenciais: o Bloco 1 imprime cada elemento individualmente (O(n)); o Bloco 2 imprime todos os pares ordenados (i, j), inclusive (i, i), com dois loops aninhados (O(n²)).

**Justificativa da complexidade:**
Pela regra da soma: O(n) + O(n²) = O(n²). O termo dominante é o loop aninhado do Bloco 2, pois cresce mais rápido que o loop simples do Bloco 1.

---

## Exercício 6 — Potências de Dois

**Complexidade:** O(log n)

**Lógica do algoritmo:**
Inicia com `i = 1` e duplica o valor a cada iteração enquanto `i < n`, imprimindo as potências de 2 encontradas.

**Justificativa da complexidade:**
A variável `i` dobra a cada passo: 1, 2, 4, 8, ..., chegando a `n` após log₂(n) iterações. O número de repetições é logarítmico em relação a `n`.

---

## Exercício 7 — Fibonacci Recursivo

**Complexidade:** O(2ⁿ)

**Lógica do algoritmo:**
Calcula o n-ésimo número de Fibonacci recursivamente, com dois casos base (n=0 e n=1) e dois chamadas recursivas para os casos n-1 e n-2.

**Justificativa da complexidade:**
Cada chamada gera duas novas chamadas recursivas, formando uma árvore binária de profundidade n. O número total de nós nessa árvore é aproximadamente 2ⁿ. Isso torna o algoritmo extremamente ineficiente para valores grandes de n — não testar com n > 40.

---

## Exercício 8 — Ordenação Bolha (Bubble Sort)

**Complexidade:** O(n²)

**Lógica do algoritmo:**
Percorre repetidamente a lista comparando elementos adjacentes e trocando-os quando estão fora de ordem. A cada passagem completa, o maior elemento não ordenado "borbulha" para sua posição final.

**Justificativa da complexidade:**
O loop externo executa n vezes; o loop interno executa n-i-1 vezes em cada passagem. O total de comparações é n(n-1)/2 ≈ n², portanto O(n²).

---

## Exercício 9 — Produto de Matrizes

**Complexidade:** O(n³)

**Lógica do algoritmo:**
Calcula o produto entre duas matrizes n×n usando três loops aninhados: o par (i, j) define a célula de saída C[i][j], e o loop interno k acumula o produto escalar da linha i de A com a coluna j de B.

**Justificativa da complexidade:**
Três loops aninhados, cada um executando n iterações. O número total de multiplicações é n × n × n = n³, resultando em O(n³).

---

## Exercício 10 — Merge Sort

**Complexidade:** O(n log n)

**Lógica do algoritmo:**
Divide recursivamente a lista ao meio até ter sublistas de tamanho 1, depois as funde em ordem crescente. A fusão (merge) intercala dois vetores ordenados em um único vetor ordenado.

**Justificativa da complexidade:**
A divisão recursiva gera log₂(n) níveis de profundidade. Em cada nível, a etapa de fusão percorre todos os n elementos. O custo total é n × log₂(n) = O(n log n), o que é assintoticamente ótimo para algoritmos de ordenação por comparação.
