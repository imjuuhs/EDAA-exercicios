// Exercício 5 — Imprimir Pares e Pares
// Complexidade: O(n²) — pela regra da soma: O(n) + O(n²) = O(n²).
// O bloco dominante é o loop aninhado do Bloco 2.

pub fn imprimir_pares_e_pares(lista: &[i32]) {
    // Bloco 1 — O(n): imprime cada elemento individualmente
    for &x in lista {
        println!("{}", x);
    }

    // Bloco 2 — O(n²): imprime todos os pares (i, j), inclusive (i, i)
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
        // Verifica que a função roda sem erros para diferentes tamanhos
        imprimir_pares_e_pares(&[]);
        imprimir_pares_e_pares(&[1]);
        imprimir_pares_e_pares(&[1, 2, 3]);
    }
}
