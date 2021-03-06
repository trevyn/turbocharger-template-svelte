# turbocharger-template-svelte

Template for a fully-configured Turbocharger project with Svelte, Tailwind, and Turbosql.

Prerequisites:

```
rustup target add wasm32-unknown-unknown
```

```
cargo install wasm-pack
```

Run full stack:

```
npm run start-debug
```

Run Svelte frontend only in Hot Module Replacement dev mode (you'll need to manually run the server with `npm run start-debug` in a separate terminal, and re-run it if you change any Rust code):

```
npm run frontend-dev
```

Make release build:

```
npm run build
```
