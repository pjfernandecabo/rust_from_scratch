# 🦀 Curso Práctico de Rust — *Learning by Doing*

Bienvenido al curso **Rust desde cero**, diseñado especialmente para ingenieros y programadores con experiencia en otros lenguajes (como Python, C++ o Go) que quieran aprender Rust **con enfoque práctico**.

El objetivo es **aprender Rust construyendo**, entendiendo cada concepto mediante ejemplos y ejercicios guiados, hasta llegar a desarrollar una aplicación real que integre **Rust + Python + IA**.

---

## 🚀 Estructura del Curso

El curso está dividido en **10 lecciones** progresivas que cubren todo lo esencial del lenguaje:

| Lección | Tema Principal | Descripción breve |
|----------|----------------|------------------|
| **0** | Introducción y fundamentos básicos | Instalación, estructura de proyecto, `cargo`, tipos básicos, `println!`, comentarios |
| **1** | Variables, mutabilidad y tipos de datos | Tipos primitivos, inferencia de tipos, mutabilidad y constantes |
| **2** | Funciones y control de flujo | `fn`, parámetros, retornos, `if`, `match`, bucles |
| **3** | Ownership, Borrowing y Lifetimes | El corazón de Rust: gestión de memoria segura sin GC |
| **4** | Structs y Enums | Modelado de datos, métodos asociados e implementación de traits |
| **5** | Collections y Strings | Vectores, strings, slices, iteradores y manipulación eficiente |
| **6** | Result, Option y manejo de errores | Patrones seguros para controlar errores |
| **7** | Módulos, Crates y organización del código | Reutilización, visibilidad y estructura en proyectos medianos |
| **8** | Concurrencia y asincronía | `threads`, `async/await`, `tokio` y patrones concurrentes |
| **9** | Integración con Python e IA | Llamadas entre Rust y Python (`pyo3`), bindings y ejemplos reales |
| **10** | Proyecto final | Construcción de una mini API Rust + Python con inferencia ML |

---

## 🧰 Instalación de Rust

1. **Instala Rust con rustup (recomendado):**

   ```bash

   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
    ```

2. **Verifica la instalación:**

    ```bash
    rustc --version
    cargo --version
    ```

3. **Actualiza Rust cuando lo necesites:**

    ```bash
    rustup update
    ```

## 🏗️ Estructura del Proyecto

```css
rust-learning-by-doing/
│
├── lesson_00_intro/
│   ├── src/
│   │   └── main.rs
│   └── Cargo.toml
│
├── lesson_01_variables/
├── lesson_02_functions/
│
└── README.md

```

Para ejecutar una lección:

```bash
cd lesson_00_intro
cargo run

```

## Git

## 🔄 Flujo de trabajo con Git

Este repositorio usa una estructura profesional basada en GitFlow:

- **`main`** → rama estable (solo versiones completas y testeadas)
- **`dev`** → rama activa de desarrollo
- **`feature/leccion-XX`** → rama temporal por cada lección

### 1️⃣ Creas la rama de desarrollo (una vez)

```bash
git checkout -b dev
git push -u origin dev
```

### 2️⃣ Creas una rama por lección

Por ejemplo, para la Lección 1:

```bash
git checkout -b feature/leccion-01
```

### 3️⃣ Trabajas y haces commits en esa rama

```bash
git add 01_object_inspector/
git commit -m "Lección 1: finalizada — Object Inspector"

```

### 4️⃣ Fusionas con dev cuando termines

```bash
git checkout dev
git merge feature/leccion-01
git push

```

### 5️⃣ Fusionas dev → main al finalizar un módulo completo

```bash
git checkout main
git merge dev
git push

```

**Ejemplo de flujo:**
```bash
git checkout -b feature/leccion-01
# trabajo y commits

git checkout dev
git merge feature/leccion-01
git push

git checkout main
git merge dev
git push
```


## 🧪 Requisitos Previos

- Conocimientos básicos de programación (Python, C++, Go…)

- Tener instalado Rust y cargo

- Editor recomendado: VSCode con extensión rust-analyzer

## 🧠 Filosofía del curso

- Learning by doing: aprenderás Rust construyendo.

- Pequeños pasos: cada lección introduce un concepto nuevo con ejercicios.

- Práctica constante: los ejemplos son ejecutables con cargo run.

- Enfoque aplicado: las últimas lecciones combinan Rust con Python e IA.

## 📄 Licencia

Este proyecto se distribuye bajo la MIT License:
MIT License

Copyright (c) 2025 

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software...

## 💡 Recomendaciones adicionales

- Usa cargo fmt para formatear tu código.

- Usa cargo clippy para linting y sugerencias.

- Explora la documentación con cargo doc --open.

- Mantén el código comentado, especialmente en las primeras lecciones.

## 📚 Recursos complementarios

[📘 Rust Book (oficial)](https://doc.rust-lang.org/book/)

[💡 Rust by Example](https://doc.rust-lang.org/rust-by-example/)

[🔬 Crates.io](https://crates.io/)
 — repositorio de librerías Rust

[🦀 Rustlings](https://github.com/rust-lang/rustlings)
 — ejercicios interactivos