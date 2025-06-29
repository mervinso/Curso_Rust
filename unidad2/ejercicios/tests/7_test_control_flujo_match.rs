#[cfg(test)]
mod match_statement_tests {

    // `match` es una de las construcciones de control de flujo más poderosas de Rust.
    // Permite comparar un valor con una serie de patrones y ejecutar código basado en qué patrón coincide.
    // Es como un `switch` en otros lenguajes, pero mucho más potente.

    #[test]
    fn test_match_basico() {
        // Objetivo: Entender la sintaxis básica de `match`.

        let numero = 3;
        let resultado: &str;

        // 1. Usa `match` para comparar `numero` con diferentes valores.

    }

    #[test]
    fn test_match_con_option() {
        // Objetivo: Usar `match` para manejar el tipo `Option<T>` de forma segura.

        fn obtener_numero(val: bool) -> Option<i32> {
            if val {
                Some(42)
            } else {
                None
            }
        }

        // 1. Usa `match` para manejar el caso `Some`.


        // 2. Usa `match` para manejar el caso `None`.
        
    }

    #[test]
    fn test_match_con_multiples_patrones_y_rangos() {
        // Objetivo: Aprender a combinar patrones y usar rangos.

        let numero = 5;

        let descripcion = match numero {
            1 | 2 => "Es uno o dos", // El operador `|` permite múltiples patrones.
            3..=6 => "Está entre 3 y 6 (incluidos)", // `..=` define un rango inclusivo.
            _ => "Es otro número",
        };

        assert_eq!(descripcion, "Está entre 3 y 6 (incluidos)");
    }

}
