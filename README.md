# gwater-installer
**gwater** installer but doesn't error out when you have more than one drive

## pros
1. it can actually work with multiple drives
2. it runs on linux (if you run garry's mod under proton)
3. rust lol

## cons
1. it can't auto-detect the gmod folder but ehh auto-detect doesn't really work and there's no reliable way to implement it (since steam path is not in env variables on windows) so whatever
2. ugly ass design
3. release version is about two times heavier than the OG installer, probably because of its massive reqwest dependency, but, if you want to, you can actually make the executable [smaller](https://github.com/johnthagen/min-sized-rust)

## FAQ
**Q:** where is my gmod folder?

**A:** go into the properties of your game -> go to "Local Files" and then click "Browse...". it will open the garry's mod folder in your file manager. note the path of the directory that just opened and browse to it while using the installer. *typically* it's `C:\Program Files\Steam\steamapps\common\GarrysMod`

**Q:** alright, but like, what is the point of all this?

**A:** i created this installer mostly because i wanted to learn egui and also because the old installer had a bug if you had more than one drive on your system, where it couldn't locate your garry's mod folder, even if you explicitly told it where it was

so, in conclusion, should you use this instead of a default launcher?
i dunno man, whatever you want
