# PROGETTO FERIE WALTER
## Avvio in locale
Per avviare in locale (dalla cartella che contiene Cargo.toml): 

```shell
cargo run -p ferie_walter_client
```
## Avvio su browser web
La prima volta, in qualsiasi cartella:
```sh
rustup target add wasm32-unknown-unknown
cargo install cargo-binstall
cargo binstall trunk
```
Per avviare il server esegui il comando dalla cartella root:
```sh
cargo run -p server
```
Per avviare il client dalla cartella root:
```sh
trunk serve --config ferie_walter_client/Trunk.toml
```
E poi clicca sul link.

## Aggiornare le librerie
Per aggiornare le librerie:
```shell
cargo update
```
Per aggiornare solo egui_custom (trovi il link del progetto sul Cargo.toml):
```shell
cargo update -p egui_custom
```




