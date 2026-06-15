// Exercício 9 — Produto de Matrizes
//
// Python original:
//   def produto_de_matrizes(A, B, n):
//       C = [[0] * n for _ in range(n)]
//       for i in range(n):
//           for j in range(n):
//               for k in range(n):
//                   C[i][j] += A[i][k] * B[k][j]
//       return C
//
// Complexidade: O(n³) — três loops aninhados, cada um percorrendo n elementos.
// Para matrizes 100x100, são 1.000.000 de multiplicações.

// Em Rust, representamos matrizes como Vec<Vec<T>> (vetor de vetores).
// Usamos i64 para evitar overflow nas multiplicações.
pub fn produto_de_matrizes(a: &[Vec<i64>], b: &[Vec<i64>]) -> Vec<Vec<i64>> {
    let n = a.len();

    // cria a matriz C com n linhas e n colunas, tudo zerado
    // vec![valor; tamanho] é a forma idiomática de inicializar vetores em Rust
    let mut c = vec![vec![0i64; n]; n];

    for i in 0..n {
        for j in 0..n {
            for k in 0..n {
                // C[i][j] acumula a soma dos produtos da linha i de A pela coluna j de B
                c[i][j] += a[i][k] * b[k][j];
            }
        }
    }

    // retornamos c com posse (ownership), o chamador recebe o vetor
    c
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn produto_identidade() {
        // A * I = A (I é a matriz identidade)
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
        // linha 0: [1*5+2*7, 1*6+2*8] = [19, 22]
        // linha 1: [3*5+4*7, 3*6+4*8] = [43, 50]
        assert_eq!(resultado, vec![vec![19, 22], vec![43, 50]]);
    }

    #[test]
    fn produto_zeros() {
        // A * zeros = zeros
        let zeros = vec![vec![0, 0], vec![0, 0]];
        let a = vec![vec![1, 2], vec![3, 4]];
        let resultado = produto_de_matrizes(&a, &zeros);
        assert_eq!(resultado, zeros);
    }
}
