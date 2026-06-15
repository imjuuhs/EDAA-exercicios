// Exercício 6 — Potências de Dois
//
// Python original:
//   def potencias_de_dois(n):
//       i = 1
//       while i < n:
//           print(i)
//           i *= 2
//
// Complexidade: O(log n) — i dobra a cada iteração (1, 2, 4, 8, ...),
// então o loop executa log₂(n) vezes até i >= n.

// Usamos u64 porque potências de 2 crescem rápido e
// i32 só vai até ~2 bilhões (2³¹). u64 suporta até ~1,8 × 10¹⁹.
pub fn potencias_de_dois(n: u64) -> Vec<u64> {
    let mut potencias: Vec<u64> = Vec::new();
    let mut i: u64 = 1; // começa em 2⁰ = 1

    while i < n {
        println!("{}", i);
        potencias.push(i);
        i *= 2; // equivalente a i = i * 2
    }

    potencias
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn potencias_ate_16() {
        // 1, 2, 4, 8 são menores que 16; 16 não entra (condição é i < n)
        let resultado = potencias_de_dois(16);
        assert_eq!(resultado, vec![1, 2, 4, 8]);
    }

    #[test]
    fn potencias_ate_1() {
        // i começa em 1, e 1 < 1 é falso, então retorna vazio
        let resultado = potencias_de_dois(1);
        assert!(resultado.is_empty());
    }

    #[test]
    fn potencias_ate_100() {
        let resultado = potencias_de_dois(100);
        assert_eq!(resultado, vec![1, 2, 4, 8, 16, 32, 64]);
    }
}
