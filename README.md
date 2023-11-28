---
runme:
  id: 01HG8S9S322K1DGCBPYT6384CF
  version: v2.0
---

# llm-sdk 用 Rust 创建 OpenAI SDK

## 实操

```shell {"id":"01HG8S9S31CJJQZT5CEBJRFN1N"}
cargo add reqwest --features rustls-tls --no-default-features --features json --features gzip

cargo add tokio --features rt --features rt-multi-thread --features macros --dev      

cargo add anyhow     

cargo add serde --features derive

cargo add serde_json     

cargo add derive_builder

cargo nextest run -- create_image_request_shoud_serialize
cargo nextest run -- create_image_request_custom_shoud_serialize         
mkdir /tmp/llm-sdk  
cargo nextest run -- create_image_should_work -- --nocapture
open /tmp/llm-sdk/caterpillar.png    

```
