# Fraction - 高精度分数库

### 📖 简介  
Rust实现的高精度分数类型，使用 `i32` 存储分子分母。支持数学运算、特殊值处理、安全类型转换及哈希

（初学 rust，个人练手）

---

### 🎯 核心功能

#### 🔢 数学运算
- **四则运算**：`+ - * /` 及对应的 `+= -= *= /=` 运算符
- **扩展运算**：取反、绝对值、倒数、符号判断、特殊值判断
- **隐式转换**：支持与整数直接运算（自动转分数）

#### 🚩 特殊值系统
- 预定义常量：`INFINITY`（`i32::MAX/1`）、`NEG_INFINITY`（`i32::MIN/1`）、`ZERO`、`NAN`
- 运算规则与浮点数规范一致

#### 🔄 类型转换
```rust
// safe
let f64_val: f64 = Fraction::new(-5, 4).into();
assert_eq!(f64_val, -1.25);
let f64_inf: f64 = f64::from(Fraction::INFINITY);
assert_eq!(f64_inf, f64::INFINITY);
// unsafe
let res_err: Result<i32, _> = Fraction::INFINITY.try_into();
assert_eq!(res_err, Err(ConversionError::InfiniteConversion));
let res_ok: Result<i32, _> = i32::try_from(Fraction::new(-3, 2));
assert_eq!(res_ok, Ok(-1));
```

#### ✅ 安全特性
- **Shrink**​​：运算溢出时自动寻找最简近似解
```rust
let n = Fraction::new(2147483647, 4);
let prev = (n + 1) / 2;
let curr = (n / prev + prev) / 2;

// shrink to 805306375/6 (= 134217729.16666666)
// not 4611686065672028153/34359738416 (= 134217729.1875)
assert_eq!(curr, Fraction::new(805306375, 6));
```
- **​​错误处理​​**：
  - `OutOfRangeError`：转换值超出目标类型范围
  - `NaNConversion`：尝试转换NaN值
  - `InfiniteConversion`：无限值转有限类型

#### 💡 使用注意
- **​​相等判断​​**：
  - **直接比较**：`==` 严格比较约分后的分子分母
  - **数学相等**：建议用 `(a - b) == Fraction::ZERO`
```rust
let a = Fraction::new(155937625, 24970004);   // 6.244997998398398
let b = Fraction::new(2103597937, 336845254); // 6.244997998398398
assert!(a != b);
assert_eq!(a - b, Fraction::ZERO);
```
- **​​哈希兼容​​**：
  - 已实现哈希特质，可直接用于HashMap等数据结构

### 示例代码（算术平方根）
```rust
use fraction::Fraction;
        
fn sqrt(n: Fraction) -> Option<Fraction> {
    let mut prev;
    let mut curr;

    if n.is_negative() {
        return None;
    } else if n.is_zero() {
        return Some(Fraction::ZERO);
    } else if (n - 1).is_positive() {
        prev = (n + 1) / 2;
    } else {
        prev = Fraction::from(1);
    }

    curr = (n / prev + prev) / 2;
    for _ in 0..20 {
        if curr - prev == Fraction::ZERO {
            return Some(curr);
        }
        prev = curr;
        curr = (n / prev + prev) / 2;
    }
    Some(curr)
}

assert_eq!(sqrt(Fraction::from(100)).unwrap(), Fraction::from(10));
```

