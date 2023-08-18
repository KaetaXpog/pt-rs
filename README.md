## About
Rust写的pt站爬虫。爬取资源信息到数据库

## 使用方法
目前支持多个pt站，以okpt为例。
1. 从浏览器获得okpt站的cookies, 新建config/okpt.cookies 文件。文件内容大致以下格式
```
c_secure_uid=XXXXXX; c_secure_pass=XXXX; c_secure_ssl=eWVhaA%3D%3D; c_secure_tracker_ssl=eWVhaA%3D%3D; c_secure_login=bm9wZQ%3D%3D

```

2. 然后
```shell
# 爬XXX站的免费资源记录
pt-rs.exe XXX free

# 爬XXX站的第 J 页 到  第 K 页数据
pt-rs.exe XXX scrape J K
```


## 数据库
数据爬取到 ./data/pts.sqlite resources表。
resource表由以下的sql语句创建
```sql
CREATE TABLE IF NOT EXISTS resources (
    ID INTEGER PRIMARY KEY AUTOINCREMENT,
    title           TEXT,       -- 英文名
    desc            TEXT,       -- 中文描述
    src_link        TEXT,       -- 详情页链接
    finished        INTEGER,    -- 完成量
    download        INTEGER,    -- 当前下载者
    upload          INTEGER,    -- 当前上传者
    size            REAL,       -- 大小 (GB)
    publish_time    INTEGER,    -- 发布的UNIX时间戳
    last_update     INTEGER,    -- 上一次更新此条目的UNIX时间戳

    discount        INTEGER,    -- 折扣状况
    discount_due    INTEGER     -- 折扣截止的UNIX时间戳
);
```

## TODO
- [ ] 处理PTTIME的cloudflare盾牌
- [ ] 检索功能
- [ ] 展示数据
