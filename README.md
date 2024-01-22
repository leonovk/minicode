# Minicode

## 👹 Esoteric programming language

![Build Status](https://github.com/leonovk/minicode/actions/workflows/ci.yml/badge.svg)

## Installation

Minicode currently supports Intel Macs, M1 ARM Macs, and Linux. The tool has been tested on these platforms and is expected to work on other Unix-like systems as well. If you encounter any issues running minicode on your system, please let me know by creating an issue on the GitHub repository.

### Unix (MacOs/Linux) manual install

This instruction works for both Linux and macOS.

Download the latest release from the [releases page](https://github.com/leonovk/minicode/releases) for your platform. For example, if you are using an Intel Mac, download the `minicode-x86_64-apple-darwin.tar.gz` file. For an M1 Mac, download the `minicode-aarch64-apple-darwin.tar.gz` file.

Extract bin file from the archive:

```bash
tar -xzvf minicode-aarch64-apple-darwin.tar
```

- Move the `minicode` binary to `/usr/local/bin` if you use **mac**
- Move the `minicode` binary to `/usr/bin` if you use **linux**
  
```bash
sudo mv minicode /usr/bin
```

> sudo is required to move the binary to `/usr/bin`.

You can enter the following command to verify that the installation was successful.

```bash
minicode --version
```

Command `--help` will offer you a list of possible commands

## Update

To update your version to the latest use the following command

```bash
sudo minicode --update
```

This command will automatically download the latest release and install it

## Contributing

Contributions to Minicode are welcome! If you have a feature request or find a bug, please create an issue on the GitHub repository. Pull requests are also welcome.
