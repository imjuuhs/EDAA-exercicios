# Texto 1 — Pesquisa: Algoritmos, Dados e Estruturas de Dados

**Disciplina:** Estruturas de Dados e Análise de Algoritmos  
**Estudante:** Júlia Starling Negrini Fudoli — RA 124222027

---

## O que é um algoritmo? Exemplos práticos.

De forma bem direta, um algoritmo é uma sequência de passos ordenados e bem definidos que resolve um problema. Para que algo seja considerado um algoritmo, ele precisa ter um início e um fim claros, as instruções precisam ser precisas (sem ambiguidade) e ele deve produzir algum resultado a partir de uma entrada.

O que muita gente não percebe é que algoritmos existem fora da computação também. Uma receita de bolo é um algoritmo: você tem ingredientes como entrada, uma lista de passos e o bolo como saída. As instruções de montagem de um móvel da IKEA também seguem essa mesma lógica. A diferença é que, no computador, o algoritmo precisa ser descrito de forma tão precisa que uma máquina consiga executar sem "interpretar" nada — ela segue exatamente o que está escrito.

Na computação, alguns algoritmos clássicos que estudamos nessa disciplina são:

- **Busca binária:** dado um vetor ordenado, divide o espaço de busca ao meio a cada passo até encontrar o elemento (ou confirmar que ele não existe). É bem mais eficiente que percorrer o vetor inteiro do começo ao fim.
- **Bubble Sort e Merge Sort:** algoritmos de ordenação. O Bubble Sort compara elementos vizinhos e vai "empurrando" o maior para o final. O Merge Sort divide a lista em partes menores, ordena cada parte e depois junta tudo — mais complexo de implementar, mas muito mais eficiente.
- **Fibonacci recursivo:** calcula o n-ésimo número da sequência chamando a si mesmo para os dois anteriores. É um ótimo exemplo de como recursão funciona, mas é bem ineficiente porque recalcula os mesmos valores várias vezes.

O ponto principal é que algoritmos são a essência da resolução de problemas na computação. Antes de escrever qualquer código, precisamos ter uma ideia clara dos passos que vão transformar a entrada na saída desejada.

---

## Diferença entre dado, informação e estrutura de dados.

Esses três termos às vezes são usados como sinônimos no dia a dia, mas na computação eles têm significados bem diferentes.

**Dado** é um valor bruto, sem contexto. O número `37.5`, a string `"Maria"` e o booleano `false` são dados. Sozinhos, eles não dizem muita coisa — o `37.5` pode ser uma temperatura, um preço, uma nota.

**Informação** é o dado quando recebe contexto e significado. Se eu sei que `37.5` é a temperatura corporal da paciente `"Maria"` medida hoje, agora tenho informação. Essa informação pode ser usada para tomar uma decisão (está com febre ou não?). A transformação de dado para informação acontece quando relacionamos os dados entre si e com o contexto.

**Estrutura de dados** é a forma como organizamos os dados na memória do computador para poder acessá-los e manipulá-los de forma eficiente. A escolha da estrutura certa impacta diretamente na complexidade dos algoritmos que trabalham com esses dados. As principais que já vimos são:

- **Array (vetor):** elementos armazenados em sequência na memória. Acesso por índice é O(1), mas inserir no meio é O(n) porque precisa deslocar os elementos.
- **Lista encadeada:** cada elemento tem um ponteiro pro próximo. Boa para inserção e remoção, mas para acessar o k-ésimo elemento é preciso percorrer tudo do início: O(n).
- **Pilha (stack):** funciona no modelo LIFO (Last In, First Out). É como uma pilha de pratos — você sempre pega o do topo. Útil para controle de chamadas de funções e desfazer/refazer operações.
- **Fila (queue):** funciona no modelo FIFO (First In, First Out). O primeiro que entrou é o primeiro que sai. Usado em filas de processamento de tarefas.
- **Tabela hash:** mapeia uma chave para um valor usando uma função de hash. Busca em O(1) no caso médio, mas pode degradar se muitas colisões acontecerem.

Resumindo: dado é o valor bruto, informação é o dado com contexto, e estrutura de dados é a forma como organizamos esses dados para conseguir trabalhar com eles eficientemente.

---

## Situações reais onde a organização dos dados impacta o desempenho.

A escolha de como organizar os dados pode fazer uma diferença enorme no desempenho de um sistema. Vou dar alguns exemplos que achei interessantes.

**Banco de dados e índices.** Imagina um sistema de uma rede de hospitais com milhões de cadastros de pacientes. Se os registros estiverem numa lista sem ordenação, buscar pelo CPF de um paciente requer percorrer todos os registros: O(n). Com um índice (que internamente usa uma árvore de busca), a mesma operação cai para O(log n). Para 10 milhões de registros, a diferença é entre 10 milhões de comparações e uns 23. Isso parece pequeno, mas multiplicado por milhares de buscas simultâneas, é a diferença entre o sistema funcionar ou travar.

**GPS e grafos.** Aplicativos como o Google Maps representam o mapa como um grafo, onde as ruas são arestas e as esquinas são vértices. O algoritmo de Dijkstra percorre esse grafo para encontrar o caminho mais curto. Se os dados não estivessem organizados como grafo, calcular rotas em tempo real seria inviável. A estrutura escolhida viabilizou o produto inteiro.

**Sistemas de busca na internet.** O Google precisa retornar resultados em menos de um segundo para buscas em bilhões de páginas. Isso só é possível porque os dados são organizados em índices invertidos: uma estrutura que mapeia cada palavra para a lista de páginas que a contém. Sem essa organização, uma busca simples exigiria varrer todos os documentos da web a cada consulta.

Esses exemplos mostram que a escolha da estrutura de dados não é só um detalhe técnico — ela define se o sistema vai funcionar ou não dentro de um tempo aceitável. É por isso que estudar as complexidades de cada estrutura é tão importante: para saber qual usar em cada situação.
