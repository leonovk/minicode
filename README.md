# 👹 Minicode

## Esoteric programming language

## Why? Just for fun!

[Documentation here](https://leonovk.github.io/minicode/)

![Build Status](https://github.com/leonovk/minicode/actions/workflows/ci.yml/badge.svg)

## Installation

Minicode currently supports Intel Macs, M1 ARM Macs, and Linux. The tool has been tested on these platforms and is expected to work on other Unix-like systems as well. If you encounter any issues running minicode on your system, please let me know by creating an issue on the GitHub repository.

### Unix (MacOS / Linux)

This instruction works for both Linux and macOS.

```bash
curl -fsSL https://raw.githubusercontent.com/leonovk/minicode/master/install.sh | bash
```

You can enter the following command to verify that the installation was successful.

```bash
minicode --version
```

Command `--help` will offer you a list of possible commands

If the minicode command was not found, most likely you need to enter a new path in the settings of your .bash_profile (or similar)

```bash
echo 'export PATH=${executable_folder}:\$PATH' >> .bash_profile
```

In this case, an example of such a command will be shown to you at the end of the installation script.

## Update

To update your version to the latest use the following command

```bash
minicode --update
```

This command will automatically download the latest release and install it

## How to run minicode in docker

To run your minicode code in docker you need to do the following:

```bash
git clone https://github.com/leonovk/minicode.git
docker build -t minicode ./minicode
```

After this, the minicode image will be available on your local machine.

Next, you can create your own images based on the minicode image and run your minicode in them.

For example:

```Dockerfile
FROM minicode:latest

COPY . .

CMD ["minicode", "-p", "hello_world.mcode"]
```

## Contributing

Contributions to Minicode are welcome! If you have a feature request or find a bug, please create an issue on the GitHub repository. Pull requests are also welcome.
