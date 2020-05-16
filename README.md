# Pipes 
## Team Members
    Nicholas Warfield
    Andrew Alarcon
    Elena Adame
    Maxwell Plotkin
    Sai Pappu

##  Design Philosophy 

## Code Snippets

## Limitations

## Lessons Learned 

## Compilation Process

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

