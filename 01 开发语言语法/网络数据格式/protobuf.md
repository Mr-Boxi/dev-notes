## protobuf

### 1 环境安装
    1 protoc v3.7.1
    2 protoc-gen-go v1.3.3
1.1 protoc 安装

参考资料：  
https://blog.csdn.net/u010918487/article/details/82947157  

https://blog.csdn.net/BeautifulGrils/article/details/52775120?spm=1001.2101.3001.6650.14&utm_medium=distribute.pc_relevant.none-task-blog-2%7Edefault%7EBlogCommendFromBaidu%7Edefault-14.opensearchhbase&depth_1-utm_source=distribute.pc_relevant.none-task-blog-2%7Edefault%7EBlogCommendFromBaidu%7Edefault-14.opensearchhbase

https://blog.csdn.net/Awesomewan/article/details/106207763


    sudo apt install autoconf automake libtool curl make g++ unzip
    
    git clone -b v3.7.1 https://github.com/protocolbuffers/protobuf.git
    
    （可以下载项目后在切换）
    
    cd protobuf
    
    git tag v3。7.1
    
    ./autogen.sh
    
    ./configure
    
    make
    
    make check
    
    sudo make install
    
    sudo ldconfig
    
    protoc --version


    go get -u -d github.com/golang/protobuf/protoc-gen-go@v1.3.3
    #指定版本，如果没有则下载最新的版本
    go install github.com/golang/protobuf/protoc-gen-go@v1.3.3
    
    将gobin 写入环境变量中
    vi /etc/profile

### 2 编写proto文件    
    ...
### 3 编译
    protoc --go_out=plugins=grpc:. ./*.proto
    // --go_out=plugins=grpc:.  输出到当前目录
    // ./*.proto  指定文件
    
    // 编译命令
    protoc -I ../ \
    --go_opt=paths=source_relative \
    --go_out=plugins=grpc:./ \
    -I ./ protos/network.proto
    
    
