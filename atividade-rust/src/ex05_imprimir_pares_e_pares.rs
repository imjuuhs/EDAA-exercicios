// Exercício 5 — Imprimir Pares e Pares
//
// Python original:
//   def imprimir_pares_e_soma(lista):
//       # Bloco 1: imprime cada elemento
//       for i in range(len(lista)):
//           print(lista[i])
//       # Bloco 2: imprime todos os pares
//       for i in range(len(lista)):
//           for j in range(len(lista)):
//               print(lista[i], lista[j])
//
// Complexidade: O(n²)
// Pela regra da soma: O(n) + O(n²) = O(n²)
// O bloco 2 domina porque cresce mais rápido que o bloco 1.
//
// Diferença do exercício 4: aqui j começa em 0 (igual ao i),
// então geramos TODOS os pares incluindo (i, i) e (i, j) e (j, i).

pub fn imprimir_pares_e_pares(lista: &[i32]) {
    // Bloco 1 — O(n): imprime cada elemento individualmente
    for &x in lista {
        println!("{}", x);
    }

    // Bloco 2 — O(n²): imprime todos os pares ordenados (x, y)
    // Note que ambos os loops percorrem a lista inteira do zero,
    // então (1,2) e (2,1) e (1,1) são todos gerados.
    for &x in lista {
        for &y in lista {
            println!("({}, {})", x, y);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn executa_sem_panic() {
        // verifica que a função não quebra para diferentes tamanhos de entrada
        imprimir_pares_e_pares(&[]);
        imprimir_pares_e_pares(&[1]);
        imprimir_pares_e_pares(&[1, 2, 3]);
    }
}
