// Exercício 6 — Potências de Dois
// Complexidade: O(log n) — i dobra a cada iteração, então são log₂(n) passos até i >= n.

pub fn potencias_de_dois(n: u64) -> Vec<u64> {
    let mut potencias: Vec<u64> = Vec::new();
    let mut i: u64 = 1;

    while i < n {
        println!("{}", i);
        potencias.push(i);
        i *= 2;
    }

    potencias
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn potencias_ate_16() {
        let resultado = potencias_de_dois(16);
        assert_eq!(resultado, vec![1, 2, 4, 8]);
    }

    #[test]
    fn potencias_ate_1() {
        let resultado = potencias_de_dois(1);
        assert!(resultado.is_empty());
    }

    #[test]
    fn potencias_ate_100() {
        let resultado = potencias_de_dois(100);
        assert_eq!(resultado, vec![1, 2, 4, 8, 16, 32, 64]);
    }
}
