# Minicode
### 👹 Esoteric programming language

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

## Tutorial

Minicode executes the code line by line. At the beginning of each line there is a command. There are currently 6 commands in the minicode.
| command | description |
|----------|----------|
| >    | Assigns a value to the specified variable. Meaning, if the value can be parsed into an integer, it will do so.   |
| p    | Displays the value from the specified variable   |
| f    | Tries to find a file at the specified path, and puts all its contents into the specified variable  |
| $>   | Asks the user for a value, if it can be represented as an integer, it will be represented as such
| =    | Start of an arithmetic operation
| ?    | Start of condition

All operations on one line are separated by spaces!

### Assigning Variables

The assignment operation always has the name of the variable and the value that needs to be put into it.

For example, how to put the number 431 in variable a:
```
> a 434
```
For example, how to put the string hello world in a variable:
```
> a Hello World!
```
You can use quotes or not, if the string cannot be represented as an integer, it will remain a string.

### Console output

You can display the contents of a variable to the console using the command `p`

For example:
```
p a
```
### Reading from a file

Using the `f` command you can read from a file

For example:
```
f a test/test_file.txt
```

### Request value from user

You can request the value from the user using the command `$>`

For example:
```
$> a
```

Also, as the third parameter, you can specify the text that will be shown to the user when requesting a value. For example:

```
$> a text
```

### Arithmetic operations

Arithmetic operations are carried out through the command `a`

For example, how to add a number to a variable:
```
= a + 12
```
For example, how to subtract a number from a variable:
```
= a - 12
```

### Conditions and cycles

At the end of each condition there is a line number where the interpreter will go if the condition is true. Accordingly, if you send the interpreter back, you can implement loops, and if you send it forward, you can implement conditional branches.

For example, if a equals 0, move the interpreter to the fifth line:
```
? a = 0 5
```
For example, if a equals not 0, move the interpreter to the fifth line:
```
? a ! 0 5
```

Here's how, for example, to implement a loop that displays the message hello world 5 times:

```
> a 0
> b Hello world
= a + 1
p b
? a ! 5 3
```
The conditions in minicode are quite powerful for typical languages of this class. You can compare and compare all data types with each other (although there are only two of them), but only in accordance with the type. You cannot compare a number with a string. You can also compare values that are in a variable. However, when specifying a variable, keep in mind that the minicode will first try to parse the value into a number, then if that doesn’t work, it will look to see if there is a variable with that name in memory, and if not, it will consider what you specified as a regular string.

For example, the following code will display only `just text` on the screen

```
> a lol
> b lol
? a = b 6
p a
p b
> c 245
> d 345
? c ! d 11
p c
p d
> text just text
p text
```

You will find more examples in the folder -> test/examples


## Contributing

Contributions to Minicode are welcome! If you have a feature request or find a bug, please create an issue on the GitHub repository. Pull requests are also welcome.
