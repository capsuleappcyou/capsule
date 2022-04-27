FROM debian:stable

RUN apt-get update && apt-get install -y nginx git fcgiwrap

RUN mkdir -p /srv/capsule/git && chown -R www-data:www-data /srv/capsule/git && mkdir -p /etc/capsule/
RUN ln -sf /dev/stdout /var/log/nginx/access.log && ln -sf /dev/stderr /var/log/nginx/error.log

COPY git-server/default.conf /etc/nginx/sites-available/default
COPY git-server/start.sh /start.sh

RUN ["chmod", "+x", "/start.sh"]
ENTRYPOINT ["/start.sh"]
