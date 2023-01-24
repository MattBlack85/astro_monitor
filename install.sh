OS=$(uname -s)
MACHINE=$(uname -m)
VERSION=$(curl -s https://api.github.com/repos/mattblack85/astro_monitor/releases/latest | grep -oP '"tag_name": "\K(.*)(?=")')
PACKAGE_NAME=astromonitor
EXTENSION=tar.gz

case $OS in
    Linux)
	case $MACHINE in
	    x86_64)
		ARCH=linux-amd64
		;;
	    armv7l)
		ARCH=armv7
		;;
	    aarch64)
		ARCH=arm64
	esac
	;;
    Darwin)
	case $MACHINE in
	    x86_64)
		ARCH=macos-x64
		;;
	    arm64)
		ARCH=macos-arm64
		;;
	esac
	;;
    *)
	echo "OS not supported, please raise an issue at https://github.com/MattBlack85/astro_monitor/issues giving the result of `uname -s` and `uname -m`"
	;;
esac


wget https://github.com/MattBlack85/astro_monitor/releases/download/$VERSION/$PACKAGE_NAME-$ARCH-$VERSION.$EXTENSION
tar -xvzf astromonitor*.tar.gz
rm astromonitor*tar.gz
sudo mv astromonitor /usr/local/bin/
