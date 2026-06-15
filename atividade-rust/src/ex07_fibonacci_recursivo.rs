// Exercício 7 — Fibonacci Recursivo
// Complexidade: O(2^n) — cada chamada gera duas chamadas recursivas, formando uma árvore binária.
// Não testar com n > 40, pois o tempo de execução se torna muito longo.

pub fn fibonacci_recursivo(n: u64) -> u64 {
    if n <= 1 {
        return n; // caso base — âncora da recursão
    }
    fibonacci_recursivo(n - 1) + fibonacci_recursivo(n - 2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn casos_base() {
        assert_eq!(fibonacci_recursivo(0), 0);
        assert_eq!(fibonacci_recursivo(1), 1);
    }

    #[test]
    fn sequencia_conhecida() {
        // Sequência: 0, 1, 1, 2, 3, 5, 8, 13, 21, 34
        assert_eq!(fibonacci_recursivo(2), 1);
        assert_eq!(fibonacci_recursivo(5), 5);
        assert_eq!(fibonacci_recursivo(9), 34);
    }

    #[test]
    fn fib_10() {
        assert_eq!(fibonacci_recursivo(10), 55);
    }
}
