ARG UBUNTU_VERSION=focal
ARG HTTP_PORT=8080

FROM ubuntu:focal
# need to repeat the variables after from since from consumes all args and they are not available afterwards
ARG HTTP_PORT

COPY /rustwebserver ./
COPY /entrypoint.sh ./


RUN apt update \
&& apt install -y openssh-server \
&& ssh-keygen -A \
&& chmod +x /rustwebserver \
&& chmod +x /entrypoint.sh

ENTRYPOINT ["/entrypoint.sh"]

EXPOSE $HTTP_PORT