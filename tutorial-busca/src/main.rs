use std::time::Instant;

// Busca sequencial simples — percorre todo o vetor, nunca interrompe antecipadamente.
// Complexidade: O(n) constante — sempre executa n comparações.
fn busca_sequencial_simples(vetor: &[i32], alvo: i32) -> (Option<usize>, usize) {
    let mut operacoes = 0;
    let mut resultado = None;

    for i in 0..vetor.len() {
        operacoes += 1;
        if vetor[i] == alvo {
            resultado = Some(i);
            // continua mesmo após encontrar, para ilustrar ineficiência
        }
    }

    (resultado, operacoes)
}

// Busca sequencial com interrupção antecipada — para assim que encontra o elemento.
// Complexidade: O(n) no pior caso, O(1) no melhor caso.
fn busca_sequencial_interrompida(vetor: &[i32], alvo: i32) -> (Option<usize>, usize) {
    let mut operacoes = 0;

    for i in 0..vetor.len() {
        operacoes += 1;
        if vetor[i] == alvo {
            return (Some(i), operacoes);
        }
    }

    (None, operacoes)
}

fn gerar_vetor(tamanho: usize) -> Vec<i32> {
    (1..=tamanho as i32).collect()
}

fn executar_experimento(tamanho: usize, alvo: i32) {
    let vetor = gerar_vetor(tamanho);

    println!("\n{}", "=".repeat(60));
    println!("Tamanho do vetor: {}", tamanho);
    println!("Elemento procurado: {}", alvo);
    println!("{}", "-".repeat(60));

    let inicio = Instant::now();
    let (resultado1, ops1) = busca_sequencial_simples(&vetor, alvo);
    let tempo1 = inicio.elapsed();

    println!("\nBUSCA SEQUENCIAL SIMPLES:");
    println!("  Resultado: {:?}", resultado1);
    println!("  Operacoes: {}", ops1);
    println!("  Tempo: {:?}", tempo1);

    let inicio = Instant::now();
    let (resultado2, ops2) = busca_sequencial_interrompida(&vetor, alvo);
    let tempo2 = inicio.elapsed();

    println!("\nBUSCA SEQUENCIAL COM INTERRUPCAO:");
    println!("  Resultado: {:?}", resultado2);
    println!("  Operacoes: {}", ops2);
    println!("  Tempo: {:?}", tempo2);

    println!("\nANALISE COMPARATIVA:");
    println!("  Diferenca de operacoes: {}", ops1.saturating_sub(ops2));
    if tempo1 > tempo2 {
        println!("  Algoritmo com interrupcao foi mais rapido");
    } else if tempo2 > tempo1 {
        println!("  Algoritmo simples foi mais rapido (variacao de medicao)");
    } else {
        println!("  Tempos praticamente iguais");
    }
    println!("{}", "=".repeat(60));
}

fn experimento_buscas_multiplas(tamanho: usize, num_buscas: usize) {
    let vetor = gerar_vetor(tamanho);

    let inicio = Instant::now();
    for _ in 0..num_buscas {
        let _ = busca_sequencial_simples(&vetor, 5);
    }
    let tempo_simples = inicio.elapsed();

    let inicio = Instant::now();
    for _ in 0..num_buscas {
        let _ = busca_sequencial_interrompida(&vetor, 5);
    }
    let tempo_interrupcao = inicio.elapsed();

    println!("\n{} buscas em vetor de tamanho {}:", num_buscas, tamanho);
    println!("  Simples:    {:?}", tempo_simples);
    println!("  Interrupcao: {:?}", tempo_interrupcao);
}

fn main() {
    println!("EXPERIMENTO: COMPARACAO DE ALGORITMOS DE BUSCA\n");

    println!("\nCENARIO 1: Elemento no inicio (melhor caso para interrupcao)");
    executar_experimento(1_000, 5);
    executar_experimento(100_000, 5);
    executar_experimento(1_000_000, 5);

    println!("\n\nCENARIO 2: Elemento no meio do vetor");
    executar_experimento(1_000, 500);
    executar_experimento(100_000, 50_000);
    executar_experimento(1_000_000, 500_000);

    println!("\n\nCENARIO 3: Elemento no final (pior caso)");
    executar_experimento(1_000, 1_000);
    executar_experimento(100_000, 100_000);
    executar_experimento(1_000_000, 1_000_000);

    println!("\n\nCENARIO 4: Elemento nao existe no vetor");
    executar_experimento(1_000, -1);
    executar_experimento(100_000, -1);
    executar_experimento(1_000_000, -1);

    println!("\n\nEXPERIMENTO: 1000 BUSCAS MULTIPLAS");
    experimento_buscas_multiplas(100_000, 1_000);

    println!("\n\nExperimento concluido!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simples_encontra_elemento() {
        let vetor = vec![1, 2, 3, 4, 5];
        let (resultado, ops) = busca_sequencial_simples(&vetor, 3);
        assert_eq!(resultado, Some(2));
        assert_eq!(ops, 5); // percorre tudo sempre
    }

    #[test]
    fn interrompida_para_cedo() {
        let vetor = vec![1, 2, 3, 4, 5];
        let (resultado, ops) = busca_sequencial_interrompida(&vetor, 3);
        assert_eq!(resultado, Some(2));
        assert_eq!(ops, 3); // para no índice 2
    }

    #[test]
    fn elemento_nao_encontrado() {
        let vetor = vec![1, 2, 3];
        let (resultado1, _) = busca_sequencial_simples(&vetor, 99);
        let (resultado2, _) = busca_sequencial_interrompida(&vetor, 99);
        assert_eq!(resultado1, None);
        assert_eq!(resultado2, None);
    }
}
