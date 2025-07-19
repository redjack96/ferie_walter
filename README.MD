# PROGETTO FERIE WALTER

Per avviare in locale (dalla cartella che contiene Cargo.toml): 

```shell
cargo run
```

Per avviare sul browser (dalla cartella che contiene Cargo.toml):
La prima volta:
```sh
rustup target add wasm32-unknown-unknown
cargo install cargo-binstall
cargo binstall trunk
```
```sh
trunk serve --config ferie_walter_client/Trunk.toml
```

E poi clicca sul link.

Per aggiornare le librerie:

```shell
cargo update
```

Per aggiornare solo egui_custom (trovi il link del progetto sul Cargo.toml):
```shell
cargo update -p egui_custom
```




