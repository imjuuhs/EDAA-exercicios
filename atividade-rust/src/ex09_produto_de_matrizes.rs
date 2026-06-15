// Exercício 9 — Produto de Matrizes
// Complexidade: O(n³) — três loops aninhados, cada um rodando n vezes.

pub fn produto_de_matrizes(a: &[Vec<i64>], b: &[Vec<i64>]) -> Vec<Vec<i64>> {
    let n = a.len();
    let mut c = vec![vec![0i64; n]; n];

    for i in 0..n {
        for j in 0..n {
            for k in 0..n {
                c[i][j] += a[i][k] * b[k][j];
            }
        }
    }

    c
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn produto_identidade() {
        // A * I = A
        let a = vec![vec![1, 2], vec![3, 4]];
        let identidade = vec![vec![1, 0], vec![0, 1]];
        let resultado = produto_de_matrizes(&a, &identidade);
        assert_eq!(resultado, a);
    }

    #[test]
    fn produto_2x2() {
        let a = vec![vec![1, 2], vec![3, 4]];
        let b = vec![vec![5, 6], vec![7, 8]];
        let resultado = produto_de_matrizes(&a, &b);
        // [1*5+2*7, 1*6+2*8] = [19, 22]
        // [3*5+4*7, 3*6+4*8] = [43, 50]
        assert_eq!(resultado, vec![vec![19, 22], vec![43, 50]]);
    }

    #[test]
    fn produto_zeros() {
        let zeros = vec![vec![0, 0], vec![0, 0]];
        let a = vec![vec![1, 2], vec![3, 4]];
        let resultado = produto_de_matrizes(&a, &zeros);
        assert_eq!(resultado, zeros);
    }
}
