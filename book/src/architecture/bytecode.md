# Bytecode

VM load the bytecode 
-> build tree in vm (Tree with value const, if value is lamda, it will eval the bytecode with offset)
-> 

## The format

## The opcode

- `LDC dr, const_offset`
- `LIM dr, number`
- `MTB dr`
- `SAT dr, rKey, rValue`
- `RET dr`
- `MTH dr`

### LDC - Load const

```txt
 1  2  3  4  5  6  7  8  9  10 11 12 13 14 15 16 17 18 19 20 21 22 23 24 25 26 27 28 29 30 31 32
|--|--|--|--|--|--|--|--|--|--|--|--|--|--|--|--|--|--|--|--|--|--|--|--|--|--|--|--|--|--|--|--|
|    8bit               |   4bit    |                          20bit                            |
```


### MTB - Make Table 

```txt
 1  2  3  4  5  6  7  8  9  10 11 12 13 14 15 16 17 18 19 20 21 22 23 24 25 26 27 28 29 30 31 32
|--|--|--|--|--|--|--|--|--|--|--|--|--|--|--|--|--|--|--|--|--|--|--|--|--|--|--|--|--|--|--|--|
|    8bit               |                                                           |           |
|    opcode             |                                                           |    Reg    |
```

### SAT - Set Attribute 

```txt
 1  2  3  4  5  6  7  8  9  10 11 12 13 14 15 16 17 18 19 20 21 22 23 24 25 26 27 28 29 30 31 32
|--|--|--|--|--|--|--|--|--|--|--|--|--|--|--|--|--|--|--|--|--|--|--|--|--|--|--|--|--|--|--|--|
|    8bit               |                                   |    4bit   |           |           |
|    opcode             |                                   |  Des Reg  |  Key Reg  | Value Reg |
```


### MTK  - Make Thunk 

```txt
 1  2  3  4  5  6  7  8  9  10 11 12 13 14 15 16 17 18 19 20 21 22 23 24 25 26 27 28 29 30 31 32
|--|--|--|--|--|--|--|--|--|--|--|--|--|--|--|--|--|--|--|--|--|--|--|--|--|--|--|--|--|--|--|--|
|    8bit               |   4bit    |                          20bit                            |
```

### RET - Return 

```txt
 1  2  3  4  5  6  7  8  9  10 11 12 13 14 15 16 17 18 19 20 21 22 23 24 25 26 27 28 29 30 31 32
|--|--|--|--|--|--|--|--|--|--|--|--|--|--|--|--|--|--|--|--|--|--|--|--|--|--|--|--|--|--|--|--|
|    8bit               |                                                           |           |
|    opcode             |                                                           |    Reg    |
```

```txt
port = 3030;
response.message = "Hello";

 1  2  3  4  5  6  7  8  9  10 11 12 13 14 15 16 17 18 19 20 21 22 23 24 25 26 27 28 29 30 31 32
|--|--|--|--|--|--|--|--|--|--|--|--|--|--|--|--|--|--|--|--|--|--|--|--|--|--|--|--|--|--|--|--|
|              |           |           |        |
     opcode       Des Reg         ...
```

## Section 

### In head
- Magic number(u32): 0x414E4749 = "ANGI"
- version(u32): 0x0001 = "0.0.1"
- const_offset(u32)
- const_size(u32)
- code_offset(u32)
- code_size(u32)


## Example
Save

```txt
{
   port = 3030;
   response = {
      message = "Hello";
   };
}
```
To memory 


```txt
[Const]
1: "port"
2: "response"
3: "message"
4: "Hello"
5: 3030
...
[Code]
MTB r0
LDC r1, c1
LDC r2, c5
SAT r0, r1, r2 // port = 3030
LDC r1, c2
MTK r2, 1
SAT r0, r1, r2 // response = <thunk 1>
RET r0

:thunk 1
MTB r0
LDC r3, c3
LDC r4, c4
SAT r0, r3, r4 // message = "Hello"
RET r0
```


```txt
{
   port = 3030;
   handler = () {
      let b = 1 + 1;
      return b;
   }
}
```


```txt
[Const]
1: "port"
2: "path"
3: "handler"
5: 3030
...
[Code]
MTB r0
LDC r1, c1
LDC r2, c5
SAT r0, r1, r2

MTB r2
LDC r3, c3
LDC r4, c4
SAT r2, r3, r4

LDC r1, c2
SAT r0, r1, r2
RET r0
```
