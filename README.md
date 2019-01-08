# enigma
enigma is a CLI tool that keeps my secrets. It is designed to be used locally and on remote machines to securely and easily distribute and install secrets.

The following backends are currently supported:
- 1Password (CLI)

## Usage

To store a secret environment variable:
```shell
$ enigma save env personal_github GITHUB_TOKEN 6eyiehhe7h8eh9h87eg8egg8ge
```

You can then load it into the environment:
```shell
$ enigma export personal_github
```

To store a secret file(s):
```shell
$ enigma save file cradle_kubeconfig ~/.kube/*.yml
```

You can then load the files, with any original folder structure preserved:
```shell
$ enigma export cradle_kubeconfig ~/.kube/
```

## TODO

- [ ] Parse JSON from items output of `op`
- [ ] Filter items by some tag or category for enigma
- [ ] Store environment variable
  - [ ] Store environment variable from environment
- [ ] Store file(s)
