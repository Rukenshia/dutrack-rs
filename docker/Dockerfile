FROM ruken/dutrack-build
MAINTAINER Jan Christophersen <jan@ruken.pw>

WORKDIR /dutrack
ADD dutrack-rs ./dutrack
ADD templates ./templates
ADD assets ./assets
ADD .env ./.env
ADD migrations ./migrations
ADD setup.sh ./setup.sh

RUN chmod +x dutrack
RUN ["/bin/bash", "./setup.sh"]

EXPOSE 8000

ENTRYPOINT service postgresql start && service redis-server start && sleep 10; find /var/log/postgresql -exec cat {} \; && ./dutrack