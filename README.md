# Pipes 
## Team Members
    Nicholas Warfield
    Andrew Alarcon
    Elena Adame
    Maxwell Plotkin
    Sai Pappu

##  Design Philosophy 

## Code Snippets

 Creating a  interger type variable.

`let x:int=32;`

Creating a  String type variable

`let x: str = "Hi!"`

Can also assign void to variable

`let x: void = ();`

Assigning a struct 

`let x: ex = {bar: 32, baz: \"Hi\",};`

Creating a list of type integer

`let x: [int] = [32, 17, -5];`

Example of creating a higher order function

`let x: (int -> int) = (a) { return 1 + a; };`

Creating a function

    func bad_adder(a: int, b: int,) -> int
        {
	                let x: int = a;
					let y: int = b;
					let result: int = a + b;
					return result;
				}

Callning a function

`foo(x ,y);`

Creating a while loop

    while (x != 5){
        return true; 

Creating for loop

`for items in list return items`


## Limitations

## Lessons Learned 

## Compilation Process

Step 1:-   Make sure you have Rust installed in your computer
 
 For  MacOS or Linux 
 `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
    
 [Other Operating systems](https://forge.rust-lang.org/infra/other-installation-methods.html#other-ways-to-install-rustup)

Step 2:- Clone latest version of the code base
`Git clone https://github.com/csun-comp430-s20/pipes.git`

Step 3:-  How to run the compiler?

 To compile a single file use command `rustc`  followed by `filename`

 Ex:- `rustc helloworld.rs`

 To build the whole project use command `cargo build`

Step 4:-  How to run the compiled code?

 To run the whole compiled project in one step use command `cargo run`


## Formal Language Defenition
`var` is a variable

`str` is a string

`int` is an integer

`bool` is a boolean

`void` is a null type

`*_name` is a user defined string

`func_name` is a function

`var_declaration ::= let var_name: type = exp`
`listy_type::= int | str | struct_name | bool |` Types available to lists

`list ::= [ exp,* ]` where exp is a listy_type

type ::= `int | str | bool | struct_name | list | type1 `=> type2
| void 

Built-in types & Custom Struct Types
`struct_type ::= struct_name { (field_name: (str | int | bool),)* }` 
Struct type

`struct ::= struct_name { (field_name: exp,)* }`
Create Struct Instance

struct_field `::= struct.field_name`
`op ::= + | - | % | / | * | < | > | <= | >= | == | != | && | ||` Arithmetic and logical
operations

exp `::= var | str | int | bool | struct | list | exp op exp | !exp | (var: type)` => exp
Variables, strings, and ints are expressions

function_def `::= func func_name ((param_name: type,)*) -> (type) block return
exp`; define function

function_call `::= func_name((exp,)*)` Calls a higher-order function

`stmt ::= var = exp; | if_stmt | while_loop | for_loop | return exp;` Variable
assignment, if statement, return statement

while_loop `::= while (exp) block` While loop

for_loop `::= for var in (list)` for loop

`if_stmt ::= if (exp) `block | `if (exp) block (elif(exp) block)`* else block if else
statement block 

`::= { stmt* }` Block

