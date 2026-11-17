This currently only works on Tapo L630 bulbs you have already connected to with the TP-Link Tapo app. 

To use create a `.env` file in the project root with the username and password for your Tapo app and the ip addresses of your bulbs:

```
TAPO_USERNAME=
TAPO_PASSWORD=
BULB_IPS=<bulb 1 IP>,<bulb 2 IP>,...
```

To find a bulbs IP address:
 - In the tapo app press on a specific bulb
 - Press the hexagon settings icon in the top left
 - Then press device info and you'll find the ip address
