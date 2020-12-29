Regresiones lineales usando Rust

El repo muestra algunos cálculos utilizando Rust para resolver regresiones lineales utilizando diversas técnicas. Principalmente se está usando algebra lineal a partir del Crate [nalgebra](https://www.nalgebra.org).

Para correr los ejercicios se está utilizando la base de datos [Boston Housing Dataset](https://www.kaggle.com/altavish/boston-housing-dataset)

Hasta ahora se está realizando la regresión lineal resolviendo el sistema de ecuaciones Ax=b con las siguientes técnicas:

- Buscando la inversa de la matriz cuadrada AtA
- Usando la descomposición QR
- Usando la descomposición SVD

En las regresiones lineales no se está utilizando ninguna librería o crate de machine learning y tampoco se está utilizando bindings a LAPACK.

Algunas referencias para la implementación de las regresiones son las siguientes:
- [Machine Learning in Rust, Linear Regression](https://vlad-orlov.com/regressor-in-rust/)
- [SVD for linear Regression](https://sthalles.github.io/svd-for-regression/)