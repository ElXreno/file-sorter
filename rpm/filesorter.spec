%global debug_package %{nil}

Name:           filesorter
Version:        0.2.0
Release:        1%{?dist}
Summary:        Utility for sorting files in directory writen in Rust

License:        GPLv3
URL:            https://github.com/ElXreno/filesorter
Source0:        %{url}/archive/%{version}/%{name}-%{version}.tar.gz

ExclusiveArch:  %{rust_arches}

BuildRequires:  rust-packaging

%description
Configurable utility writen in Rust for simple sorting files in directory.


%prep
%autosetup -n %{name}-%{commit}


%build
cargo build --release


%install
cargo install --root=%{buildroot}%{_prefix} --path=.
rm -f %{buildroot}%{_prefix}/.crates.toml


%files
%license LICENSE
%doc README.md
%{_bindir}/filesorter



%changelog
* Mon Mar 16 2020 ElXreno <elxreno@gmail.com> - 0.2.0-1
- Updated to version 0.2.0

* Sun Nov 10 2019 ElXreno <elxreno@gmail.com> - 0.1.0-2.20191110git1bd3452
- Updated to 0.1.1 version

* Sun Nov 10 2019 ElXreno <elxreno@gmail.com> - 0.1.0-2.20191110git1bd3452
- Updated commit to 1bd34523dc7522018ce67bf8fabc0de5db5b0c84

* Sun Nov 10 2019 ElXreno <elxreno@gmail.com>
- Initial packaging
