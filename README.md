# Install WebAsssembly & RUST

### Установить wasm-pack (инструмент для сборки WebAssembly)
`cargo install wasm-pack`


### Create a new lib

`cargo new --lib wasm_name`


# RUN:

`npm run start`

or

`wasm-pack build --target web`

После компиляции появится папка pkg/, содержащая:
•	.wasm файл,
•	JS-обёртку для работы с Wasm.


Add a new package into `index.html` and RUN:

`npx http-server .`


