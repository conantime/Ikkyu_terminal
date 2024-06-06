# Ikkyu terminal
通过rust的clap和请求API接口，实现的一个简单只能命令行工具

使用方法
1. 下载代码 (rust开发环境 和 git工具安装)
2. cargo build --release 编译
3. 使用说明 -- 测试命令
   -  ikkyu -c -w "add"  相当于 git add . +  git commit -m "add" 两个命令
   -  ikkyu -C -w "add"  相当于 git add . +  git commit -m "add + git push origin master  三个命令
   -  ikkyu -r -l http://xxxxxxxxxx.git -w "add"  相当于 git remote add origin http://xxxxxxxxxx.git + git add . +  git commit -m "add + git push origin master  `四个命令

4. godot引擎 rust脚本 环境的快速创建功能 -- 测试功能

  - ikkyu new test_demo  创建一个godot-rust的游戏工程
  -  进入test_demo 目录
     ikkyu class player   创建一个默认的Node节点的rust脚本，名字为Player.
     ikkyu class player Area2D    创建一个Area2D节点的rust脚本，名字为Player.
    
  -  关于godot的节点名字，请查询godot学习网站
