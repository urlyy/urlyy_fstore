# 介绍
一个用rust + actix实现的简易文件存储服务，仅支持上传和下载两个功能

编写原因:懒得购买云存储服务，但是想分离出一个单独的文件读写服务，但是又懒得搭minio之类的中间件

- 在配置文件.env中，可以修改服务的ip/port/文件下载接口/文件存储目录路径
- 通过访问`http://{ip}:{port}/{download_route}/{filename}`来访问该服务所在服务器的本地文件即`./{file_path}/{filename}`
- 通过`POST http://{ip}:{port}/{upload_route}`上传文件，上传用的是formData