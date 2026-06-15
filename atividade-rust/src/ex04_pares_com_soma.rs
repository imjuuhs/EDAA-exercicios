// Exercício 4 — Pares com Soma
//
// Python original:
//   def pares_com_soma(lista, alvo):
//       for i in range(len(lista)):
//           for j in range(i + 1, len(lista)):
//               if lista[i] + lista[j] == alvo:
//                   print(lista[i], lista[j])
//
// Complexidade: O(n²) — dois loops aninhados. O loop interno começa em i+1
// para não repetir pares e não comparar um elemento com ele mesmo.
// Para n elementos, fazemos n*(n-1)/2 comparações ≈ n².

// Optei por retornar um Vec com os pares em vez de só imprimir,
// o que facilita muito os testes (dica opcional do enunciado).
pub fn pares_com_soma(lista: &[i32], alvo: i32) -> Vec<(i32, i32)> {
    let n = lista.len();
    let mut pares: Vec<(i32, i32)> = Vec::new();

    for i in 0..n {
        // j começa em i+1 para não repetir pares (ex: (1,2) e (2,1))
        for j in (i + 1)..n {
            if lista[i] + lista[j] == alvo {
                println!("{} + {} = {}", lista[i], lista[j], alvo);
                pares.push((lista[i], lista[j]));
            }
        }
    }

    pares
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encontra_par() {
        let resultado = pares_com_soma(&[1, 2, 3, 4], 5);
        assert!(resultado.contains(&(1, 4)));
        assert!(resultado.contains(&(2, 3)));
    }

    #[test]
    fn nenhum_par() {
        let resultado = pares_com_soma(&[1, 2, 3], 10);
        assert!(resultado.is_empty());
    }

    #[test]
    fn lista_vazia() {
        let resultado = pares_com_soma(&[], 5);
        assert!(resultado.is_empty());
    }
}
