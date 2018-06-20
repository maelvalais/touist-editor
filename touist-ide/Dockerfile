FROM node:latest as node

RUN mkdir -p /usr/src/app
WORKDIR /usr/src/app

# Create a cache layer for yarn dependencies and speed up docker re-builds.
# The --pure-lockfile will prevent yarn from creating the lockfile; it will
# only read from the existing lock.
# From https://github.com/yarnpkg/yarn/issues/749#issuecomment-253253608
COPY package.json yarn.lock ./
RUN yarn --pure-lockfile

# Now, actually build. Don't forget to add useless files in .dockerignore
# (e.i., Dockerfile, node_modules), otherwise 'docker build' will rebuild
# layers over and over.
COPY . .

RUN yarn build

# We don't need anything from above except for the dist/ produced. We only
# need a static www server.
FROM nginx:latest
COPY --from=node /usr/src/app/dist /usr/share/nginx/html

# https://www.virtuallyghetto.com/2017/04/quick-tip-creating-a-multiline-dockerfile-using-heredoc-wvariable-substitution.html
RUN echo '\
server {\n\
  listen ${PORT};\n\
  server_name ${HOST};\n\
  location / {\n\
    root /usr/share/nginx/html;\n\
    index index.html index.htm;\n\
    if ($http_x_forwarded_proto = "http") {\n\
         return 301 https://$server_name$request_uri;\n\
    }\n\
  }\n\
}\n'\
>> /etc/nginx/conf.d/nginx.tmpl

ENV PORT=8080
ENV HOST=localhost
ENV API_URL=http://localhost:8000
EXPOSE ${PORT}

CMD ["/bin/bash", "-c", "sed -i s#%API_URL%#${API_URL}# /usr/share/nginx/html/static/js/app.*.js \
    && envsubst '$$PORT,$$HOST' < /etc/nginx/conf.d/nginx.tmpl > /etc/nginx/conf.d/default.conf \
    && cat /etc/nginx/conf.d/default.conf \
    && nginx -g \"daemon off;\""]
