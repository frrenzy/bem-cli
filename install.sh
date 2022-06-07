#!/bin/zsh

URL=$(curl -s https://api.github.com/repos/frrenzy/bem-cli/releases/latest \
        | grep browser_download_url \
        | cut -d : -f 2,3 \
        | tr -d \" \
        | tr -d "[:space:]")

wget $URL
chmod +x bem
mv bem /usr/local/bin/bem