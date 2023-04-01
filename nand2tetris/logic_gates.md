Logic Gates
=

![Nand diagram](https://cdn1.byjus.com/wp-content/uploads/2020/06/nand-gate.png)

Elementary logic gates: Nand
-

Any Boolean function can be represented using an expression containing only NAND operations.

```
NOT(x) = (x NAND x)
(x AND y) NOT(x NAND y)
```

<br>
Gate logic: a technique for implementing Boolean functins using logic gates.

Nand functinal specification: 

```
if (a==1 and b==1)
then out=0 else out=1
```

|a|b|out|
|-|-|-|
|0|0|1|
|0|1|1|
|1|0|1|
|1|1|0|

<br><br>
Elementary logc gates: And, Or, Not
<br><br>

AND
```
if (a==1 and b==1)
then out=1 else out=0
```

OR

```
if (a==1 or b==1)
then out=1 else out=0
```

NOT

```
if (n==0)
then out=1 else out=0
```

Gate Interface / Gate Implementation
-

A `logic gate interface` refers to the way in which a logic gate can be connected to other components in a larger system. <br>
This includes the physical connections or pins that a logic gate has for input and output signals, as well as the timing and <br>
voltage specifications that must be followed to ensure proper operation.
<br>
The logic gate interface can be considered an abstraction because it simplifies the way in which the gate can be used by other <br>
components in a larger system, while hiding the complex details of the gate's internal implementation.
<br>
Interface answers the question what.
<br><br>

`The implementation` of a logic gate involves going into another level of detail to describe how the gate actually performs its logical function.
<br>
At the implementation level, we would typically look at the specific electronic components used to construct the gate, such as transistors, <br>
diodes, and resistors, and analyze how they are connected together to create the desired logic function. This could involve examining the gate's <br>
schematic diagram, or even looking at the physical layout of the gate on a chip.
<br><br>
So, while the interface provides a simplified view of the gate that is sufficient for most practical purposes, the implementation level provides a <br>
more detailed and precise description of how the gate works at the physical level.
