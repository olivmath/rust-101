//! # Como dar nome às coisas?
//!
//! Em Rust usamos "snake_case" para variáveis e funções, por exemplo:
//! - nome_de_uma_variavel              
//! - outro_nome_escrito_com_snake_case
//! - AssimEstaErrado
//! - assim_Tambem_Esta_Errado
//!
//! **Dicas:**
//! - os nomes devem ser descritivos, mas concisos.
//! - os nomes devem esclarecer sobre valores guardados na variável.

#[test]
fn test_nomeando_variaveis() {
    let lower_case_with_underscores = "Rust";
    assert!(lower_case_with_underscores == "Rust");
}

#[test]
fn test_variavel_de_letra_unica() {
    let a = "Valor";
    // Este teste falha porque estamos tentando desencorajar o uso de nomes de variáveis de uma letra.
    assert!(false, "Utilize um nome mais descritivo em vez de `a`.");
}

#[test]
fn test_variavel_com_numero() {
    let var123 = "Número da Conta";
    // Este teste falha porque números no nome da variável devem ter um significado claro.
    assert!(
        false,
        "Utilizar um nome descritivo sem números ou com números significativos."
    );
}

#[test]
fn test_variavel_com_acento() {
    let variável = "portugues";
    // Este teste falha porque acentos no nome da variável podem causar "erros".
    assert!(false, "Utilizar um nome descritivo em ingles.");
}

#[test]
fn test_variavel_caso_misto() {
    let Xx = "Algum valor";
    // Este teste falha porque estamos tentando desencorajar o uso de mistura de maiúsculas e minúsculas sem um padrão claro.
    assert!(false, "Use snake_case para nomes de variáveis em Rust.");
}

#[test]
fn test_variavel_caso_de_camelo() {
    let helloWorld = "Olá Mundo";
    // Este teste falha porque não estamos seguindo a convenção snake_case.
    assert!(
        false,
        "Os nomes das variáveis devem estar em snake_case e não em camelCase."
    );
}

#[test]
fn test_variavel_nao_descritiva() {
    let minha_casa = 122000;
    // Este teste falha porque o nome da variável não é descritivo.
    assert!(
        false,
        "Escolha um nome para a variável que descreva o seu objetivo ou conteúdo."
    );
}

#[test]
fn test_sem_variavel_numero_magico() {
    // Este teste falha porque um valor sem variável torna o código confuso
    // Neste caso, o número 4.90 deveria ser uma variável `dolar_price`.

    assert!(
        4.90 * 100.00 == 490.00,
        "Escolha um nome para a variável que guarda o preço do dólar."
    );
}
