<h1 align="center">Bilibili-dl</h1>
<p align="center" class="shields">
    <a href="https://github.com/jw-jackson/bilibili-dl/issues" style="text-decoration:none">
        <img src="https://img.shields.io/github/issues/jw-jackson/bilibili-dl.svg" alt="GitHub issues"/>
    </a>
    <a href="https://github.com/jw-jackson/bilibili-dl/stargazers" style="text-decoration:none" >
        <img src="https://img.shields.io/github/stars/jw-jackson/bilibili-dl.svg" alt="GitHub stars"/>
    </a>
    <a href="https://github.com/jw-jackson/bilibili-dl/network" style="text-decoration:none" >
        <img src="https://img.shields.io/github/forks/jw-jackson/bilibili-dl.svg" alt="GitHub forks"/>
    </a>
    <a href="https://github.com/jw-jackson/bilibili-dl/blob/master/LICENSE" style="text-decoration:none" >
        <img src="https://img.shields.io/badge/License-GPLv3-blue.svg" alt="GitHub license"/>
    </a>
</p>
<h3 align="center">不断更新中....</h3>


A commandline tool to download video from bilibili.

# 目录结构
```
bilibili-dl
|
|-- src/
|   |-- main.c
|   |-- api.c 
|   |-- login.c
|   
|-- include/
|-- lib/
|
|-- api.json
|-- Makefile
|-- READEME.md
```

# 库
```
|- cJSON        -- 处理 json 数据, 数据存储
|
|- curl         -- http 请求, fetch stream
|
|- Gstreamer    -- 考虑使用其 api 混流
|
|- FFmpeg       -- 考虑使用其 api 混流
```


# 音视频处理
```
API 使用过于复杂，目前选择调用 ffmpeg
```
