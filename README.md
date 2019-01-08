# enigma
enigma is a CLI tool that keeps my secrets. It is designed to be used locally and on remote machines to securely and easily distribute and install secrets.

The following backends are currently supported:
- 1Password (CLI)

## Usage

To store a secret environment variable:
```shell
$ enigma save env personal_github GITHUB_TOKEN 6eyiehhe7h8eh9h87eg8egg8ge
```

or, if the variable is in the environment, you can omit the value:
```shell
$ enigma save env personal_github GITHUB_TOKEN
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

# License

This project is licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or
   http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or
   http://opensource.org/licenses/MIT)

at your option.

## TODO

- [x] Parse JSON from items output of `op`
- [x] Filter items by some tag or category for enigma
- [x] Store environment variable
  - [x] Store environment variable from environment
- [x] Store file(s)
- [ ] Move to local daemon model
- [ ] Create server
- [ ] Add lease approvals to client/server
- [ ] Add secret rotation to server
