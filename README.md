cids
=====

ContentIdentifiers

usage
------

```console
cids 0.1.0
Hiromitsu Oshiba <h.oshiba.0117@gmail.com>
Content Identifiers

USAGE:
    cids [FLAGS] <target>

FLAGS:
    -d, --dir        Activate dir mode
    -h, --help       Prints help information
    -V, --version    Prints version information

ARGS:
    <target>    target
```

### file mode

```console
$ cids .cargo-lock
Path    SHA256  SHA1    MD5
.cargo-lock     e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855        da39a3ee5e6b4b0d3255bfef95601890afd80709        d41d8cd98f00b204e9800998ecf8427e
```

### dir mode

```console
$ cids -d .
Path    SHA256  SHA1    MD5
.\.cargo-lock   e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855        da39a3ee5e6b4b0d3255bfef95601890afd80709        d41d8cd98f00b204e9800998ecf8427e
.\.fingerprint\ansi_term-702bf4f5b6a96562\dep-lib-ansi_term-702bf4f5b6a96562    22b2d8e0497ff10563e54cb847cfc31e36310c69be344ed3b452e64657b75821        214be4eae7ad60ec4132588bbe5e5dc34bfffc7e        054fad327e372476d01a7d29239dd992
...
```

setup
------

```sh
git clone git@github.com:0x75960/cids && cd cids
cargo build --release
```
