## Install from crates.io

```sh
cargo install gitql

# On Single repository
gitql <repository_path>

# On multi repositoies
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

# On multi repositoies
cargo run -- --repo <repository_path> <repository_path> ...etc
cargo run -- -r <repository_path> <repository_path> ...etc
```