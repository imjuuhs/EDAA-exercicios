// Exercício 8 — Ordenação Bolha (Bubble Sort)
// Complexidade: O(n²) — dois loops aninhados, o externo roda n vezes e o interno até n-i-1.

pub fn ordenacao_bolha(lista: &mut [i32]) {
    let n = lista.len();

    for i in 0..n {
        for j in 0..(n - i - 1) {
            if lista[j] > lista[j + 1] {
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
