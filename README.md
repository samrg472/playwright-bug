# Playwright bug demonstration

This repository is a demonstration of node native modules not having output using the standard `println!` macro in Rust.

## Running

### 1. Setup the main project
```shell
$ npm install
```

### 2. Build the native module

```shell
$ cd native
$ npm install
$ npm run build
```

### 3. Run the test suite
```shell
$ npx playwright test
```
