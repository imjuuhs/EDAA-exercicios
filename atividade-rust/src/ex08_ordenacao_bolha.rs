// Exercício 8 — Ordenação Bolha (Bubble Sort)
//
// Python original:
//   def ordenacao_bolha(lista):
//       n = len(lista)
//       for i in range(n):
//           for j in range(0, n - i - 1):
//               if lista[j] > lista[j + 1]:
//                   lista[j], lista[j + 1] = lista[j + 1], lista[j]
//       return lista
//
// Complexidade: O(n²) — dois loops aninhados. A cada passagem do loop externo,
// o maior elemento "borbulha" para sua posição final, então o loop interno
// vai ficando menor (n-i-1), mas no total ainda fazemos ≈ n²/2 comparações.

// &mut [i32] — precisamos de referência mutável porque vamos modificar o slice
// (trocar elementos de lugar). A função não retorna nada (modifica no local).
pub fn ordenacao_bolha(lista: &mut [i32]) {
    let n = lista.len();

    for i in 0..n {
        // a cada passagem, os últimos i elementos já estão no lugar certo,
        // então o loop interno vai até n-i-1
        for j in 0..(n - i - 1) {
            if lista[j] > lista[j + 1] {
                // Em Python faríamos: lista[j], lista[j+1] = lista[j+1], lista[j]
                // Em Rust, .swap() faz isso de forma idiomática e segura.
                // Não podemos fazer a troca manualmente com dois empréstimos
                // porque o compilador não deixa dois &mut do mesmo slice ao mesmo tempo.
                lista.swap(j, j + 1);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ordena_lista_desordenada() {
        let mut lista = [5, 3, 1, 4, 2];
        ordenacao_bolha(&mut lista);
        assert_eq!(lista, [1, 2, 3, 4, 5]);
    }

    #[test]
    fn lista_ja_ordenada() {
        let mut lista = [1, 2, 3, 4, 5];
        ordenacao_bolha(&mut lista);
        assert_eq!(lista, [1, 2, 3, 4, 5]);
    }

    #[test]
    fn lista_invertida() {
        let mut lista = [5, 4, 3, 2, 1];
        ordenacao_bolha(&mut lista);
        assert_eq!(lista, [1, 2, 3, 4, 5]);
    }

    #[test]
    fn lista_com_um_elemento() {
        let mut lista = [42];
        ordenacao_bolha(&mut lista);
        assert_eq!(lista, [42]);
    }

    #[test]
    fn lista_vazia() {
        let mut lista: [i32; 0] = [];
        ordenacao_bolha(&mut lista);
        assert_eq!(lista, []);
    }
}
