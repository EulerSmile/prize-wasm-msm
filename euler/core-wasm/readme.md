## Miracl库的极简版
### 只保留了G1的部分
包括Fp域计算

包括ECP点运算

包括rand随机数

### 脚本
emcc.py 可以生产wasm，用http-serve可以启动测试，

修改index.html可在浏览器console中查看相应输出

gcc.py 可以生产pc可执行文件

clean.py 可以清理生产的文件

### 关于条件编译
wasm环境中，c语言的print、time等均无法使用

所以在pc端测试需要输出时，读取时间时，可以使用条件编译

```	c
#if GCC
    time_t start, end;
#endif

#if GCC
    ECP_BLS12381_output(&Q);
#endif

```

## 常用输出

fp的输出

```
void BIG_384_29_output(BIG_384_29 a)
```

point的输出

```
void ECP_BLS12381_output(ECP_BLS12381 *P)
```

## 常用数据结构

大整数，无论是fp还是fr均采用

```c
BIG_384_29
```

点

```c
ECP_BLS12381
```

