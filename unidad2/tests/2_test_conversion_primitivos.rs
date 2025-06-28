#[cfg(test)]
mod conversion_tests {
    #[test]
    fn test_widening_conversion_as() {
        // Objetivo: Practicar la conversión "widening" (ampliación) donde no se pierde información.

        // 1. Declara una variable `small_number` de tipo u8 con el valor 100.


        // 2. Convierte `small_number` a un tipo u32 usando la palabra clave `as`.
        // Esta conversión es segura porque u32 puede contener todos los valores de u8.


        // 3. Verifica que la conversión fue exitosa.


        // 4. Declara un i16 y conviértelo a i64.

    }

    #[test]
    fn test_narrowing_conversion_as() {
        // Objetivo: Entender los riesgos de la conversión "narrowing" (reducción).

        // 1. Declara una variable `big_number` de tipo i32 con el valor 300.


        // 2. Convierte `big_number` a un tipo i8 usando `as`.
        // ¡Atención! i8 solo puede contener valores de -128 a 127.
        // La conversión truncará los bits. 300 en binario es 0001 0010 1100.
        // Al truncar a 8 bits, nos quedamos con 0010 1100, que es 44 en decimal.


        // 3. Verifica el resultado del truncamiento.


        // 4. ¿Qué pasa con un número negativo?

        // 1000 en binario es 0011 1110 1000. Truncado a 8 bits es 1110 1000.
        // En complemento a dos, 1110 1000 representa -24.

    }

    #[test]
    fn test_float_to_integer_conversion() {
        // Objetivo: Observar cómo se maneja la conversión de flotante a entero.

        // 1. Declara una variable `pi` de tipo f64.


        // 2. Convierte `pi` a un i32 usando `as`.
        // La parte decimal se trunca (se elimina, no se redondea).


        // 3. Verifica que la parte decimal fue eliminada.


        // 4. Prueba con un número negativo.

    }

    #[test]
    fn test_integer_to_float_conversion() {
        // Objetivo: Practicar la conversión de entero a flotante.

        // 1. Declara un entero `my_integer`.

        // 2. Conviértelo a f64. Esta conversión generalmente es segura y no pierde precisión
        // para enteros de 32 bits.


        // 3. Verifica la conversión.


        // 4. Para enteros muy grandes (i64), la conversión a f32 puede perder precisión.
        let big_integer: i64 = 9007199254740991; // Límite de precisión para f64
        let float_from_big: f64 = big_integer as f64;

        let bigger_integer: i64 = 9007199254740993; // Un número mayor
        let imprecise_float: f64 = bigger_integer as f64;

        assert_eq!(float_from_big, 9007199254740991.0);
        // Debido a la falta de precisión, el resultado de la conversión no es exacto.
        // El entero se redondea al valor f64 más cercano representable.
        assert_eq!(imprecise_float, 9007199254740992.0, "La conversión debería redondear al flotante más cercano.");
    }

    #[test]
    fn test_from_trait_conversion() {
        // Objetivo: Usar el trait `From` para conversiones seguras y explícitas.
        // `From` es una forma más idiomática y segura de realizar conversiones "widening".

        // 1. Declara un u8.


        // 2. Usa `u16::from()` para convertir el u8 a u16.


        // 3. Haz lo mismo para convertir un i8 a i32.


        // 4. El trait `Into` es la contraparte de `From`. Si `T::from(U)` está implementado,
        // entonces `U` tiene una implementación de `into()` para `T`.

    }
}
