//! # O que é uma variável?
//! Variável é um nome dado a uma localização de memória que um programa pode manipular.
//! Em Rust, as variáveis são declaradas usando a palavra-chave `let`.

#[test]
fn test_como_declarar_variaveis() {
    let nome_da_variavel = "Valor da variável";
    assert_eq!(nome_da_variavel, "Valor da variável");
}

#[test]
fn test_definindo_uma_variavel() {
    let x = 10;
    assert_eq!(x, 10);
}

#[test]
fn test_mudando_o_valor_de_uma_variavel() {
    let mut y = 5;
    y = y + 5;
    assert_eq!(y, 10);
}
