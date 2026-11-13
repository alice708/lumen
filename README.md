This currently only works on Tapo L630 bulbs you have already connected to with the TP-Link Tapo app. 

To use create a `.env` file in the project root with the username and password for your Tapo app and the ip address of a bulb:

```
TAPO_USERNAME=
TAPO_PASSWORD=
IP_ADDRESS=
```

To find a bulbs IP address:
 - In the tapo app press on a specific bulb
 - Press the hexagon settings icon in the top left
 - Then press device info and you'll find the ip address




# Debian / Ubuntu / Mint
sudo apt install pkg-config libfontconfig1-dev

# Fedora
sudo dnf install pkgconf-pkg-config fontconfig-devel

# Arch
sudo pacman -S pkgconf fontconfig

# openSUSE
sudo zypper install pkg-config fontconfig-devel