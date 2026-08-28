<p align="center">
     <a href="https://github.com/TheUnknownGroup/UKMCL">
          <img src="assets/images/ukmcl-img.svg" alt="UKMCL" width="450px">
     </a>
</p>

<p align="center">This is the official Rustified Minecraft Launcher for & by the Unknown Group.</p>
     
---
## Windows Users
If you choose to download the [windows](https://github.com/TheUnknownGroup/UKMCL/releases/latest) version, please know it will be caught by Microsoft's SmartScreen/Defender. It isn't a virus, everything the app creates in your system (for example: [here]()) to your system is deleted via the script that is bundled in the installer. If you truly don't believe the app is safe to install, I understand your decision and you're welcome to download the source code and build it yourself or completely ignore this app at all.

## Linux Users
If you choose to download the [linux](https://github.com/TheUnknownGroup/UKMCL/releases/latest) version, please know that it won't be flagged, but it will ask that you use sudo to install it. Please look [here]() for more information. 

## macOS Users
If you choose to download the [macOS](https://github.com/TheUnknownGroup/UKMCL/releases/latest) version, I don't know exactly what will happen, if it gets flagged by your antivirus or other type of app, please know that this is not a virus, it does not contain any malicious code or malware, please refer [here]() for more information on how to delete any file that is still left over after uninstalling.

## What does this add to my computer?
- If you're on Windows, your set up will look similarly to this: ``C:\Users\(your username)\.ukmcl``, along with its contents. If you choose to uninstall the script will delete the parent folder automatically at ``.ukmcl``, after that nothing else remains.

- If you're on Linux, your set up will look similarly to these file paths: ``/home/(your username)/.ukmcl``, and ``/usr/share/ukmcl``, and a few things in ``/var/lib/dpkg/info/``[*](). The second file path is where the app houses its inital launch script of the app, in the case of the app being uninstalled, that script and its parent folder (ukmcl) will be deleted automatically. The files in the /var/lib/ file path are deleted as well using the post removal script. 

- If you're on macOS, your set up will look similarly to this: ``/Users/(your username)/.ukmcl/``. I'm not completely sure as I personally do not have a Mac/Macbook to test this on. By my understanding there are no files nor scripts I can add to hold the uninstall process accountable for, so in the event that you choose to uninstall the app, please double check that every file has been deleted.

##### /*/
1. In the case you're on Debian / Ubuntu-based distros, it most likely will use dpkg to install the app.