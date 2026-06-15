// Exercício 1 — Verificar Primeiro
// Complexidade: O(1) — acesso direto ao primeiro elemento, independente do tamanho da lista.

pub fn verificar_primeiro(lista: &[i32]) -> Option<i32> {
    lista.first().copied()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lista_vazia() {
        assert_eq!(verificar_primeiro(&[]), None);
    }

    #[test]
    fn lista_com_elementos() {
        assert_eq!(verificar_primeiro(&[10, 20, 30]), Some(10));
    }

    #[test]
    fn lista_com_um_elemento() {
        assert_eq!(verificar_primeiro(&[42]), Some(42));
    }
}
