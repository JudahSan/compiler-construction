![XOR](https://www.build-electronic-circuits.com/wp-content/uploads/2022/09/Truth-table-XOR-gate-417x500.png)

Outputs 1 iff one of its inputs is 1.

|a|b|out|
|-|-|-|
|0|0|0|
|0|1|1|
|1|0|1|
|1|1|0|

<br>

Gate Iterface
-

Implemented as an Hardware Description Language(HDL) stuf file

![alt image](https://github.com/JudahSan/compiler-construction/blob/mod1-boolean-functions-gate-logic/nand2tetris/HDL.png)

```
/** Xor gate: out = (a And Not(b)) Or (Not(a) And b)) */
/** Interface */
CHIP Xor {
  IN a,b;
  OUT out;

/** Implementation */
  PARTS:
  Not (in=a, out=nota);
  Not (in=b, out=notb);
  And (a=a, b=notb, out=aAndNotb);
  And (a-nota, b=b, out=notaAndb);
  Or (a=aAndNotb, b=notaAndb, out=out);
}
```

HDL tips
- HDL is a functional/declarative language.
- The order of HDL statements is insignificant.
- Before using a chip part, you must know its interface. For example
  `Not(in=, out=), And(a= ,b=, out= ), Or(a= ,b= ,out= )`
- Connections like `partName(a=a,...) ` and `partName(...,out=out)` are common.
- Comments: Adding comments to your code is important because it helps you and other programmers understand the purpose and functionality of your code. Comments can also help with debugging and maintaining your code.

- Documentation: Documentation is important because it provides a comprehensive explanation of the design, purpose, and functionality of your code. This is especially important when working on large and complex projects, as it can help other programmers understand how your code fits into the overall system.

- Naming conventions: Using consistent and meaningful naming conventions is important because it makes your code more readable and easier to understand. Good naming conventions can also help prevent errors and make it easier to maintain your code.

- Indentation: Proper indentation is important because it helps to make your code more readable and easier to understand. By indenting your code, you can clearly show the structure of your code and make it easier to see where functions, loops, and other code blocks begin and end.

Common HDLs:
-

- VHDL
- Verilog


Project HDL
-

- Similar to other HDLs
- Minimal and simple

> [HDL Survival Guide](https://www.nand2tetris.org/)
> The Element of Computing Systems: Appendix 2
