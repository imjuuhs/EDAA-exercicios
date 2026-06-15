// Exercício 10 — Merge Sort
// Complexidade: O(n log n) — divide a lista em log n níveis, cada nível fazendo O(n) trabalho na fusão.

fn merge(esquerda: Vec<i32>, direita: Vec<i32>) -> Vec<i32> {
    let mut resultado = Vec::with_capacity(esquerda.len() + direita.len());
    let mut i = 0;
    let mut j = 0;

    while i < esquerda.len() && j < direita.len() {
        if esquerda[i] <= direita[j] {
            resultado.push(esquerda[i]);
            i += 1;
        } else {
            resultado.push(direita[j]);
            j += 1;
        }
    }

    // Adiciona os elementos restantes
    resultado.extend_from_slice(&esquerda[i..]);
    resultado.extend_from_slice(&direita[j..]);
    resultado
}

pub fn merge_sort(lista: Vec<i32>) -> Vec<i32> {
    if lista.len() <= 1 {
        return lista; // caso base — devolve a posse
    }

    let meio = lista.len() / 2;
    let esquerda = merge_sort(lista[..meio].to_vec());
    let direita = merge_sort(lista[meio..].to_vec());

    merge(esquerda, direita)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ordena_lista_desordenada() {
        let lista = vec![5, 3, 1, 4, 2];
        assert_eq!(merge_sort(lista), vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn lista_ja_ordenada() {
        let lista = vec![1, 2, 3, 4, 5];
        assert_eq!(merge_sort(lista.clone()), lista);
    }

    #[test]
    fn lista_vazia() {
        let lista: Vec<i32> = vec![];
        assert_eq!(merge_sort(lista), vec![]);
    }

    #[test]
    fn lista_com_um_elemento() {
        assert_eq!(merge_sort(vec![42]), vec![42]);
    }

    #[test]
    fn lista_com_duplicatas() {
        let lista = vec![3, 1, 2, 1, 3];
        assert_eq!(merge_sort(lista), vec![1, 1, 2, 3, 3]);
    }
}
