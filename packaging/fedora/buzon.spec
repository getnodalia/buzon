%global appid dev.danielmiguelt.Buzon
# The payload is a binary built outside rpmbuild (tools/build-packages.sh),
# so there is nothing to extract debuginfo from.
%global debug_package %{nil}

Name:           buzon
Version:        0.1.0
Release:        1%{?dist}
Summary:        The email client of the Nodalia suite
License:        AGPL-3.0-or-later
URL:            https://getnodalia.com
Source0:        %{name}-%{version}-bin.tar

# Passwords and OAuth tokens are stored via the Secret Service D-Bus API
Recommends:     gnome-keyring
# The GNOME Files right-click entry (Settings -> System -> GNOME Files) is
# a Python extension; this is the loader Files needs for it.
Recommends:     nautilus-python

%description
Buzón is the email client of the Nodalia suite. It talks IMAP, POP3, JMAP and
SMTP directly, keeps your mail and credentials on your machine, and blocks
trackers by default: no telemetry, no analytics.

%prep
%setup -q -n %{name}-%{version}-bin

%install
install -Dm755 buzon %{buildroot}%{_bindir}/buzon
install -Dm644 %{appid}.desktop %{buildroot}%{_datadir}/applications/%{appid}.desktop
install -Dm644 %{appid}.metainfo.xml %{buildroot}%{_datadir}/metainfo/%{appid}.metainfo.xml
for size in 256x256 512x512; do
    install -Dm644 icons/$size/%{appid}.png \
        %{buildroot}%{_datadir}/icons/hicolor/$size/apps/%{appid}.png
done
install -Dm644 icons/scalable/%{appid}.svg \
    %{buildroot}%{_datadir}/icons/hicolor/scalable/apps/%{appid}.svg
install -Dm644 icons/symbolic/%{appid}-symbolic.svg \
    %{buildroot}%{_datadir}/icons/hicolor/symbolic/apps/%{appid}-symbolic.svg
# Message catalogues, staged by tools/build-packages.sh as
# locale/<lang>/LC_MESSAGES/buzon.mo; the binary looks under
# %{_datadir}/locale beside itself (src/i18n.rs).
for mo in locale/*/LC_MESSAGES/buzon.mo; do
    [ -e "$mo" ] || continue
    install -Dm644 "$mo" %{buildroot}%{_datadir}/"$mo"
done
%find_lang %{name}

%files -f %{name}.lang
%license COPYING
%{_bindir}/buzon
%{_datadir}/applications/%{appid}.desktop
%{_datadir}/metainfo/%{appid}.metainfo.xml
%{_datadir}/icons/hicolor/*/apps/%{appid}.png
%{_datadir}/icons/hicolor/scalable/apps/%{appid}.svg
%{_datadir}/icons/hicolor/symbolic/apps/%{appid}-symbolic.svg

%changelog
* Thu Nov 12 2026 Daniel Miguel Tejedor - 0.1.0-1
- First package of Buzón
