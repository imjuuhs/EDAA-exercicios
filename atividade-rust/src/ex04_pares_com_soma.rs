// Exercício 4 — Pares com Soma
// Complexidade: O(n²) — dois loops aninhados comparando todos os pares possíveis.

pub fn pares_com_soma(lista: &[i32], alvo: i32) -> Vec<(i32, i32)> {
    let n = lista.len();
    let mut pares: Vec<(i32, i32)> = Vec::new();

    for i in 0..n {
        for j in (i + 1)..n {
            if lista[i] + lista[j] == alvo {
                println!("{} + {} = {}", lista[i], lista[j], alvo);
                pares.push((lista[i], lista[j]));
            }
        }
    }

    pares
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encontra_par() {
        let resultado = pares_com_soma(&[1, 2, 3, 4], 5);
        assert!(resultado.contains(&(1, 4)));
        assert!(resultado.contains(&(2, 3)));
    }

    #[test]
    fn nenhum_par() {
        let resultado = pares_com_soma(&[1, 2, 3], 10);
        assert!(resultado.is_empty());
    }

    #[test]
    fn lista_vazia() {
        let resultado = pares_com_soma(&[], 5);
        assert!(resultado.is_empty());
    }
}
