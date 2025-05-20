@echo off
taskkill /IM wicked-waifus-config-server.exe /F
taskkill /IM wicked-waifus-hotpatch-server.exe /F
taskkill /IM wicked-waifus-login-server.exe /F
taskkill /IM wicked-waifus-gateway-server.exe /F
taskkill /IM wicked-waifus-game-server.exe /F
taskkill /IM cmd.exe /F
echo All servers and CMD windows have been terminated.
pause