# bunkers

*bunkers* is a CLI tool to create, manage, and mount **LUKS-encrypted disk images**

## usage

### creating a bunker

to create a bunker, run the following:

```shell
bunkers create <name> <size in megabytes>
```

this may prompt for your sudo/doas password.
you will be asked to type `YES` in all-caps, this is to provide confirmation to `cryptsetup` that you want to format the image.

this will prompt you to enter a password *three times*.
> the first two are to create the password.
> the last one is to unlock the image so that it can be formatted.

i recommend that you store the password with `pass` (password-store).
> it should be saved under `bunkers/<bunker name>`

### mounting a bunker

to mount a bunker, run:

```shell
bunkers mount <name>
```

you will be asked for your sudo/doas password.

this will first check `pass show bunkers/<bunker name>` for the password.
if the password is not found there, or an error is thrown, you will be asked to input the password manually.

the bunker will then be mounted at `~/.bunkers/_mount/<name>`

### unmounting a bunker

to unmount a bunker, run:

```shell
bunkers umount <name>
```

you will be asked for your sudo/doas password.

## installation

the recommended method of installation is through [wares](https://github.com/indium114/wares).

to install, simply add the following to your `config.yaml` file:

```yaml
wares:
  bunkers:
    name: bunkers
    repo: indium114/bunkers
    asset: "bunkers_Linux_x86_64"
```
