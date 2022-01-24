### js语法点(ts为主)

### 数据类型
- 原始数据类型：
string，number，bigint，boolean，symbol，null 和 undefined
- 对象： object

### 变量声明
var 当前模块作用域

let 当前代码作用域

const 当前模块静态变量

### 模块系统
JavaScript 现在有两种模块。
一种是 ES6 模块，简称 ESM；
另一种是 CommonJS 模块，简称 CJS。

CommonJS 模块是 Node.js 专用的，与 ES6 模块不兼容。
语法上面，两者最明显的差异是，
CommonJS 模块使用require()和module.exports，
ES6 模块使用import和export。