// Exercício 2 — Somar Lista
//
// Python original:
//   def somar_lista(lista):
//       total = 0
//       for elemento in lista:
//           total += elemento
//       return total
//
// Complexidade: O(n) — percorremos cada elemento uma vez,
// então o tempo cresce linearmente com o tamanho da lista.

// &[i32] — passamos por referência imutável porque a função só lê os dados
pub fn somar_lista(lista: &[i32]) -> i32 {
    // mut porque vamos modificar total dentro do loop
    let mut total = 0;

    // "for &elemento in lista" desestrutura a referência automaticamente,
    // então elemento já é i32 (e não &i32).
    // Sem o &, elemento seria do tipo &i32 e precisaríamos desreferenciar.
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
        // soma de lista vazia deve ser 0
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
