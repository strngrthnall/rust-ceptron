/*
 * netmath.rs
 *
 * Módulo de funções matemáticas para a rede neural.
 *
 * Este módulo implementa:
 *   - Funções de ativação (identidade, sigmoid)
 *   - Funções de custo (MSE - Mean Squared Error)
 */

use num::pow;

/*
 * Função de ativação Identidade.
 *
 * Retorna o valor de entrada sem modificação.
 * Útil para problemas de regressão linear onde
 * a saída não precisa ser limitada a um intervalo.
 *
 * Parâmetros:
 *   x - valor de entrada (soma ponderada + bias)
 *
 * Retorno:
 *   O próprio valor x (f(x) = x)
 */
#[allow(dead_code)]
pub fn ident(x: f32) -> f32 {
    x
}

/*
 * Função de ativação Sigmoid (Logística).
 *
 * Comprime qualquer valor de entrada para o intervalo (0, 1).
 * Muito utilizada em problemas de classificação binária.
 *
 * Fórmula: σ(x) = 1 / (1 + e^(-x))
 *
 * Propriedades:
 *   - Saída sempre entre 0 e 1
 *   - σ(0) = 0.5
 *   - Derivada: σ'(x) = σ(x) * (1 - σ(x))
 *
 * Parâmetros:
 *   x - valor de entrada (soma ponderada + bias)
 *
 * Retorno:
 *   Valor entre 0 e 1 representando a probabilidade de ativação
 */
#[allow(dead_code)]
pub fn sigmoid(x: f32) -> f32 {
    1.0 / (1.0 + (-x).exp())
}

/*
 * Calcula o Erro Quadrático Médio (Mean Squared Error).
 *
 * Parâmetros:
 *   out_true - vetor com os valores esperados (gabarito)
 *   out_pred - vetor com os valores preditos pelo neurônio
 *   sample_size - número de amostras
 *
 * Retorno:
 *   O erro quadrático médio entre os valores esperados e preditos
 */

pub fn mse(
    out_true: &[f32], 
    out_pred: &[f32], 
    sample_size: usize
) -> f32 {
    let mut sum_squared_errors = 0.0;

    for i in 0..sample_size {
        sum_squared_errors += pow(out_pred[i] - out_true[i], 2);
    }
    sum_squared_errors / sample_size as f32
}
