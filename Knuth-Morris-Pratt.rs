// main.rs

/// Encontra todas as ocorrências de um padrão (needle) dentro de um texto (haystack)
/// usando o algoritmo Knuth-Morris-Pratt.
///
/// A função é genérica e funciona com qualquer tipo que implemente o trait `PartialEq`.
///
/// # Argumentos
///
/// * `haystack`: A fatia de dados onde a busca será realizada (o "palheiro").
/// * `needle`: A fatia de dados a ser procurada (a "agulha").
///
/// # Retorno
///
/// Retorna um `Vec<usize>` contendo os índices de início de todas as ocorrências
/// do `needle` no `haystack`. Se nenhuma ocorrência for encontrada, retorna um vetor vazio.
pub fn kmp_search<T>(haystack: &[T], needle: &[T]) -> Vec<usize>
where
    T: PartialEq,
{
    // Casos base: se o padrão for vazio ou maior que o texto, não há correspondência.
    if needle.is_empty() || haystack.len() < needle.len() {
        return vec![];
    }

    // 1. Pré-processamento: construir a tabela LPS para o padrão (needle).
    let lps_table = compute_lps_table(needle);
    let mut results = Vec::new();

    let mut i = 0; // índice para o haystack
    let mut j = 0; // índice para o needle

    // 2. Busca: percorrer o haystack usando a tabela LPS para saltos inteligentes.
    while i < haystack.len() {
        if haystack[i] == needle[j] {
            // Os caracteres correspondem, avançamos ambos os ponteiros.
            i += 1;
            j += 1;
        }

        if j == needle.len() {
            // Encontramos uma correspondência completa!
            // O início da correspondência é `i - j`.
            results.push(i - j);
            // Preparamos para a próxima busca usando a tabela LPS para saber onde continuar.
            j = lps_table[j - 1];
        } else if i < haystack.len() && haystack[i] != needle[j] {
            // Os caracteres não correspondem.
            if j != 0 {
                // Usamos a tabela LPS para dar um "salto" inteligente no padrão (needle),
                // evitando retroceder no texto (haystack).
                j = lps_table[j - 1];
            } else {
                // Se `j` já é 0, não há para onde saltar. Apenas avançamos no texto.
                i += 1;
            }
        }
    }

    results
}

/// Função auxiliar para calcular a tabela LPS (Longest Proper Prefix which is also Suffix).
/// Esta tabela é o coração do KMP, permitindo os "saltos" eficientes.
fn compute_lps_table<T>(needle: &[T]) -> Vec<usize>
where
    T: PartialEq,
{
    if needle.is_empty() {
        return vec![];
    }
    
    // A tabela LPS terá o mesmo tamanho do padrão.
    let mut lps = vec![0; needle.len()];
    let mut length = 0; // Comprimento do maior prefixo-sufixo anterior.
    let mut i = 1;

    // O loop calcula lps[i] para i de 1 a n-1. lps[0] é sempre 0.
    while i < needle.len() {
        if needle[i] == needle[length] {
            // Se correspondem, o novo comprimento do prefixo-sufixo é o anterior + 1.
            length += 1;
            lps[i] = length;
            i += 1;
        } else {
            // Se não correspondem...
            if length != 0 {
                // ...voltamos para o comprimento do prefixo-sufixo anterior.
                // Esta é a parte mais engenhosa do algoritmo.
                length = lps[length - 1];
            } else {
                // Se `length` é 0, não há prefixo-sufixo.
                lps[i] = 0;
                i += 1;
            }
        }
    }

    lps
}


// --- Exemplo de Uso ---
fn main() {
    // Exemplo 1: Busca de texto (string)
    let text = "ABABCABABABCD";
    let pattern = "ABABCD";
    
    // Convertendo para vetores de char para usar a função genérica
    let text_chars: Vec<char> = text.chars().collect();
    let pattern_chars: Vec<char> = pattern.chars().collect();

    let matches = kmp_search(&text_chars, &pattern_chars);
    println!("Texto: '{}'", text);
    println!("Padrão: '{}'", pattern);
    println!("Padrão encontrado nos índices: {:?}", matches); // Deve imprimir [7]
    println!("---");

    // Exemplo 2: Múltiplas ocorrências, incluindo sobrepostas
    let text2 = "abababa";
    let pattern2 = "aba";
    let text_chars2: Vec<char> = text2.chars().collect();
    let pattern_chars2: Vec<char> = pattern2.chars().collect();
    
    let matches2 = kmp_search(&text_chars2, &pattern_chars2);
    println!("Texto: '{}'", text2);
    println!("Padrão: '{}'", pattern2);
    println!("Padrão encontrado nos índices: {:?}", matches2); // Deve imprimir [0, 2, 4]
    println!("---");

    // Exemplo 3: Genérico, usando números (u8)
    let sequence: Vec<u8> = vec![1, 2, 3, 1, 2, 4, 5, 1, 2, 3, 1, 2, 3, 5];
    let sub_sequence: Vec<u8> = vec![1, 2, 3, 5];

    let matches3 = kmp_search(&sequence, &sub_sequence);
    println!("Sequência: {:?}", sequence);
    println!("Sub-sequência: {:?}", sub_sequence);
    println!("Sub-sequência encontrada nos índices: {:?}", matches3); // Deve imprimir [9]
    println!("---");
    
    // Exemplo 4: Sem ocorrências
    let text4 = "abcdefg";
    let pattern4 = "xyz";
    let text_chars4: Vec<char> = text4.chars().collect();
    let pattern_chars4: Vec<char> = pattern4.chars().collect();

    let matches4 = kmp_search(&text_chars4, &pattern_chars4);
    println!("Texto: '{}'", text4);
    println!("Padrão: '{}'", pattern4);
    println!("Padrão encontrado nos índices: {:?}", matches4); // Deve imprimir []
}
