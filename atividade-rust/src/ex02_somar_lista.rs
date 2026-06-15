// Exercício 2 — Somar Lista
// Complexidade: O(n) — percorre cada elemento da lista exatamente uma vez.

pub fn somar_lista(lista: &[i32]) -> i32 {
    let mut total = 0;
    for &elemento in lista {
        total += elemento;
    }
    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lista_vazia() {
        assert_eq!(somar_lista(&[]), 0);
    }

    #[test]
    fn soma_positivos() {
        assert_eq!(somar_lista(&[1, 2, 3, 4, 5]), 15);
    }

    #[test]
    fn soma_com_negativos() {
        assert_eq!(somar_lista(&[-1, 2, -3, 4]), 2);
    }
}
