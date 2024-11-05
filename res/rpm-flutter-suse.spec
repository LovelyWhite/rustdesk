Name:       rustdesk
Version:    1.3.2
Release:    0
Summary:    RPM package
License:    GPL-3.0
Requires:   gtk3 libxcb1 xdotool libXfixes3 alsa-utils libXtst6 libvdpau1 libva2 pam gstreamer-plugins-base gstreamer-plugin-pipewire
Recommends: libayatana-appindicator3-1
Provides:   libdesktop_drop_plugin.so()(64bit), libdesktop_multi_window_plugin.so()(64bit), libfile_selector_linux_plugin.so()(64bit), libflutter_custom_cursor_plugin.so()(64bit), libflutter_linux_gtk.so()(64bit), libscreen_retriever_plugin.so()(64bit), libtray_manager_plugin.so()(64bit), liburl_launcher_linux_plugin.so()(64bit), libwindow_manager_plugin.so()(64bit), libwindow_size_plugin.so()(64bit), libtexture_rgba_renderer_plugin.so()(64bit)

%description
The best open-source remote desktop client software, written in Rust.

%prep
# we have no source, so nothing here

%build
# we have no source, so nothing here

# %global __python %{__python3}

%install

mkdir -p "%{buildroot}/usr/lib/rustdesk" && cp -r ${HBB}/flutter/build/linux/x64/release/bundle/* -t "%{buildroot}/usr/lib/rustdesk"
mkdir -p "%{buildroot}/usr/bin"
install -Dm 644 $HBB/res/lidesk.service -t "%{buildroot}/usr/share/rustdesk/files"
install -Dm 644 $HBB/res/lidesk.desktop -t "%{buildroot}/usr/share/rustdesk/files"
install -Dm 644 $HBB/res/lidesk-link.desktop -t "%{buildroot}/usr/share/rustdesk/files"
install -Dm 644 $HBB/res/128x128@2x.png "%{buildroot}/usr/share/icons/hicolor/256x256/apps/rustdesk.png"
install -Dm 644 $HBB/res/scalable.svg "%{buildroot}/usr/share/icons/hicolor/scalable/apps/rustdesk.svg"

%files
/usr/lib/rustdesk/*
/usr/share/rustdesk/files/lidesk.service
/usr/share/icons/hicolor/256x256/apps/rustdesk.png
/usr/share/icons/hicolor/scalable/apps/rustdesk.svg
/usr/share/rustdesk/files/lidesk.desktop
/usr/share/rustdesk/files/lidesk-link.desktop

%changelog
# let's skip this for now

# https://www.cnblogs.com/xingmuxin/p/8990255.html
%pre
# can do something for centos7
case "$1" in
  1)
    # for install
  ;;
  2)
    # for upgrade
    systemctl stop lidesk || true
  ;;
esac

%post
cp /usr/share/rustdesk/files/lidesk.service /etc/systemd/system/lidesk.service
cp /usr/share/rustdesk/files/lidesk.desktop /usr/share/applications/
cp /usr/share/rustdesk/files/lidesk-link.desktop /usr/share/applications/
ln -s /usr/lib/rustdesk/lidesk /usr/bin/lidesk
systemctl daemon-reload
systemctl enable lidesk
systemctl start lidesk
update-desktop-database

%preun
case "$1" in
  0)
    # for uninstall
    systemctl stop lidesk || true
    systemctl disable lidesk || true
    rm /etc/systemd/system/lidesk.service || true
  ;;
  1)
    # for upgrade
  ;;
esac

%postun
case "$1" in
  0)
    # for uninstall
    rm /usr/share/applications/lidesk.desktop || true
    rm /usr/share/applications/lidesk-link.desktop || true
    rm /usr/bin/lidesk || true
    update-desktop-database
  ;;
  1)
    # for upgrade
  ;;
esac
