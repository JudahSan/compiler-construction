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

Implemented as an HDL stuf file

```
/** Xor gate: out = (a And Not(b)) Or (Not(a) And b)) */

CHIP Xor {
  IN a,b;
  OUT out;
  
  PARTS:
  // Implementation missing
}
```
