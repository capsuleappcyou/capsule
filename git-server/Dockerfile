FROM ubuntu:22.04
RUN apt-get update && apt-get install -y nginx git fcgiwrap apache2-utils
RUN mkdir -p /capsule/git && chown -R www-data:www-data /capsule/git && mkdir -p /var/capsule/git/
COPY git-server/default.conf /etc/nginx/sites-available/default
VOLUME ["/capsule/git","/var/capsule/git/"]
ENTRYPOINT ["nginx","-g","daemon off;"]