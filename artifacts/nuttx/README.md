# How to build NuttX

## Install kconfig-frontends

```
$ git clone https://bitbucket.org/nuttx/tools.git
$ cd tools/kconfig-frontends/
$ ./configure
$ make
$ sudo make install
$ sudo /sbin/ldconfig
```

## Getting NuttX

```
$ mkdir nuttx
$ cd nuttx/
$ git clone https://bitbucket.org/nuttx/nuttx.git
$ git clone https://bitbucket.org/nuttx/apps.git
```

## Configuration

```
$ cd nuttx/
$ ./tools/configure.sh hifive1-revb:nsh
```

## Editing `./defconfig` file and Make

```
Delete CONFIG_ARCH_CHIP_FE310_G002=y
Add CONFIG_ARCH_CHIP_FE310_QEMU=y
```

```
$ make
```

# Links

 - [Nuttx](https://bitbucket.org/nuttx/nuttx/src/master/)
