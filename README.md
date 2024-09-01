# Predicate Template by Olta Labs

This repository contains base to create a project around predicates.

## Differents modules

- `src` containing the source code of the predicate
- `typescript_interactions` containing code to make interactions with the predicate in Typescript
- `rust_interactions` containing code to make interactions with the predicate in Rust

## Steps

### 1. Compile the Predicate

```
forc build
```

### 2. Launch a local node

Download the source code from : https://github.com/FuelLabs/fuel-core 

Be careful, maybe the Rust SDK or Fuel SDK are supposed to be used with a certain version, if you encouter versions error in the futur, stop your node, checkout to tag `v0.xx.x`.

Launch a node with a consensus key to build a block :

```
cargo run --bin fuel-core -- --db-type in-memory --consensus-key 9e24cfa071f6c1c4984a17ecf18061a8d0c9c304e7dd7703788bd122bd578650
```

### 3. Interact with the predicate (using Typescript)

Go to `typescript_interactions` folder.

Install dependencies :

```
npm i
```

Run the script that will interact with the predicate :
```
npm run start
```

### 3. Interact with the predicate (using Rust)

Go to `rust_interactions` folder.

Run the script that will interact with the predicate :
```
PREDICATE_BINARY_PATH=../out/debug/predicate.bin cargo run
```

## Special comments

Do not hesitate to participate to this template if you think it could be improved.

If you encouter any problem, feel free to open an issue