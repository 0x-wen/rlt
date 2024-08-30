/*
# 下方get请求数据
cargo run --example=http_reqwest -- https://gateway_middle.meuat.xyz/api/browser/region/list --duration 100s  --rate 1000000 --concurrency 1000

cargo run --example=http_reqwest -- https://me-explorer.meuat.xyz/me/common/getCommonParams --duration 100s  --rate 1000000 --concurrency 400

cargo run --example=http_reqwest -- https://me-explorer.meuat.xyz/me/common/getCommonParams --method GET --duration 100s  --rate 1000000 --concurrency 400

# post 请求数据
cargo run --example http_reqwest -- https://me-explorer.meuat.xyz/me/validator/getValidatorDelegationByPage --method POST --data "{\"page_number\": 1, \"page_size\": 100}" -d 10s -r 10 -c 3
*/
