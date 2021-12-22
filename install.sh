OS=$(uname -s)
MACHINE=$(uname -m)
VERSION=$(curl -s https://api.github.com/repos/mattblack85/astro_monitor/releases/latest | grep -oP '"tag_name": "\K(.*)(?=")')
PACKAGE_NAME=astromonitor
EXTENSION=tar.gz

case $OS in
    Linux)
	case $MACHINE in
	    x86_64)
		ARCH=amd64
		;;
	    armv7)
		ARCH=$OS;
		;;
	    armv6)
		ARCH=$OS
	esac
	;;
    Darwin)
	ARCH=macos
	;;
    *)
	echo "OS not supported, please raise an issue at https://github.com/MattBlack85/astro_monitor/issues giving the result of `uname -s`"
	;;
esac


wget https://github.com/MattBlack85/astro_monitor/releases/download/$VERSION/$PACKAGE_NAME-$ARCH-$VERSION.$EXTENSION
tar -xvzf astromonitor*.tar.gz
rm astromonitor*tar.gz
sudo mv astromonitor /usr/bin/
