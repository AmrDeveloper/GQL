# Releasing GitQL Application

1. Update the `CHANGELOG.md`:
2. Update `Cargo.toml` with the new version.


3. Commit

   ```
   $ git commit -am "Prepare version X.Y.X"
   ```

4. Tag

   ```
   $ git tag -am "X.Y.Z" X.Y.Z
   ```

5. Push!

   ```
   $ git push && git push --tags
   ```

   This will trigger a GitHub Action workflow which will create a GitHub release and
   publish to Cargo.

# Releasing GitQL SDK crate

  1. Update `Cargo.toml` with the new version.
  2. `cargo publish --allow-dirty`