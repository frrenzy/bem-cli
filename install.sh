#!/bin/zsh

set -e

URLS=("${(@f)$(curl -s https://api.github.com/repos/frrenzy/bem-cli/releases/latest \
                                 | grep browser_download_url \
                                 | cut -d : -f 2,3 \
                                 | tr -d \" \
                                 | awk '{print substr($0, 2)}')}")

ABOBA="${URLS[@]:0:1}"
MASK="${ABOBA%_*}"

OS=$(uname -a | awk '{print $1}')
ARCH=$(uname -a | awk '{print $NF}')

if [[ $OS == "Darwin" ]]
then
  if [[ $ARCH == "arm64" ]]
  then
    URL="${MASK}_m1"
    NAME="bem_m1"
  else
    URL="${MASK}_intel"
    NAME="bem_intel"
  fi
else
  URL="${MASK}_deb"
  NAME="bem_deb"
fi

wget "$URL"

chmod a+rx $NAME
rm -rf /usr/local/bin/bem
mv $NAME /usr/local/bin/bem
export PATH=/usr/local/bin/bem:$PATH
echo "export PATH=/usr/local/bin/bem:$PATH" >> ~/.zshrc
