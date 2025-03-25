# Pharia Skill CLI

A simple CLI that helps you publish skills on Pharia Kernel.

## Installation

You can install the CLI from the latest release on GitHub, or using the installer script from the release:

```sh
curl https://github.com/Aleph-Alpha/pharia-skill-cli/releases/download/<INSERT_LATEST_VERSION_HERE>/pharia-skill-cli-installer.sh | sh
```

## Usage

Publish a skill to OCI registry

```
Usage: pharia-skill-cli publish [OPTIONS] --registry <REGISTRY> --repository <REPOSITORY> --tag <TAG> --username <USERNAME> --token <TOKEN> <SKILL>

Arguments:
  <SKILL>  Path to skill .wasm file

Options:
  -R, --registry <REGISTRY>      The OCI registry the skill will be published to (e.g. 'ghrc.io') [env: SKILL_REGISTRY=]
  -r, --repository <REPOSITORY>  The OCI repository the skill will be published to (e.g. 'org/pharia-skills/skills') [env: SKILL_REPOSITORY=]
  -n, --name <NAME>              Published skill name
  -t, --tag <TAG>                Published skill tag
  -u, --username <USERNAME>      User name for OCI registry [env: SKILL_REGISTRY_USER=]
  -p, --token <TOKEN>            Token for OCI registry [env: SKILL_REGISTRY_TOKEN=]
  -h, --help                     Print help
```
