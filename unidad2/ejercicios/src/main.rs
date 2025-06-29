/// Para ejecutar las pruebas en Rust, generalmente se utiliza el comando `cargo test`
/// desde la raíz del proyecto. Cargo compilará tu código de prueba (las funciones
/// anotadas con `#[test]` dentro de módulos con `#[cfg(test)]`) y luego las ejecutará.
///
/// **Ejecutar todas las pruebas:**
/// ```bash
/// cargo test
/// ```
/// Este comando ejecutará todas las pruebas definidas en tu proyecto.
///
/// **Ejecutar una prueba específica:**
/// Puedes filtrar las pruebas por nombre de función o por parte del nombre.
/// Por ejemplo, para ejecutar una prueba llamada `mi_prueba_especifica`:
/// ```bash
/// cargo test mi_prueba_especifica
/// ```
///
/// **Ejecutar pruebas en un módulo específico:**
/// Si tienes pruebas dentro de un módulo llamado `integracion`, puedes ejecutar solo esas pruebas:
/// ```bash
/// cargo test --test integracion
/// ```
/// (Nota el uso de `--test` seguido del nombre del archivo de integración, sin la extensión `.rs`).
///
/// **Ejecutar pruebas que coincidan con un patrón:**
/// Puedes usar patrones para ejecutar múltiples pruebas. Por ejemplo, para ejecutar todas
/// las pruebas que contengan la palabra "http":
/// ```bash
/// cargo test http
/// ```
///
/// **Mostrar la salida de `println!` en las pruebas:**
/// Por defecto, la salida de `println!` en las pruebas exitosas se captura y no se muestra.
/// Para ver esta salida (incluso en pruebas exitosas), puedes usar la siguiente bandera:
/// ```bash
/// cargo test -- --nocapture
/// ```
/// (Nota el doble guion `--` que separa las opciones de `cargo test` de las opciones
/// pasadas al ejecutable de pruebas).
///
/// **Ejecutar solo pruebas fallidas de la ejecución anterior:**
/// Esto puede ser útil para iterar rápidamente en las pruebas que están fallando:
/// ```bash
/// cargo test -- --only-failures
/// ```
///
/// Recuerda que las funciones de prueba deben estar anotadas con `#[test]` y generalmente
/// se agrupan dentro de módulos anotados con `#[cfg(test)]`.

fn main() {
    println!("Ejercicios de Rust!. Use: 'cargo test' para ejecutar la pruebas.");
}