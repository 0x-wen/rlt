/*

# post 请求数据
```
cargo run --example http_reqwest -- https://me-explorer.meuat.xyz/me/validator/getValidatorDelegationByPage --method POST --data "{\"page_number\": 1, \"page_size\": 100}" -d 10s -r 10 -c 3


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

*/


## manage后台api测试

```shell
cargo run --example http_reqwest -- 'https://gateway.meuat.xyz/group/group/recommended?address=me1z2twmv5mpg4swjda2c6cdha9f3mdap98w8afa7' --method GET -d 10s -r 10 -c 3

cargo run --example http_reqwest -- 'https://gateway.meuat.xyz/group/group/recommended?address=me1z2twmv5mpg4swjda2c6cdha9f3mdap98w8afa7' --method GET -d 60s -r 100 -c 10
```