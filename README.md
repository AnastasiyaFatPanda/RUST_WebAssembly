# Install WebAsssembly & RUST
### Установить Rust (если ещё не установлен)
`curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`

### Добавить таргет для WebAssembly
`rustup target add wasm32-unknown-unknown`

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


Add the package into `index.html` and RUN:

`npx http-server .`


