# Pith
Clone this repository, and then build with `cargo install --path pith-bin`. You can then use `pith --help` to get a list of commands.
## About
pith is a language based around vectors. It features 3 stacks; The left stack I, the right stack J, and the control stack N. Each command is one byte, with two data types also present in the source code. In general, a pith program will look like a sequence of ascii characters, with occasional newlines. All characters not used by pith are ignored, which makes comments using text possible without any delimiter.

Each program written in pith is split into a line-by-line structure, wherein the <code>,</code> operator can jump to various other lines. pith is not space sensitive, but is newline sensitive. If a program reaches the end of a line but hasn't halted, it will move onto the next line. If there is no next line, the program will terminate. Similarly, jumping to a line past the last one will terminate the program.

The two data types found in pith are vectors and floats. Floats are just a 64-bit floating point number as defined by the IEEE 754 standard. A vector is an array of two of these floating point numbers, stylized as `[F, F]`.
## Syntax
[Click here](https://esolangs.org/wiki/Pith#Syntax)
## Programs
### Cat program
```
<#&,1_>
!
```

### Truth Machine
```
#&,1<&_>
_
```

### Hello World
```
:72|:101|:108&2||:111&|:44|:32|:87||:114||:100|:33|
```

### Division
Only works when the output is a whole number
```
#2:1*<&*-&,1==:1+*>
==_
```