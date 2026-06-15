// Exercício 10 — Merge Sort
//
// Python original:
//   def merge_sort(lista):
//       if len(lista) <= 1:
//           return lista
//       meio = len(lista) // 2
//       esquerda = merge_sort(lista[:meio])
//       direita = merge_sort(lista[meio:])
//       resultado = []
//       i = j = 0
//       while i < len(esquerda) and j < len(direita):
//           if esquerda[i] <= direita[j]:
//               resultado.append(esquerda[i]); i += 1
//           else:
//               resultado.append(direita[j]); j += 1
//       resultado += esquerda[i:]
//       resultado += direita[j:]
//       return resultado
//
// Complexidade: O(n log n) — a divisão cria log n níveis de recursão,
// e a fusão (merge) em cada nível percorre n elementos no total.

// Recebemos Vec<i32> com posse porque o merge sort cria novos vetores
// a cada chamada (não modifica no lugar como o bubble sort).
// Se usássemos &[i32], precisaríamos copiar de qualquer forma.
pub fn merge_sort(lista: Vec<i32>) -> Vec<i32> {
    // caso base: lista com 0 ou 1 elemento já está ordenada
    if lista.len() <= 1 {
        return lista;
    }

    let meio = lista.len() / 2;

    // lista[..meio].to_vec() cria um novo Vec copiando a primeira metade
    // lista[meio..].to_vec() cria um novo Vec copiando a segunda metade
    let esquerda = merge_sort(lista[..meio].to_vec());
    let direita = merge_sort(lista[meio..].to_vec());

    // junta os dois vetores já ordenados
    merge(esquerda, direita)
}

// função auxiliar privada que intercala dois vetores ordenados
fn merge(esquerda: Vec<i32>, direita: Vec<i32>) -> Vec<i32> {
    // with_capacity evita realocações durante o push
    let mut resultado = Vec::with_capacity(esquerda.len() + direita.len());
    let mut i = 0; // índice da esquerda
    let mut j = 0; // índice da direita

    // enquanto ambos os lados têm elementos, pega o menor
    while i < esquerda.len() && j < direita.len() {
        if esquerda[i] <= direita[j] {
            resultado.push(esquerda[i]);
            i += 1;
        } else {
            resultado.push(direita[j]);
            j += 1;
        }
    }

    // um dos lados ainda pode ter elementos restantes — adiciona tudo
    // equivalente ao "resultado += esquerda[i:]" do Python
    resultado.extend_from_slice(&esquerda[i..]);
    resultado.extend_from_slice(&direita[j..]);

    resultado
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
