# gitbook入门

typero + gitbook + git 

(gitbook.com <---关联---> github.com)

教程：

    https://blog.csdn.net/qq_43363200/article/details/105388065?utm_medium=distribute.pc_relevant.none-task-blog-2~default~baidujs_baidulandingword~default-0.no_search_link&spm=1001.2101.3001.4242.1
出错：

    https://blog.csdn.net/ARPOSPF/article/details/118789156
### 1.1 安装相关环境
1 nodejs环境
   
    # gitbook工具需要nodejs环境
    https://nodejs.org/en/download/
    
2 安装gitbook工具

    npm install -g gitbook-cli

3 安装 typora
    
    https://typora.io/#windows

### 1.2 安装gitbook报错 

坑：gitbook出现TypeError: cb.apply is not a function解决办法
https://blog.csdn.net/yq_forever/article/details/112121742
解决方法：
https://www.cnblogs.com/cyxroot/p/13754475.html

### 1.3 设置gitbook_workspace
在电脑中创建一个 Gitbook 文件夹，用于放置自己的 Gitbook 文件。
再在 Gitbook 文件夹中创建新的文件夹，以 bookname 命名。

gitbook init
坑：在此系统禁止运行脚本
此处若失败并出现提示信息“因为在此系统禁止运行脚本”，需要以管理员身份打开power shell，输入set-ExecutionPolicy RemoteSigned，选择“是”。


### 1.4 编辑
markdown + 管理summary.md    

### 1.5 使用git做版本管理
1 在 Github 创建好仓库
2 在 bookname 目录下执行 git init初始化本地仓库
3 git remote add origin git@server-name:path/repo-name.git关联远程仓库。
4 关联后，使用命令git push -u origin master第一次推送master分支的所有内容。


### 1.6 关联Gitbook
进入 Gitbook，用 Github 账号登录。

高阶 Gitbook 命令
生成 PDF gitbook pdf ./ ./mybook.pdf

生成 epub 格式的电子书 gitbook epub ./ ./mybook.epub

生成 mobi 格式的电子书 gitbook mobi ./ ./mybook.mobi

https://www.cnblogs.com/jiangming-blogs/p/14643151.html



