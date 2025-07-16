# 🦀 Diplomado Rust - 
*Viaje de Aprendizaje y Soluciones*

![Rust Version](https://img.shields.io/badge/rust-1.72.0%2B-orange)
![Última Actualización](https://img.shields.io/github/last-commit/mervinso/curso-rust-ejercicios/main)
[![Sincronizado con batouxclass/curso-rust-ejercicios](https://img.shields.io/badge/Sincronizado%20con-batouxclass%2Fcurso--rust--ejercicios-brightgreen)](https://github.com/batouxclass/curso-rust-ejercicios)

Este repositorio documenta mi progreso y soluciones a los ejercicios del Diplomado de Rust impartido por [Orlando Contreras Castellanos](https://github.com/batouxclass) (basado en su material original de `batouxclass/curso-rust-ejercicios`). El objetivo principal es aplicar los conceptos aprendidos y tener un registro personal de mi avance.

## 🚀 Sobre el Diplomado
<!-- Completa esta sección con detalles específicos del diplomado -->
*   **Nivel:** (Ej: Introductorio, Intermedio) 
*   **Enfoque:** (Ej: Fundamentos de Rust, Programación de sistemas, etc.)
*   **Material Original:** [batouxclass/curso-rust-ejercicios](https://github.com/batouxclass/curso-rust-ejercicios)

## 🛠️ Pre-requisitos

*   **Rust:** Versión `1.72.0` o superior. Puedes instalar Rust siguiendo las instrucciones en [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install).
*   **Cargo:** El gestor de paquetes y sistema de construcción de Rust. Se instala automáticamente con Rust.

## 🗂️ Estructura del Repositorio

El contenido está organizado de la siguiente manera:

```text
.
├── .gitignore          # Archivos y directorios ignorados por Git
├── README.md           # Este archivo
├── unidad1/            # (Vacío actualmente) Material y notas de la Unidad 1: Fundamentos iniciales y configuración
├── unidad2/            # Material y ejercicios de la Unidad 2: Conceptos básicos y tipos de datos
│   ├── clase/          # Ejemplos y código desarrollados durante las clases
│   │   ├── Cargo.toml  # Metadatos del proyecto de ejemplos de clase
│   │   └── src/        # Código fuente de los ejemplos
│   └── ejercicios/     # Mis soluciones a los ejercicios propuestos
│       ├── Cargo.toml  # Metadatos del proyecto de ejercicios
│       ├── README.md   # (Será eliminado, información integrada aquí)
│       ├── src/        # Código fuente de los ejercicios (main.rs y otros módulos si los hubiera)
│       └── tests/      # Pruebas unitarias para validar los ejercicios
├── unidad3/            # (Vacío actualmente) Próximas unidades: (Ej: Ownership, Structs, Enums)
└── recursos/           # (Vacío actualmente) Materiales de apoyo: imágenes, PDFs, libros de referencia, etc.
```
*(Nota: La estructura detallada de `target/` y otros archivos generados por Cargo se omite por brevedad).*

## 💻 Ejercicios y Progreso

A continuación, se detalla el progreso de los ejercicios por unidad.

### Unidad 2: Conceptos Básicos de Rust
*(Temas: Variables, tipos primitivos, literales, shadowing, constantes, tipos escalares, tipos compuestos, tuplas, arrays, control de flujo if/else, bucles (loop, while, for), match, strings, slices.)*

| Ejercicio                                 | Estado         | Fecha de Finalización | Solución / Evidencia                                                      |
| ----------------------------------------- | -------------- | --------------------- | ------------------------------------------------------------------------- |
| `1_test_primitive_types`                  | ✅ Completado  | 2024-07-15            | [Ver tests](unidad2/ejercicios/tests/1_test_primitive_types.rs)           |
| `2_test_conversion_primitivos`            | ✅ Completado | 2024-07-16             | [Ver tests](unidad2/ejercicios/tests/2_test_conversion_primitivos.rs)     |
| `3_test_tipos_compuestos_tuplas`          | 🚧 En Progreso | ---                   | [Ver tests](unidad2/ejercicios/tests/3_test_tipos_compuestos_tuplas.rs)   |
| `4_test_tipos_compuestos_array`           | ⏳ Pendiente   | ---                   | [Ver tests](unidad2/ejercicios/tests/4_test_tipos_compuestos_array.rs)    |
| `5_test_control_flujo_if`                 | ⏳ Pendiente   | ---                   | [Ver tests](unidad2/ejercicios/tests/5_test_control_flujo_if.rs)        |
| `6_test_control_flujo_for_while_loop`     | ⏳ Pendiente   | ---                   | [Ver tests](unidad2/ejercicios/tests/6_test_control_flujo_for_while_loop.rs)|
| `7_test_control_flujo_match`              | ⏳ Pendiente   | ---                   | [Ver tests](unidad2/ejercicios/tests/7_test_control_flujo_match.rs)     |
| `8_test_control_string_str_slice`         | ⏳ Pendiente   | ---                   | [Ver tests](unidad2/ejercicios/tests/8_test_control_string_str_slice.rs)|

**Leyenda de Estado:**
*   ✅ `Completado`: Todos los tests del archivo pasan.
*   🚧 `En Progreso`: Trabajando activamente en la solución.
*   ⏳ `Pendiente`: Aún no iniciado.
*   ❌ `Bloqueado/Problemas`: Se encontraron dificultades que impiden la finalización.

## ⚙️ Cómo Ejecutar los Ejercicios

Todos los ejercicios se encuentran dentro del proyecto `unidad2/ejercicios/` y se pueden validar usando las pruebas unitarias.

1.  **Navegar al directorio de ejercicios:**
    ```bash
    cd unidad2/ejercicios
    ```

2.  **Ejecutar un conjunto de tests específico:**
    (Los tests están agrupados por archivos, cada archivo corresponde a un "ejercicio" listado arriba)
    ```bash
    # Reemplaza <nombre_del_archivo_de_test> con el nombre del archivo sin la extensión .rs
    # Ejemplo: Para ejecutar los tests de '1_test_primitive_types.rs':
    cargo test --test 1_test_primitive_types
    ```

3.  **Ejecutar todos los tests de los ejercicios de la Unidad 2:**
    ```bash
    cargo test
    ```
    Esto ejecutará todos los archivos de test presentes en el directorio `unidad2/ejercicios/tests/`.

## 🔄 Sincronización con el Repositorio Base del Curso

Para mantener este repositorio actualizado con el material original del profesor:

```bash
# 1. Configurar el repositorio remoto 'upstream' (solo la primera vez):
git remote add upstream https://github.com/batouxclass/curso-rust-ejercicios.git

# 2. Verificar los remotos configurados (opcional, para confirmar):
git remote -v

# 3. Traer los cambios y ramas del 'upstream':
git fetch upstream

# 4. Asegurarse de estar en tu rama principal (generalmente 'main' o 'master'):
git checkout main  # o tu rama principal

# 5. Fusionar los cambios de 'upstream/main' (o la rama principal del curso) en tu rama local 'main':
git merge upstream/main # Asumiendo que la rama principal del curso es 'main'

# 6. Subir los cambios actualizados a tu repositorio personal en GitHub:
git push origin main
```
*Nota: Si realizas cambios locales en archivos que también fueron modificados en el `upstream`, podrías necesitar resolver conflictos de fusión.*

## 🛡️ Resolución de Conflictos al Sincronizar

Si al fusionar los cambios del repositorio base (`main`) en tu rama de trabajo aparece un conflicto (ejemplo: `.gitignore`), sigue estos pasos:

1. Abre el archivo marcado como conflictivo y resuelve las diferencias manualmente.
2. Elimina las marcas de conflicto (`<<<<<<<`, `=======`, `>>>>>>>`) y guarda el archivo.
3. Agrega el archivo resuelto:
    ```bash
    git add <archivo>
    ```
4. Finaliza el merge:
    ```bash
    git commit
    ```
5. Continúa trabajando en tu rama y sube tus cambios normalmente.

Esto asegura que mantienes tu progreso y los cambios del curso actualizados.

## 📚 Recursos Adicionales Útiles

*   **Documentación Oficial de Rust:**
    *   [El Libro de Rust (Español)](https://google.github.io/comprehensive-rust/es/index.html) - Referencia completa y detallada. Ideal para profundizar conceptos.
    *   [Rust by Example (Inglés)](https://doc.rust-lang.org/rust-by-example/) - Aprendizaje práctico con ejemplos de código concisos.
*   **Plataformas de Aprendizaje y Práctica:**
    *   [Rustlings](https://github.com/rust-lang/rustlings) - Pequeños ejercicios interactivos para acostumbrarse a leer y escribir código Rust. ¡Muy recomendado!
    *   [Exercism - Rust Track](https://exercism.org/tracks/rust) - Desafíos de programación en Rust con mentoría de la comunidad.
*   **Comunidad:**
    *   [Foro Oficial de Usuarios de Rust (Inglés)](https://users.rust-lang.org/) - Para preguntas y discusiones.
    *   [Comunidad de Rust en Español (Telegram)](https://t.me/rust_es) - Grupo activo para hispanohablantes.
*   **Gestión de Paquetes:**
    *   [Crates.io](https://crates.io/) - El registro oficial de paquetes (crates) de la comunidad Rust.

## 👨‍💻 Sobre el Autor

**Mervin Sosa**
*   GitHub: [mervinso](https://github.com/mervinso)
<!-- *   (Opcional: LinkedIn, Twitter, etc.) -->

## 📄 Licencia

El contenido de este repositorio, incluyendo mis soluciones a los ejercicios, es para fines educativos y de aprendizaje personal. El material original del curso pertenece a [Orlando Contreras Castellanos](https://github.com/batouxclass).
Por favor, consulta la licencia del repositorio original [batouxclass/curso-rust-ejercicios](https://github.com/batouxclass/curso-rust-ejercicios) para el material base.
Mis propias adiciones y soluciones en este fork pueden considerarse bajo una licencia permisiva como MIT si necesitas reutilizarlas, pero siempre dando crédito al material original del curso.
