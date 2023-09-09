# Ex Interpreter

This is a simple interpreter for the Ex scripting language. It can be used either as a command line interpreter or to interpret files.

## Purpose

The purpose of the Ex language is to provide a simplified way to directly call external commands, similar to Bash, but with the convenience of the Python language. Ex allows for easy and efficient scripting by allowing users to execute system commands and manipulate files in a concise and readable manner. Ex is currently in development, and many features have yet to be implemented. 

## Usage

To run the command line interpreter:

ex.exe


To interpret a file (see examples folder):

ex.exe <path_to_file>


To show the usage message:

ex.exe --help or ex.exe -h

## Language constructions

### Variables

In the Ex language, variables are created with their name and initializing value. Currently, variables can be of the following types: int, float, bool, string. The type of a variable depends on its initializing value. The type of a variable can change as a result of assigning a new value, and all variables are mutable <br />
 <br />
examples:  <br />
a = 1     // int value <br />
b = 1.1   // float value <br />
c = "str" // string value <br />
d = true  // (or false) - bool value <br />
