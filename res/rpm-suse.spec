Name:       rustdesk
Version:    1.1.9
Release:    0
Summary:    RPM package
License:    GPL-3.0
Requires:   gtk3 libxcb1 xdotool libXfixes3 alsa-utils libXtst6 libvdpau1 libva2 pam gstreamer-plugins-base gstreamer-plugin-pipewire
Recommends: libayatana-appindicator3-1

%description
The best open-source remote desktop client software, written in Rust.

%prep
# we have no source, so nothing here

%build
# we have no source, so nothing here

%global __python %{__python3}

%install
mkdir -p %{buildroot}/usr/bin/
mkdir -p %{buildroot}/usr/lib/rustdesk/
mkdir -p %{buildroot}/usr/share/rustdesk/files/
mkdir -p %{buildroot}/usr/share/icons/hicolor/256x256/apps/
mkdir -p %{buildroot}/usr/share/icons/hicolor/scalable/apps/
install -m 755 $HBB/target/release/rustdesk %{buildroot}/usr/bin/lidesk
install $HBB/libsciter-gtk.so %{buildroot}/usr/lib/rustdesk/libsciter-gtk.so
install $HBB/res/lidesk.service %{buildroot}/usr/share/rustdesk/files/
install $HBB/res/128x128@2x.png %{buildroot}/usr/share/icons/hicolor/256x256/apps/rustdesk.png
install $HBB/res/scalable.svg %{buildroot}/usr/share/icons/hicolor/scalable/apps/rustdesk.svg
install $HBB/res/lidesk.desktop %{buildroot}/usr/share/rustdesk/files/
install $HBB/res/lidesk-link.desktop %{buildroot}/usr/share/rustdesk/files/

%files
/usr/bin/lidesk
/usr/lib/rustdesk/libsciter-gtk.so
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
    update-desktop-database
  ;;
  1)
    # for upgrade
  ;;
esac
