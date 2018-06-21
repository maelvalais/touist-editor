FROM nginx:latest

RUN echo '\
server {\n\
  listen 80;\n\
  location /api {\n\
    proxy_pass http://${HOST_API}:${PORT_API};\n\
  }\n\
  location / {\n\
    proxy_pass http://${HOST_WWW}:${PORT_WWW};\n\
  }\n\
}\n'\
>> /etc/nginx/conf.d/nginx.tmpl

ENV HOST=localhost \
    PORT=80 \
    HOST_API=localhost \
    PORT_API=8000 \
    HOST_WWW=localhost \
    PORT_WWW=8080

CMD ["bash", "-c", "envsubst '$$HOST,$$PORT,$$HOST_API,$$HOST_WWW,$$PORT_API,$$PORT_WWW' \
        < /etc/nginx/conf.d/nginx.tmpl > /etc/nginx/conf.d/default.conf && \
        cat /etc/nginx/conf.d/default.conf && \
        nginx -g 'daemon off;'" ]
