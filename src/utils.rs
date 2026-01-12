/*
 * utils.rs
 *
 * Módulo de utilitários gerais.
 *
 * Este módulo implementa:
 *   - Geração de números aleatórios para inicialização de pesos
 */

use rand::Rng;

/*
 * Gera um valor aleatório em um intervalo especificado.
 *
 * Parâmetros:
 *   min - limite inferior do intervalo (inclusivo)
 *   max - limite superior do intervalo (exclusivo)
 *
 * Retorno:
 *   Um valor aleatório entre min e max
 */

pub fn randomize(min: f32, max: f32) -> f32 {
    rand::thread_rng().gen_range(min..max)
}