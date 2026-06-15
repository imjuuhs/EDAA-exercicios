// Exercício 7 — Fibonacci Recursivo
//
// Python original:
//   def fibonacci_recursivo(n):
//       if n <= 1:
//           return n
//       return fibonacci_recursivo(n - 1) + fibonacci_recursivo(n - 2)
//
// Complexidade: O(2^n) — cada chamada gera duas chamadas recursivas,
// formando uma árvore binária de chamadas. Para fib(5), por exemplo,
// fib(3) é calculado duas vezes, fib(2) três vezes, etc.
//
// NÃO testar com valores acima de ~40, o tempo de execução explode.

// u64 porque os valores de Fibonacci crescem muito rápido:
// fib(93) já ultrapassa o limite do u64, mas para os testes está bom.
pub fn fibonacci_recursivo(n: u64) -> u64 {
    // caso base: fib(0) = 0 e fib(1) = 1
    // sem isso, a recursão nunca termina
    if n <= 1 {
        return n;
    }

    // chamada recursiva para os dois anteriores
    // a soma deles dá o próximo número da sequência
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
        // sequência: 0, 1, 1, 2, 3, 5, 8, 13, 21, 34
        assert_eq!(fibonacci_recursivo(2), 1);
        assert_eq!(fibonacci_recursivo(5), 5);
        assert_eq!(fibonacci_recursivo(9), 34);
    }

    #[test]
    fn fib_10() {
        assert_eq!(fibonacci_recursivo(10), 55);
    }
}
