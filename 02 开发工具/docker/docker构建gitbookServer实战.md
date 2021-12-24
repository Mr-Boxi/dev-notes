## 利用docker发布gitbook的书籍

### 构建相关文件
```dockerfile
# 构建docker容器
# 基础镜像
FROM nginx
# 容器中的工作目录
WORKDIR /usr/share/nginx/html
# 将当前的_book路径拷贝到容器的工作目录
ADD ./_book .
# 暴露端口方便映射
EXPOSE 80
```
``` makefile
build:
	sudo docker build -t matrixchaindocs:v1.0 .
run:
	sudo docker run -p 4000:80 --name matrixchaindocs -d matrixchaindocs:v1.0
stop:
	sudo docker stop matrixchaindocs
rebuild:
	## 重新构建,用于更新内容
	# 停掉容器
	sudo docker stop matrixchaindocs
	# 删除容器
	sudo docker rm matrixchaindocs
	# 删除镜像
	sudo docker rmi matrixchaindocs:v1.0
	# 重新build
	sudo docker build -t matrixchaindocs:v1.0 .
```