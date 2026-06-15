mod ex01_verificar_primeiro;
mod ex02_somar_lista;
mod ex03_busca_binaria;
mod ex04_pares_com_soma;
mod ex05_imprimir_pares_e_pares;
mod ex06_potencias_de_dois;
mod ex07_fibonacci_recursivo;
mod ex08_ordenacao_bolha;
mod ex09_produto_de_matrizes;
mod ex10_merge_sort;

use ex01_verificar_primeiro::verificar_primeiro;
use ex02_somar_lista::somar_lista;
use ex03_busca_binaria::busca_binaria;
use ex04_pares_com_soma::pares_com_soma;
use ex05_imprimir_pares_e_pares::imprimir_pares_e_pares;
use ex06_potencias_de_dois::potencias_de_dois;
use ex07_fibonacci_recursivo::fibonacci_recursivo;
use ex08_ordenacao_bolha::ordenacao_bolha;
use ex09_produto_de_matrizes::produto_de_matrizes;
use ex10_merge_sort::merge_sort;

fn separador(titulo: &str) {
    println!("\n{}", "=".repeat(50));
    println!("  {}", titulo);
    println!("{}", "=".repeat(50));
}

fn main() {
    separador("Ex 1 — Verificar Primeiro  [O(1)]");
    println!("Lista vazia:      {:?}", verificar_primeiro(&[]));
    println!("Lista [10,20,30]: {:?}", verificar_primeiro(&[10, 20, 30]));

    separador("Ex 2 — Somar Lista  [O(n)]");
    println!("Soma [1,2,3,4,5]: {}", somar_lista(&[1, 2, 3, 4, 5]));

    separador("Ex 3 — Busca Binária  [O(log n)]");
    let lista_ord = [1, 3, 5, 7, 9, 11];
    println!("Buscar 7 em {:?}: índice {:?}", lista_ord, busca_binaria(&lista_ord, 7));
    println!("Buscar 6 em {:?}: {:?}", lista_ord, busca_binaria(&lista_ord, 6));

    separador("Ex 4 — Pares com Soma  [O(n²)]");
    println!("Pares de [1,2,3,4,5] que somam 6:");
    pares_com_soma(&[1, 2, 3, 4, 5], 6);

    separador("Ex 5 — Imprimir Pares e Pares  [O(n²)]");
    println!("Lista [1, 2, 3]:");
    imprimir_pares_e_pares(&[1, 2, 3]);

    separador("Ex 6 — Potências de Dois  [O(log n)]");
    println!("Potências de 2 menores que 100:");
    potencias_de_dois(100);

    separador("Ex 7 — Fibonacci Recursivo  [O(2^n)]");
    for n in [0, 1, 5, 10] {
        println!("fib({}) = {}", n, fibonacci_recursivo(n));
    }

    separador("Ex 8 — Ordenação Bolha  [O(n²)]");
    let mut lista = [5, 3, 1, 4, 2];
    println!("Antes:  {:?}", lista);
    ordenacao_bolha(&mut lista);
    println!("Depois: {:?}", lista);

    separador("Ex 9 — Produto de Matrizes  [O(n³)]");
    let a = vec![vec![1i64, 2], vec![3, 4]];
    let b = vec![vec![5i64, 6], vec![7, 8]];
    let c = produto_de_matrizes(&a, &b);
    println!("A = {:?}", a);
    println!("B = {:?}", b);
    println!("A × B = {:?}", c);

    separador("Ex 10 — Merge Sort  [O(n log n)]");
    let lista = vec![38, 27, 43, 3, 9, 82, 10];
    println!("Antes:  {:?}", lista);
    let ordenada = merge_sort(lista);
    println!("Depois: {:?}", ordenada);
}
