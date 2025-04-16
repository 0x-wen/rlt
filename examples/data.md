

# post 请求数据
```
cargo run --example http_reqwest -- https://me-explorer.meuat.xyz/me/validator/getValidatorDelegationByPage --method POST --data "{\"page_number\": 1, \"page_size\": 100}" -d 10s -r 10 -c 3

cargo run --example http_reqwest -- https://browser3.meuat.xyz/proxy/api/rest/txlist?limit=10&offset=0 --method GET -d 60s -r 100 -c 10 
./target/release/examples/http_reqwest https://browser3.meuat.xyz/proxy/api/rest/txlist?limit=10&offset=0 --method GET -d 60s -r 100 -c 10 

./target/release/examples/http_reqwest https://explorer4.meuat.xyz/api/rest/region/states --method POST --data "{\"offset\": 0, \"limit\": 100 }" -d 60s -r 100 -c 100 

```

### 测试get请求 带参数与不带参数对比
```
cargo run --example http_reqwest -- http://middle-ground-gateway.xd.local/api/browser/block/detail --method GET --data "{\"height\": 2261600}" -d 10s -r 10 -c 3

cargo run --example http_reqwest -- 'http://middle-ground-gateway.xd.local/api/browser/block/detail?height=2261600' --method GET -d 10s -r 10 -c 3


url: "http://middle-ground-gateway.xd.local/api/browser/block/detail?block_height=2261600"
status: Status { kind: ClientError, code: 400 }, bytes: 111, duration: 19.176167ms

url: "http://middle-ground-gateway.xd.local/api/browser/block/detail?height=2261600"
status: Status { kind: Success, code: 200 }, bytes: 403, duration: 16.345917ms
```



## manage后台api测试

```shell
cargo run --example http_reqwest -- 'https://gateway.meuat.xyz/group/group/recommended?address=me1z2twmv5mpg4swjda2c6cdha9f3mdap98w8afa7' --method GET -d 10s -r 10 -c 3

cargo run --example http_reqwest -- 'https://gateway.meuat.xyz/group/group/recommended?address=me1z2twmv5mpg4swjda2c6cdha9f3mdap98w8afa7' --method GET -d 60s -r 100 -c 10
```


## 环境编译
```shell
# 查看rust 支持的所有目标平台
rustup target list
# 安装 Ubuntu 对应的目标（例如 x86_64-unknown-linux-gnu）
rustup target add x86_64-unknown-linux-gnu
# 使用 musl 静态链接（兼容性更强）
rustup target add x86_64-unknown-linux-musl

# 安装 Homebrew 的交叉编译工具
brew install FiloSottile/musl-cross/musl-cross

# 静态链接所有库（避免依赖 glibc 版本）
cargo build --release --target=x86_64-unknown-linux-musl
# 检查二进制文件是否静态链接
file ./target/x86_64-unknown-linux-musl/release/your_binary
# 输出应包含 "statically linked"

```
