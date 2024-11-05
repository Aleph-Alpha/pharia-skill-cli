# Pharia Skill CLI

A simple CLI that helps you publish and run skills on Pharia Kernel.

## Contributing

Currently, the integration tests have a dependency on the Pharia Kernel crate, which is available on Aleph Alpha's Cargo registry in JFrog Artifactory.

The access token can be provided with:

```sh
cargo login --registry=jfrog "Bearer $JFROG_TOKEN"
```

This only needs to be done once. The provided token is stored in `$CARGO_HOME/credentials.toml`.
