# Sitemap Rustico 👨‍🌾

Originalmente hecho por [ph4un00b](https://github.com/ph4un00b/sitemap-rustico). Esto es un fork temporal ✨.


## 🍕 Diagrama General

![diagrama](diagrama.png)

- antes de crear algún cambio modificar el diagrama❗
- https://www.tldraw.com/s/v2_c_vsmDpfm4FPQSM-nfkKamO?d=v-1889.-958.3912.2580.page

## 🍔 Instructions

- fetch below repositories:
  - ```sh
    git clone https://github.com/RustLangES/blog.git
    git clone https://github.com/RustLangES/RustLangES.github.io.git home
    git clone https://github.com/RustLangES/rust-book-es.git book
    git clone https://github.com/RustLangES/rust-para-dotnet-devs.git dotnet
    ```
- Sólo elegís el script para tu sistema, no tengas miedo de otros scripts de otros sistemas operativos no te van a morder.
  - windows: `generate.bat`
  - POSIX: `generate.sh`

## ✨ Agregando un nuevo libro ó pagina para indexar:

- obtener los path que van ser indexados, por ejemplo:
  - para el libro _rust para dotnet devs_:
    - clonar el repositorio `https://github.com/RustLangES/rust-para-dotnet-devs.git`
    - asignarle un nombre de una sola palabra `git clone https://github.com/RustLangES/rust-para-dotnet-devs.git dotnet`
    - obtener los path de las nuevas páginas para ser indexadas.
      - trae las archivos de forma recursiva: `git ls-tree -r --name-only HEAD --full-tree src/es`
      - filtramos y formato: `grep "\.md$" | awk -F "." "{print $1}"`
      - podes ver un ejemplo para bash o batch en `/scripts`
    - agregar al `ignore_paths.rs` los path que no tienen sentido como:
      - `dotnet/src/es/SUMMARY`,
      - `dotnet/src/es/license`
    - ajustar apropiadamente `src/generate.rs`
      - podes usar de referencia: `https://github.com/ph4un00b/sitemap-rustico/commit/2c0e4e761fd48d25ff2d27f89723d11e57140c59`

## 🧪 testing workflow

- deps:
  - docker
  - gh
  - act
- act.exe -j test -s GITHUB_TOKEN="$(gh auth token)"

## 🍗 hard todo

- [ ] remove xmllint

## ✨ todo

- [ ] el código no esta tan trabajado si querés mejorar la estructura, podes contactar antes en la comunidad para retroalimentar y no mal usar vuestro tiempo.
- [ ] mejorar el readme para agregar libros.
