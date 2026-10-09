FROM rust:bookworm

RUN apt-get update -y \
 && apt-get install fd-find ripgrep sudo \
 && useradd jeff \
 && usermod -aG sudo jeff \
 && echo 'jeff ALL=(ALL) NOPASSWD:ALL' > /etc/sudoers.d/jeff

USER jeff
WORKDIR /home/jeff
