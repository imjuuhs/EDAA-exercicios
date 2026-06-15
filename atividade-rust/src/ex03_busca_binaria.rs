// Exercício 3 — Busca Binária
// Complexidade: O(log n) — a cada iteração o espaço de busca é dividido ao meio.
// A lista deve estar ordenada em ordem crescente.

pub fn busca_binaria(lista: &[i32], alvo: i32) -> Option<usize> {
    let mut esquerda: isize = 0;
    let mut direita: isize = lista.len() as isize - 1;

    while esquerda <= direita {
        let meio = (esquerda + direita) / 2;
        let idx = meio as usize;

        if lista[idx] == alvo {
            return Some(idx);
        } else if lista[idx] < alvo {
            esquerda = meio + 1;
        } else {
            direita = meio - 1;
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encontra_elemento_no_meio() {
        let lista = [1, 3, 5, 7, 9];
        assert_eq!(busca_binaria(&lista, 5), Some(2));
    }

    #[test]
    fn encontra_elemento_no_inicio() {
        let lista = [1, 3, 5, 7, 9];
        assert_eq!(busca_binaria(&lista, 1), Some(0));
    }

    #[test]
    fn encontra_elemento_no_fim() {
        let lista = [1, 3, 5, 7, 9];
        assert_eq!(busca_binaria(&lista, 9), Some(4));
    }

    #[test]
    fn elemento_nao_encontrado() {
        let lista = [1, 3, 5, 7, 9];
        assert_eq!(busca_binaria(&lista, 4), None);
    }

    #[test]
    fn lista_vazia() {
        assert_eq!(busca_binaria(&[], 1), None);
    }
}
