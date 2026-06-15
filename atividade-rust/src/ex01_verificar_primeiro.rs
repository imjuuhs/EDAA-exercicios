// Exercício 1 — Verificar Primeiro
//
// Python original:
//   def verificar_primeiro(lista):
//       if len(lista) == 0:
//           return None
//       return lista[0]
//
// Complexidade: O(1) — não importa o tamanho da lista, sempre fazemos
// só uma operação: acessar o primeiro elemento.

// &[i32] é uma referência a uma fatia (slice) do vetor.
// Não precisamos tomar posse, só queremos ler.
// Retornamos Option<i32> porque a lista pode estar vazia.
// Em Python usaríamos None, em Rust usamos Option::None.
pub fn verificar_primeiro(lista: &[i32]) -> Option<i32> {
    // .first() já retorna Option<&i32> (referência ao primeiro elemento ou None)
    // .copied() converte Option<&i32> para Option<i32>, copiando o valor
    lista.first().copied()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lista_vazia() {
        // lista vazia deve retornar None
        assert_eq!(verificar_primeiro(&[]), None);
    }

    #[test]
    fn lista_com_elementos() {
        // deve retornar o primeiro elemento, ignorando o resto
        assert_eq!(verificar_primeiro(&[10, 20, 30]), Some(10));
    }

    #[test]
    fn lista_com_um_elemento() {
        assert_eq!(verificar_primeiro(&[42]), Some(42));
    }
}
