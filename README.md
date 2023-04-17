# Bing 4K 壁纸

一个简单应用，用于自动获取并设置当日 Bing 图像为壁纸，适用于如下桌面系统：

- Windows
- macOS
- GNOME
- KDE
- Cinnamon
- Unity
- Budgie
- XFCE
- LXDE
- MATE
- Deepin
- Most Wayland compositors (set only, requires swaybg)
- i3 (set only, requires feh)

特性：可传入自定义的壁纸 URL（包括了一个远程壁纸 URL 地址），不传入参数则默认自解析以获取 4K 中国区壁纸源。当设置过当日壁纸后，下次调用程序将直接退出，以在移动设备（MacBook 等）设备配合 crontab 使用。

```bash
# parse get url from cn.bing.com
bing_wallpaper
# or download image from url
bing_wallpaper https://YOUR-WALLPAPER-URL-HERE
bing_wallpaper default-server
```