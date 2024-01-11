项目结构:
    Noter/
        - db/ 数据库
        - src/ 前端源代码
            - components/ 组件
            - router/ 路由
            - store/ 状态管理
            - views/ 页面
            - App.vue: 入口组件
            - main.js: 入口文件
        - src-tauri/
            - src/ 后端源代码
            - target/ 编译目标
                - release/ 发布文件夹
                    - bundle/msi/ 安装包
                    - Noter.exe: 可执行文件
                - .rustc_info.json: Rust编译信息
            - Cargo.lock: Rust依赖锁
            - Cargo.toml: Rust依赖项
            - .gitignore: Git忽略文件
        - screenshots/ 运行截图
        - README.txt: 项目说明

项目介绍: 
    本项目使用了Vue.js、Tauri、Sled、Rust、Node.js、TypeScript、HTML、CSS等技术，
    是一个跨平台的桌面应用程序，可以在Windows、Linux、MacOS等系统上运行。
    本项目的功能是实现一个简单的记事本，可以添加、删除、修改、查询、导出、导入记事本。
    本项目的特点是使用了Tauri框架，可以将前端、后端、数据库打包成一个可执行文件.
    最后，本项目在数据库方面根据项目数据结构的特点,使用了rust社区支持的sled树形数据库,
    这是一个非常高效的数据库,可以在多线程环境下运行,而且可以在多核处理器上运行,并且提供了非常好的api.

编者备注:
    由于本人出于对函数式编程的兴趣，所以本项目使用了函数式编程的思想，使用了函数式编程的方法，
    在前端和后端源代码中可以多次见到迭代器的使用,而不是显式的使用for循环，这样可以使代码更加简洁.
    以及函数式编程风格中的变量可不变性,在前端和后端源代码中可以多次见到不可变变量的使用,
    这样可以减少函数的副作用,以及更好的处理并发问题和利用多核处理器来提高程序的运行速度.
    Rust is Awesome!

    电子2102 李源罡 20216264
