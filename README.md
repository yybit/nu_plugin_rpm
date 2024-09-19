[![crates.io](https://img.shields.io/crates/v/nu_plugin_rpm.svg)](https://crates.io/crates/nu_plugin_rpm)
[![docs.rs](https://docs.rs/nu_plugin_rpm/badge.svg)](https://docs.rs/nu_plugin_rpm)

## nu_plugin_rpm
A nushell plugin for reading rpm package. Require nushell >= `0.98.0`.

### Installation
```shell
cargo install nu_plugin_rpm
plugin add ~/.cargo/bin/nu_plugin_rpm
plugin use rpm
```

### Usage
```shell
# Simple use
open xxx.rpm
# Same result as above
open -r xxx.rpm | from rpm
# Display additional `files` field
open -r xxx.rpm | from rpm -f
```

### Example

```
> open curl-8.10.0-1.fc42.x86_64.rpm 
╭───────────────────────┬──────────────────────────────────────────────────────────────────────────────────────╮
│ name                  │ curl                                                                                 │
│ version               │ 8.10.0                                                                               │
│ release               │ 1.fc42                                                                               │
│ arch                  │ x86_64                                                                               │
│ size                  │ 453.3 KiB                                                                            │
│ vendor                │ Fedora Project                                                                       │
│ url                   │ https://curl.se/                                                                     │
│ vcs                   │                                                                                      │
│ license               │ curl                                                                                 │
│ summary               │ A utility for getting files from remote servers (FTP, HTTP, and others)              │
│ description           │ curl is a command line tool for transferring data with URL syntax, supporting        │
│                       │ FTP, FTPS, HTTP, HTTPS, SCP, SFTP, TFTP, TELNET, DICT, LDAP, LDAPS, FILE, IMAP,      │
│                       │ SMTP, POP3 and RTSP.  curl supports SSL certificates, HTTP POST, HTTP PUT, FTP       │
│                       │ uploading, HTTP form based upload, proxies, cookies, user+password                   │
│                       │ authentication (Basic, Digest, NTLM, Negotiate, kerberos...), file transfer          │
│                       │ resume, proxy tunneling and a busload of other useful tricks.                        │
│ group                 │ Unspecified                                                                          │
│ packager              │ Fedora Project                                                                       │
│ build_time            │ a week ago                                                                           │
│ build_host            │ buildvm-x86-15.iad2.fedoraproject.org                                                │
│ cookie                │                                                                                      │
│ source_rpm            │ curl-8.10.0-1.fc42.src.rpm                                                           │
│ pre_install_script    │                                                                                      │
│ post_install_script   │                                                                                      │
│ pre_uninstall_script  │                                                                                      │
│ post_uninstall_script │                                                                                      │
│ pre_trans_script      │                                                                                      │
│ post_trans_script     │                                                                                      │
│ pre_untrans_script    │                                                                                      │
│ post_untrans_script   │                                                                                      │
│                       │ ╭───┬──────────────┬───────┬───────────────╮                                         │
│ provides              │ │ # │     name     │ flags │    version    │                                         │
│                       │ ├───┼──────────────┼───────┼───────────────┤                                         │
│                       │ │ 0 │ curl         │ EQUAL │ 8.10.0-1.fc42 │                                         │
│                       │ │ 1 │ curl(x86-64) │ EQUAL │ 8.10.0-1.fc42 │                                         │
│                       │ │ 2 │ curl-full    │ EQUAL │ 8.10.0-1.fc42 │                                         │
│                       │ │ 3 │ curl-minimal │ EQUAL │ 8.10.0-1.fc42 │                                         │
│                       │ │ 4 │ webclient    │       │               │                                         │
│                       │ ╰───┴──────────────┴───────┴───────────────╯                                         │
│                       │ ╭────┬─────────────────────────────────────┬───────────────────────┬───────────────╮ │
│ requires              │ │  # │                name                 │         flags         │    version    │ │
│                       │ ├────┼─────────────────────────────────────┼───────────────────────┼───────────────┤ │
│                       │ │  0 │ libc.so.6()(64bit)                  │ FIND_REQUIRES         │               │ │
│                       │ │  1 │ libc.so.6(GLIBC_2.14)(64bit)        │ FIND_REQUIRES         │               │ │
│                       │ │  2 │ libc.so.6(GLIBC_2.15)(64bit)        │ FIND_REQUIRES         │               │ │
│                       │ │  3 │ libc.so.6(GLIBC_2.17)(64bit)        │ FIND_REQUIRES         │               │ │
│                       │ │  4 │ libc.so.6(GLIBC_2.2.5)(64bit)       │ FIND_REQUIRES         │               │ │
│                       │ │  5 │ libc.so.6(GLIBC_2.3)(64bit)         │ FIND_REQUIRES         │               │ │
│                       │ │  6 │ libc.so.6(GLIBC_2.3.4)(64bit)       │ FIND_REQUIRES         │               │ │
│                       │ │  7 │ libc.so.6(GLIBC_2.33)(64bit)        │ FIND_REQUIRES         │               │ │
│                       │ │  8 │ libc.so.6(GLIBC_2.34)(64bit)        │ FIND_REQUIRES         │               │ │
│                       │ │  9 │ libc.so.6(GLIBC_2.4)(64bit)         │ FIND_REQUIRES         │               │ │
│                       │ │ 10 │ libc.so.6(GLIBC_2.7)(64bit)         │ FIND_REQUIRES         │               │ │
│                       │ │ 11 │ libc.so.6(GLIBC_ABI_DT_RELR)(64bit) │ FIND_REQUIRES         │               │ │
│                       │ │ 12 │ libcurl(x86-64)                     │ GREATER | EQUAL       │ 8.10.0-1.fc42 │ │
│                       │ │ 13 │ libcurl.so.4()(64bit)               │ FIND_REQUIRES         │               │ │
│                       │ │ 14 │ rpmlib(CompressedFileNames)         │ LESS | EQUAL | RPMLIB │ 3.0.4-1       │ │
│                       │ │ 15 │ rpmlib(FileDigests)                 │ LESS | EQUAL | RPMLIB │ 4.6.0-1       │ │
│                       │ │ 16 │ rpmlib(PayloadFilesHavePrefix)      │ LESS | EQUAL | RPMLIB │ 4.0-1         │ │
│                       │ │ 17 │ rpmlib(PayloadIsZstd)               │ LESS | EQUAL | RPMLIB │ 5.4.18-1      │ │
│                       │ │ 18 │ rtld(GNU_HASH)                      │ FIND_REQUIRES         │               │ │
│                       │ ╰────┴─────────────────────────────────────┴───────────────────────┴───────────────╯ │
│ conficts              │ [list 0 items]                                                                       │
│                       │ ╭───┬──────────────┬───────┬─────────╮                                               │
│ obsoletes             │ │ # │     name     │ flags │ version │                                               │
│                       │ ├───┼──────────────┼───────┼─────────┤                                               │
│                       │ │ 0 │ curl-minimal │ LESS  │ 8.6.0-4 │                                               │
│                       │ ╰───┴──────────────┴───────┴─────────╯                                               │
│ recommends            │ [list 0 items]                                                                       │
│ suggests              │ [list 0 items]                                                                       │
╰───────────────────────┴──────────────────────────────────────────────────────────────────────────────────────╯
```