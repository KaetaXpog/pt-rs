## About
Rust写的pt站爬虫。爬取资源信息到数据库。用于检索。

## 使用方法
目前支持多个pt站，以okpt为例。
1. 新建config/okpt.cookies 文件，从浏览器获得okpt站的cookies。文件内容大致以下格式
```
c_secure_uid=XXXXXX; c_secure_pass=XXXX; c_secure_ssl=eWVhaA%3D%3D; c_secure_tracker_ssl=eWVhaA%3D%3D; c_secure_login=bm9wZQ%3D%3D

```

2. cargo run

## TODO
- [ ] 处理PTTIME的cloudflare盾牌
    + [ ] RSS 方案绕过反爬
- [x] 把cookies从代码中分离出来
- [x] 增加爬虫调度器
    + [ ] 爬虫状态保存
    + [ ] 爬虫状态恢复
- [x] 适配其他pt站
    + [x] okpt
    + [x] icc2022    
    + [x] ggpt
    + [x] carPT
    + [x] pttime

- [ ] 识别，记载free资源
- [ ] 检索功能
- [ ] 展示数据
- [ ] 加点命令行参数
