### solidity语法点

### 1 数据类型
- 值类型
- 地址类型
- 引用类型

### 2 变量
- 状态变量
- 局部变量
- 全局变量
### 3 变量作用域
- public
- internal
- private
### 4 运算符
- 算术运算
- 比较运算
- 逻辑运算
- 位运算
- 赋值运算
- 条件运算

### 5 循环语句
- while
- do...while
- for
- break and continue

### 6 条件语句
- if
- if...else
- if...else if...

### 7 数据位置
对于引用类型确定数据存储位置(gas费贵)
string, bytes, struct, map
- storage
- memory
- calldata
- stack

### 8 变量的数据位置规则

### 9 赋值的数据位置规则

### 10 字符串

### 11 数组

### 12 枚举-enum

### 13 结构体-struct

### 14 映射-mapping

### 15 类型转换

### 16 以太单位
- 货币单位 wei、finney、szabo或ether
- 时间单位 
```
assert(1 seconds == 1);
assert(1 minutes == 60 seconds);
assert(1 hours == 60 minutes);
assert(1 day == 24 hours);
assert(1 week == 7 days);
```

### 17 特殊的变量、全局变量

### 18 函数
- 函数定义：```function (<parameter types>) {internal(默认)|external} [constant] [payable] [returns (<return types>)]```
- 函数修饰符：modifier
- view 函数 只读
- pure 函数 只计算
- 回退函数
- 函数重载
- 数学函数(内置)
- 加密函数(内置)
- 提款模式
- 限制访问模式

### 19 智能合约
类比java的类
- 合约的结构
- 合约继承
- 合约的构造函数
- 抽象合约
- 接口
- 库
- 事件(event)
- 错误处理

### 20 常用代码库 openzline

### 21 实战
- erc20
- erc721
- erc1155