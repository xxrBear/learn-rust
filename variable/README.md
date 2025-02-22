## 变量默认不可更改

```rust
let number = 1;

number = 2; // 会报错
```

## 可修改的变量

```rust
let mut number = 1;
println!("this number is {}", number);
number = 2;
println!("this number is {}", number);
```

## 忽略未使用的变量

```rust
let _x = 1;
```
