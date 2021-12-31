// by boxi
// 学习回调函数
//////////////////////////////
// 阻塞例子
//////////////////////////////
var fs = require("fs");
// var data = fs.readFileSync('input.txt');
// console.log(data.toString());
// console.log("程序结束");

//////////////////////////////
// 非阻塞例子
//////////////////////////////
fs.readFile('input.txt', function(err, data1){
    if (err) return console.error(err);
    console.log(data1.toString());
});
console.log("程序结束");