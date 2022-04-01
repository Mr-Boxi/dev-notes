## SGX技术

https://github.com/apache/incubator-teaclave-sgx-sdk/wiki/Environment-Setup
https://github.com/openenclave/openenclave/blob/master/docs/GettingStartedDocs/install_oe_sdk-Ubuntu_18.04.md
https://github.com/apache/incubator-teaclave-sgx-sdk/blob/783f04c002e243d1022c5af8a982f9c2a7138f32/dockerfile/Dockerfile.1804.nightly
https://edp.fortanix.com/docs/installation/guide/

## sgx 的开源项目
teaclave

libos

occlum

#Contributers
### 检查内核是否指出kvm技术
```shell script
sudo apt update -y
sudo apt install cpu-checker -y
kvm-ok
# https://blog.csdn.net/zyklbr/article/details/121722608?spm=1001.2101.3001.6650.18&utm_medium=distribute.pc_relevant.none-task-blog-2%7Edefault%7EBlogCommendFromBaidu%7ERate-18.pc_relevant_default&depth_1-utm_source=distribute.pc_relevant.none-task-blog-2%7Edefault%7EBlogCommendFromBaidu%7ERate-18.pc_relevant_default&utm_relevant_index=24
```

### ubuntu20.04 安装 sgx

