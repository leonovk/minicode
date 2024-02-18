# Minicode docs

[Документация на русском тут!](https://leonovk.github.io/minicode/ru)

Minicode executes the code line by line, first compiling it into a certain set of commands. Which makes it a classic interpreted programming language. Each line of code in minicode begins with a command, followed by an expression.

At the moment, the minicode has 13 commands and 3 data types, integers, strings and arrays.
Integers behave exactly the same as in Lua. If you don't use decimal characters they will be integers, otherwise not integers. In any case, both will be stored in memory as non-integers.

Minicode is a Turing complete language and in theory allows you to implement any function.

| command | description |
|----------|----------|
| >    | Assigns a value to the specified variable. Meaning, if the value can be parsed into an integer, it will do so.   |
| []   | Initializing an empty array |
| []<  | Add a new element to the end of the array |
| p    | Displays the value from the specified variable   |
| f    | Tries to find a file at the specified path, and puts all its contents into the specified variable  |
| $>   | Asks the user for a value, if it can be represented as an integer, it will be represented as such |
| =    | Start of an arithmetic operation |
| ?    | Start of condition |
| >>   | Write data to file |
| &    | Run operating system command |
| ->   | Include code file |
| -_-  | Stop the code thread |
| @    | Send a message via TCP |

Each line begins with one of these commands and is separated from the expressions by a space.

## Assigning Variables

The variable creation operation symbol is always followed by its name. There is no need to declare the variable type when creating it. The minicode itself will indicate the type. Also, if you do not pass any value, the value of the variable will be equal to the empty string - "".

Here variable a will contain an empty string.

```mcode
> a
```

Here the variable a will contain the number 324.

```mcode
> a 324
```

Here the variable a will contain the string - "hello world"

```mcode
> a hello world
```

Note that you do not need to use double quotes to denote strings. The minicode itself will understand the string you meant or another variable.

If you pass a string as the second argument, the minicode will try to find a previously designated variable with that name, and if it finds one, it will copy its value to the new variable.

Here the variable b will contain the number 324.

```mcode
> a 324
> b a
```

You can also pass many values at once and depending on the previously created variables there will be one or another result.

For example, if we want to copy not the entire string into a new variable, but only a specific character from a specific string in another variable, we can do this:

Here the variable b will contain the string - "e"

```mcode
> a hello
> b a 1
```

You can also select elements from an array.

Arrays are created as follows:

```mcode
# here in variable 'a' we initialize an empty array.
[] a

# here we put a new value at the end of the array.
[]< a 43
```

An example of how to fill an array with characters from the word 'hello'

```mcode
# First we fill the array

[] a
> empty_line
> word hello
> i 0
> char word i
[]< a char
= i + 1
? char ! empty_line 7

# Then we select the necessary elements from it and put them in variables
# and print to the screen

> h a 0
> e a 1
> l a 2
> ll a 3
> o a 4

p h
p e
p l
p ll
p o
```

In such cases, the value of other variables can also act as an index. If nothing is found at the address of the variable 'a', then a new line will simply be created - "a 1".

Minicode supports dynamic heterogeneous arrays that automatically resize and can contain elements of any type.

```mcode
[] first_array
> a text_1
> b 12
[] second_array
[]< first_array a
[]< first_array b
[]< second_array first_array
> arr second_array 0
> b_num arr 1
> num 23
= num + b_num
p num
```

## Console output

You can display the contents of a variable to the console using the command `p`

For example:

```mcode
p a
```

## Reading from a file

Using the `f` command you can read from a file

For example:

```mcode
f a test/test_file.txt
```

## Request value from user

Using the `$>` command you can request values from users. The value will also be moved to the variable according to its type. The minicode will determine the type independently. It is worth noting that all values requested from users in this way will be requested before the code is executed. The code will then be executed as usual, where the query command will be converted into a command to create a regular variable, with the value specified by the users.

For example:

```mcode
$> a
```

Also, as the third parameter, you can specify the text that will be shown to the user when requesting a value. For example:

```mcode
$> a text
```

## Arithmetic operations

Arithmetic operations begin with the command `=`, then there is always a variable that will be changed in the course of the further operation. The original variable will definitely be changed.

For example, this way you can increase the value of the variable 'a' by 12

```mcode
= a + 12
```

You can also chain variables together.

As a result of this operation, the variable 'a' will be increased by the value of the variable 'b'

```mcode
= a + b
```

The same thing works with subtraction, multiplication and division.

An example of a program that displays the word 'hello' line by line:

```mcode
> empty_line
> a hello
> i 0
> char a i
p char
= i + 1
? char ! empty_line 4
```

If you need to find the sum of two numbers, you can do this:

```mcode
> int1 23
> int2 32
> sum 0
> sum + int1
> sum + int2
```

Similarly, you can calculate the difference, etc.

## Conditions and cycles

Conditions and loops are implemented by moving the interpreter to the desired line. Accordingly, if you move the interpreter backwards, you can create a loop; if forward, you can create a conditional branch.
In any case, both begin with a condition command.

The condition command takes an expression that should return true and the line number that the interpreter will jump to if the condition is true.

This operation moves the interpreter to line 5 if the value in the variable 'a' is zero.

```mcode
? a = 0 5
```

For example, if a equals not 0, move the interpreter to the fifth line:

```mcode
? a ! 0 5
```

Here's how, for example, to implement a loop that displays the message hello world 5 times:

```mcode
> a 0
> b Hello world
= a + 1
p b
? a ! 5 3
```

You can also compare variables with each other. The only condition is that the variables must be of the same type.

For example, the following code will display only `just text` on the screen

```mcode
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

The greater than or less than operators '>', '<' are also supported

You will find more examples in the folder -> tests/examples

## Write data to file

Using the >> command, you can easily write the contents of any variable or specified string to a file. The file path always comes first. The file will not be overwritten if it already exists

For example:

```mcode
> a hello world
>> test/test.txt a
```

## Run OS command

Using the '&' command you can easily execute any command on your operating system. The execution result will be stored in a variable. In this case, there is no need to create the variable in advance. The minicode itself will create it if it does not exist, or replace its contents if it does exist.

For example, this is how you can write the current version of your minicode into a variable:

```mcode
& a minicode --version
```

## Include file

You can include another minicode file into your code. The code will connect and be executed immediately.

```mcode
-> tests/examples/hello_world.mcode
```

Just like when running regular minicode code, you can pass arguments to the command line, which will be available as ARG_1, ARG_2 and so on in the list

If you specify a long arrow as a command, as in the example below, then the following code will be executed asynchronously.

```mcode
--> tests/examples/hello_world.mcode
```

## Work with network

Using minicode, you can send messages using the TCH protocol. This allows, in theory, to implement drivers for working with databases and more.

```mcode
> address 127.0.0.1:1509
> message hello from minicode
@ address message
p done
```

## Some examples

### Fibonacci sequence

The code asks the user for the maximum number to which the sequence needs to be built

```mcode
$> max
> i 0
> first 0
> second 1
p first
p second
> sum 0
= sum + first
= sum + second
= first + second
= second + sum
= i + 1
? i ! max 5
```
