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
<pre>
Example:  
 
a = 1     // int value 
b = 1.1   // float value 
c = "str" // string value 
d = true  // (or false) - bool value 
</pre>
### If statement

The Ex language supports the "if" construction, which has the following format: if [condition] {code block} [else {code block}].
The condition should be a boolean expression.  <br />
<pre>
Example:  

 if 1 > 2 {
    writeln("false") // writeln is part of the standard library, see examples  
}  
 
my_bool = true  
if my_bool {  
    writeln("true")  
}
</pre>

### Loops

For loop:
for i in [start, end] { ... }

- "i" is the loop variable that takes on values from "start" to "end"
- The code inside the curly braces will be executed for each value of "i"
<pre>
Example:
 
writeln("start for loop from 0 to 9")
for i in [0, 10] {
    writeln("index in loop is ", i)
}
writeln("loop finished")
</pre>
 
While loop:
while condition { ... }
- The code inside the curly braces will be executed repeatedly as long as the "condition" is true
<pre>
Example:
 
i = 10
writeln("i = ", i)
writeln("run while untill i > 0")
while i > 0 {
    writeln("in while i == ", i)
    i = i - 1
}
writeln("while finished")
</pre>

### Functions
You can use functions from the standard library in Ex, for a list of them see examples folder
<pre>
Example:
 
a = read()
write("input is: ", a)
b = "   multi line test    "
writeln(b, b, b)
</pre>
You can also declare your own functions as follows: 
<pre>
fn {function_name}([list_of_args]) 
   function_body 
}
Example:
 
fn sum2(a, b) {
    return a + b
}
a = sum2(44, 6)
b = a + 2
write(a + b - 2
</pre>

