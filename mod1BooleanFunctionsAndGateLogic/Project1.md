PROJECT 1 
=

- Given: `Nand`
- Goal: Build the following gates:
  - `Not`
  - `And`
  - `Or`
  - `Xor`
  - `Mux`
  - `Dmux`
  - `Not16`
  - `And16`
  - `Or16`
  - `Mux16`
  - `Or8Way`
  - `Mux4Way16`
  - `Mux8Way16`
  - `DMux4Way`
  - `DMux8Way`
  
  Elementary logic gates
  -

  - `Not`
  - `And`
  - `Or`
  - `Xor`
  - `Mux`
  - `Dmux`


  16-bit variants
  -

  - `Not16`
  - `And16`
  - `Or16`
  - `Mux16`


  Multi-way variants
  -

  - `Or8Way`
  - `Mux4Way16`
  - `Mux8Way16`
  - `DMux4Way`
  - `DMux8Way`


Multiplexor
-

![mux](https://upload.wikimedia.org/wikipedia/commons/thumb/3/39/Multiplexer_2-to-1.svg/175px-Multiplexer_2-to-1.svg.png)


```h
if (sel==0)
  out=a
else
  out=b
```

|a|b|sel|out|
|-|-|-|-|
|0|0|0|0||
|0|1|0|0||
|1|0|0|1||
|1|1|0|1||
|0|0|1|0||
|0|1|1|1||
|1|0|1|0||
|1|1|1|1|

|sel|out|
|0|a|
|1|b|

- A 2-way multiplexor enables selecting, and outputting, one out of two possible inputs.
- Widely used in:
    - Digital design
    - Communications networks

- using mux logic to build a programmable gate <br>
![mux](img/image.png)

Demulriplexor
-

![dum](https://i.stack.imgur.com/n9upM.png)

```h
if (sel==0)
  {a,b}={in,0}
else
  {a,b}={0,in}
```
|in|sel|a|b|
|-|-|-|-|
|0|0|0|0|
|0|1|0|0|
|1|0|1|0|
|1|1|0|1|

- Acts like the "inverse" of a multiplexor.
- Distributes the single input value into one of two possible destinations.
  
- DMux.hdl

```h
CHIP DMux {
  IN in, sel;
  OUT a,b;

  PARTS:
  // Enter code here
}
```

Exmple: Multiplexing/ demultiplexing in communication networks
-

![muxDmux](img/muxDmux.png)

- Each `sel ` bit is connected to an oscillator that produces a repetitive train of alternating 0 and 1 signals.
- Enables transmitting multiple messages on a single, shared communications line
-  A common use of multiplexing / demultiplexing logic


16-bit variants
-

- `Not16`
- `And16`
- `Or16`
- `Mux16`

  And16
  -

  ![And16](img/And16.png)

Multi-way variants
- 
 
  Mux4Way16
  - 
  
  ![16-bit, 4-way multiplexor](img/16bit4wayMultiplexor.png)
  

Chip building materials(using Xor as an example)
-

![Xor](img/Xor.png)

 - The contract: _When running your `Xor.hdl` on the supplied <br> `X0r.tst` your `Xor.out` should <br>be the same as the supplied `Xor.comp`_

- Xor.hdl
  
  ```h
  CHIP Xor {
    IN a, b;
    OUT out;

    PARTS:
    // Put your code here
  }
  ```
- Xor.tst
  ```h
  load.Xor.hdl,
  output-file Xor.out,
  compare-to Xor.cmp,
  output-list a b out;
  set a 0, set b 0, eval, output;
  set a 0, set b 1, eval, output;
  set a 1, set b 0, eval, output;
  set a 1, set b 1, eval, output;
  ```