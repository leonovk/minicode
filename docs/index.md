# Minicode docs

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

## Assigning Variables

The assignment operation always has the name of the variable and the value that needs to be put into it.

For example, how to put the number 431 in variable a:

```mc
> a 434
```

For example, how to put the string hello world in a variable:

```mc
> a Hello World!
```

You can use quotes or not, if the string cannot be represented as an integer, it will remain a string.

## Console output

You can display the contents of a variable to the console using the command `p`

For example:

```mc
p a
```

## Reading from a file

Using the `f` command you can read from a file

For example:

```mc
f a test/test_file.txt
```

## Request value from user

You can request the value from the user using the command `$>`

For example:

```mc
$> a
```

Also, as the third parameter, you can specify the text that will be shown to the user when requesting a value. For example:

```mc
$> a text
```

### Arithmetic operations

Arithmetic operations are carried out through the command `a`

For example, how to add a number to a variable:

```mc
= a + 12
```

For example, how to subtract a number from a variable:

```mc
= a - 12
```

Multiplication and division are also supported. The second argument can be variables.

### Conditions and cycles

At the end of each condition there is a line number where the interpreter will go if the condition is true. Accordingly, if you send the interpreter back, you can implement loops, and if you send it forward, you can implement conditional branches.

For example, if a equals 0, move the interpreter to the fifth line:

```mc
? a = 0 5
```

For example, if a equals not 0, move the interpreter to the fifth line:

```mc
? a ! 0 5
```

Here's how, for example, to implement a loop that displays the message hello world 5 times:

```mc
> a 0
> b Hello world
= a + 1
p b
? a ! 5 3
```

The conditions in minicode are quite powerful for typical languages of this class. You can compare and compare all data types with each other (although there are only two of them), but only in accordance with the type. You cannot compare a number with a string. You can also compare values that are in a variable. However, when specifying a variable, keep in mind that the minicode will first try to parse the value into a number, then if that doesn’t work, it will look to see if there is a variable with that name in memory, and if not, it will consider what you specified as a regular string.

For example, the following code will display only `just text` on the screen

```mc
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
