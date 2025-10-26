#
# ~/.bash_profile
#

export ANDROID_SDK_ROOT=$HOME/Android/Sdk
export ANDROID_HOME=$HOME/Android/Sdk
export PATH=$PATH:$ANDROID_SDK_ROOT/emulator
export PATH=$PATH:$ANDROID_SDK_ROOT/platform-tools

export PATH=/home/kalab/.local/share/solana/install/active_release/bin:$PATH

#some theme staf
export QT_QPA_PLATFORM="xcb"
#export QT_PLATFORMTHEME="qt5ct"
export QT_PLATFORM_PLUGIN="qt6ct"
#export QT_AUTO_SCREEN_SCALE_FACTOR=0
#export QT_SCALE_FACTOR=1
#export QT_STYLE_OVERRIDE=kvantum
#Startx Automatically
if [[ -z "$DISPLAY" ]] && [[ $(tty) = /dev/tty1 ]]; then
	. startx
	logout
fi

[[ -f ~/.bashrc ]] && . ~/.bashrc
