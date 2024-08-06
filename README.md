## Que hace el lab1

El lab1 es un sistema de Inventarios de una librería, en la cual se guardan datos respecto
a los distintos libros que tiene la librería.<br>
Cada libro contiene la siguiente información:

- ISBN (PK)
- Nombre (Pk)
- Autor
- Categoría
- Precio
- Cantidad
  El sistema es capaz de insertar un registro, actualizar un registro, eliminar un registro y buscar un registro.

## Instalación de Rust y Cargo

Para ejecutar el programa en modo debug, sera necesario instalar Rust y Cargo.<br>
Para instalar Rust ingrese al siguiente link y siga las instrucciónes: [Instalar Rust](https://www.rust-lang.org/tools/install).<br>

También si lo desea puede seguir las instrucciones del [libro](https://doc.rust-lang.org/book/ch01-01-installation.html)<br>

Para verificar que la instalación ha sido exitosa, ejecute estos comandos en consola:

```
rustc --version
cargo --version
```

## Ejecutando el programa

Para ejecutar el programa siga los siguiente pasos:

1. Clone el proyecto
   `git clone https://github.com/apoque96/E2-lab1.git`
2. Navege al proyecto
   `cd lab1`
3. Ejecutelo con Cargo
   `Cargo Run`

Con el programa ejecutado, este le pedira la ruta relativa del archivo con los datos y metodos a realizar, posteriormente le pedira la ruta relativo del archivo con las busquedas a realizar.<br>

El programa imprimira el resultado en consola y creara un archivo `output.txt` con el resultado.
