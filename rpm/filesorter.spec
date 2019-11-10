%global debug_package %{nil}

%global commit      27689b5ceda5e838000ab39317582ec4f3cd0d2b
%global shortcommit %(c=%{commit}; echo ${c:0:7})
%global date        20191110

Name:           filesorter
Version:        0.1.0
Release:        1.%{date}git%{shortcommit}%{?dist}
Summary:        Utility for sorting files in directory writen in Rust

License:        GPLv3
URL:            https://github.com/ElXreno/filesorter
Source0:        %{url}/archive/%{commit}/%{name}-%{version}.%{date}git%{shortcommit}.tar.gz

ExclusiveArch:  %{rust_arches}

BuildRequires:  rust-packaging
#Requires:

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
* Sun Nov 10 2019 ElXreno <elxreno@gmail.com>
- Initial packaging
