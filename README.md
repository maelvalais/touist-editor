TouIST web editor
=================

![](https://img.shields.io/docker/build/touist/editor-ide.svg?label=touist/editor-ide:latest)
![](https://img.shields.io/docker/build/touist/editor-server.svg?label=touist/editor-server:latest)
![](https://img.shields.io/docker/build/touist/editor-nginx.svg?label=touist/editor-nginx:latest)

A simple browser based ide for [Touist](https://github.com/touist/touist)

![ScreenShot](./assets/screenshot1.png)

### Installation
```git clone https://github.com/graphman65/touist-editor```

### Requirements
- [Docker](https://www.docker.com/)
- [docker-compose](https://docs.docker.com/compose/)

### Development
```HOST=touist.localhost docker-compose -f docker-compose.dev.yml up```


### Prod
```
HOST=touist.localhost docker-compose build
docker-compose up
```
