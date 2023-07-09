#!/usr/bin/env bash

set -eux

apt-get update -qq
apt-get install -qq -y make docker.io git sudo curl jq libssl-dev clang pkg-config

id unagi || useradd -m -s /bin/bash -G sudo -G docker -G adm unagi
mkdir -p /home/unagi/.ssh
for github_id in imos chokudai wata-orz sulume toslunar iwiwi; do
    curl -s "https://github.com/${github_id}.keys" >> /home/unagi/.ssh/authorized_keys;
done
sort -u /home/unagi/.ssh/authorized_keys > /home/unagi/.ssh/authorized_keys.tmp
mv /home/unagi/.ssh/authorized_keys.tmp /home/unagi/.ssh/authorized_keys
chown -R unagi:unagi /home/unagi/.ssh
chmod 700 /home/unagi/.ssh
chmod 600 /home/unagi/.ssh/authorized_keys
for user in imos chokudai wata sulume toslunar iwiwi; do
    mkdir -p /home/unagi/${user}
    chown unagi:unagi /home/unagi/${user}
done
sudo -u unagi bash -c "curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh"
