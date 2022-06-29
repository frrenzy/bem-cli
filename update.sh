set -e

VERSION=$(curl -s https://api.github.com/repos/frrenzy/bem-cli/releases/latest \
                                         | grep tag_name \
                                         | cut -d : -f 2,3 \
                                         | tr -d \" \
                                         | tr -d \, \
                                         | tr -d v \
                                         | awk '{print substr($0, 2)}')

#URLS=("${(@f)$(curl -s https://api.github.com/repos/frrenzy/bem-cli/releases/latest \
#                                 | grep browser_download_url \
#                                 | cut -d : -f 2,3 \
#                                 | tr -d \" \
#                                 | awk '{print substr($0, 2)}')}")
#
#ABOBA="${URLS[@]:0:1}"
#MASK="${ABOBA%_*}"
#
#OS=$(uname -a | awk '{print $1}')
#ARCH=$(uname -a | awk '{print $NF}')
#
#if [[ $OS == "Darwin" ]]
#then
#  if [[ $ARCH == "arm64" ]]
#  then
#    URL="${MASK}_m1"
#    NAME="bem_m1"
#  else
#    URL="${MASK}_intel"
#    NAME="bem_intel"
#  fi
#else
#  URL="${MASK}_deb"
#  NAME="bem_deb"
#fi
#
#wget "$URL"
##
##chmod a+rx $NAME
##
##if [ -d "$HOME/bin" ]; then
##  rm -rf $HOME/bin/bem
##else
##  mkdir "$HOME/bin"
##fi
#
#mv $NAME "$HOME/bin/bem"

echo $VERSION
