## Install from Package managers

## Cargo.io

```sh
cargo install gitql
```

> Note that from version `0.10.0` onward installing from Cargo requires `Cmake` to be installed so it can build the dependencies.

## Winget on Windows

```sh
winget install gitql
```

## Scoop on Windows

```sh
scoop install gitql
```

## Homebrew on MacOS and Linux

```sh
brew install gql
```

# On Single repository
gitql <repository_path>

# On multi repositories
gitql --repo <repository_path> <repository_path> ...etc

# Or
gitql -r <repository_path> <repository_path> ...etc
```

## Download Binaries

From Github repository page you can download the right executable for your OS and Arch from the latest release

## Build GQL From source code

```sh
git clone https://github.com/amrdeveloper/gql
cd gql

# On Single repository
cargo run <repository_path>

# On multi repositories
cargo run -- --repo <repository_path> <repository_path> ...etc
cargo run -- -r <repository_path> <repository_path> ...etc
```

# Command line arguments

```
Usage: gitql [OPTIONS]

Options:
-r,  --repos <REPOS>        Path for local repositories to run query on
-q,  --query <GQL Query>    GitQL query to run on selected repositories
-p,  --pagination           Enable print result with pagination
-ps, --pagesize             Set pagination page size [default: 10]
-o,  --output               Set output format [render, json, csv]
-a,  --analysis             Print Query analysis
-e,  --editor               Enable GitQL LineEditor
-h,  --help                 Print GitQL help
-v,  --version              Print GitQL Current Version
```
