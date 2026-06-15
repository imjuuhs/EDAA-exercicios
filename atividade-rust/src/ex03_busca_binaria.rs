// Exercício 3 — Busca Binária
//
// Python original:
//   def busca_binaria(lista, alvo):
//       esquerda, direita = 0, len(lista) - 1
//       while esquerda <= direita:
//           meio = (esquerda + direita) // 2
//           if lista[meio] == alvo:
//               return meio
//           elif lista[meio] < alvo:
//               esquerda = meio + 1
//           else:
//               direita = meio - 1
//       return -1
//
// Complexidade: O(log n) — a cada iteração descartamos metade dos elementos.
// Para n = 1.000.000, precisamos de no máximo ~20 iterações.
//
// ATENÇÃO: a lista precisa estar ordenada para funcionar corretamente.

// Retornamos Option<usize> em vez de -1 porque usize (tipo de índice em Rust)
// não pode ser negativo. Some(idx) quando encontrar, None quando não encontrar.
pub fn busca_binaria(lista: &[i32], alvo: i32) -> Option<usize> {
    // Usamos isize (com sinal) porque direita pode chegar a -1
    // quando o alvo não existe e a busca esgota o espaço.
    // Se usássemos usize, o underflow causaria panic em modo debug.
    let mut esquerda: isize = 0;
    let mut direita: isize = lista.len() as isize - 1;

    while esquerda <= direita {
        let meio = (esquerda + direita) / 2;
        let idx = meio as usize; // converte para acessar o slice (slice usa usize)

        if lista[idx] == alvo {
            return Some(idx); // encontrou!
        } else if lista[idx] < alvo {
            esquerda = meio + 1; // alvo está na metade direita
        } else {
            direita = meio - 1; // alvo está na metade esquerda
        }
    }

    None // não encontrou
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
