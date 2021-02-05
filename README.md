<h1 align="center">
 <img src="https://user-images.githubusercontent.com/45159366/107091607-eb0ae280-67b6-11eb-91af-c052daa76876.png">
  <br />
  Pop!_OS Guide
</h1>

#### A guide on setting up your Pop!_OS Desktop with all the essential Applications, Tools, and Games to make your experience with Pop!_OS great!

# Table of Contents

1. [Getting Started](https://github.com/mikeroyal/Pop!_OS-Guide/blob/main/README.md#getting-started)

2. [Getting Software](https://github.com/mikeroyal/Pop!_OS-Guide/blob/main/README.md#getting-software)

3. [Gaming](https://github.com/mikeroyal/Pop!_OS-Guide/blob/main/README.md#gaming)

4. [Setting up a macOS workspace](https://github.com/mikeroyal/Pop!_OS-Guide/blob/main/README.md#setting-up-macos-workspace)

5. [Setting up a Windows 10 workspace](https://github.com/mikeroyal/Pop!_OS-Guide/blob/main/README.md#setting-up-windows-10-workspace)

6. [GNOME Extensions](https://github.com/mikeroyal/Pop!_OS-Guide/blob/main/README.md#gnome-extensions)

7. [Advanced Topics](https://github.com/mikeroyal/Pop!_OS-Guide/blob/main/README.md#advanced-topics)


# Getting Started

[Pop!_OS](https://pop.system76.com/) is an operating system(based on [Ubuntu](https://ubuntu.com/)) for STEM and creative professionals who use their computer as a tool to discover and create developed [System76](https://system76.com/).

[Popsicle](https://github.com/pop-os/popsicle) is a Linux utility for flashing multiple USB devices in parallel, written in Rust.

[Etcher](https://www.balena.io/etcher/) is an open source, cross-platform software that makes it easy to flash operating system images to a microSD card or USB device.

[Differences between Pop!_OS and Ubuntu](https://support.system76.com/articles/difference-between-pop-ubuntu/)

[Pop!_OS GitHub](https://github.com/pop-os)

[Pop!_OS reddit](https://www.reddit.com/r/pop_os/)


 <img src="https://user-images.githubusercontent.com/45159366/107091581-e0e8e400-67b6-11eb-8357-51bf416bfb1d.png">


## GNOME Tweaks

```sh
Open the terminal and run: 
sudo apt install gnome-tweak-tool //let's you customize your desktop layout.
```

## Enable Firewall

```sh
Open the terminal and run: 
sudo ufw enable  //enables ubuntu firewall
sudo ufw status //checks status of firewall
```
# Getting Software

[Back to the Top](https://github.com/mikeroyal/Pop!_OS-Guide/blob/main/README.md#table-of-contents)

## Pop Shop

<img src="https://user-images.githubusercontent.com/45159366/107091582-e21a1100-67b6-11eb-9043-1385d2d8afd9.png">

**Note 1: All this software is also available in other popular Linux distributions such as [Debian](https://www.debian.org/), [Linux Mint](https://linuxmint.com/), [elementary OS](https://elementary.io/), [Fedora](https://getfedora.org), [Manjaro Linux](https://manjaro.org/), [EndeavourOS](https://endeavouros.com/) and [Arch Linux](https://archlinux.org/).**

**Note 2: For new users not comfortable with using the command-line or need software not available in the Pop Shop checkout the Essential Apps section to get started. Also, if you scroll down further you'll see other easy ways to get software applications through Flathub, Snap Store, and AppImages.**

## Essential Apps(depending on your workflow)

[Google Chrome browser](https://www.google.com/chrome/)

[Microsoft Edge browser](https://www.microsoftedgeinsider.com/en-us/download/?platform=linux)

[Visual Studio Code](https://code.visualstudio.com/Download)

[Microsoft Teams](https://www.microsoft.com/en-us/microsoft-teams/download-app)

[Microsoft 365 with Office apps](https://www.microsoft.com/en-us/microsoft-365?legRedir=default&CorrelationId=335c4ab6-175d-4c4f-888d-15cfd03e4d32)

[Google Workspace (formerly G Suite)](https://workspace.google.com/)

[Zoom](https://zoom.us/download?os=linux)

[Slack](https://slack.com/downloads/linux)

[Trello](https://trello.com/platforms)

[Skype](https://www.skype.com/en/get-skype/)

[Spotify](https://www.spotify.com/us/download/linux/)

[Discord](https://discord.com/download)

[TeamViewer](https://www.teamviewer.com/en/download/linux/)

[VMware Workstation Player](https://www.vmware.com/products/workstation-player/workstation-player-evaluation.html) is an ideal utility for running a single virtual machine on a Windows or Linux PC. Organizations use Workstation Player to deliver managed corporate desktops, while students and educators use it for learning and training.

[VMware Workstation Pro](https://www.vmware.com/products/workstation-pro.html) is the industry standard for running multiple operating systems as virtual machines (VMs) on a single Linux or Windows PC. IT professionals, developers and businesses who build, test or demo software for any device, platform or cloud rely on Workstation Pro.

[CrossOver Linux®](https://www.codeweavers.com/crossover) is a Microsoft Windows compatibility layer(based on [WINE(Wine Is Not an Emulator)](https://www.winehq.org)). The CrossOver compatibility layer enables thousands of Windows-based applications to run on Linux, macOS, or Chrome OS.

[WinApps for Linux](https://github.com/Fmstrat/winapps) is a program that runs Windows apps such as Microsoft Office & Adobe in Linux (Ubuntu/Fedora) and GNOME/KDE as if they were a part of the native OS, including Nautilus integration for right clicking on files of specific mime types to open them.

[DaVinci Resolve video editor](https://www.blackmagicdesign.com/products/davinciresolve/) is complete video editing solution that combines professional 8K editing, color correction, visual effects and audio post production all in one software tool.

[Reaper Audio editor](https://www.reaper.fm/download.php) is a complete digital audio production application for computers, offering a full multitrack audio and MIDI recording, editing, processing, mixing and mastering toolset.

[Flameshot](https://flameshot.org/) is a powerful yet simple to use screenshot software.

[Stacer](https://github.com/oguzhaninan/Stacer) is an open source system optimizer and application monitor that helps users to manage their entire system. Also available as an AppImage.

[Nativefier](https://github.com/nativefier/nativefier) is a command-line tool to easily create a desktop app for any web site with minimal configuration. Apps are wrapped by [Electron](https://www.electronjs.org/) (which uses Chromium under the hood) in an OS executable (.app, .exe, etc) for use on Windows, macOS and Linux.


## App Outlet

[App Outlet](https://app-outlet.github.io/) is a Universal application store(Flatpaks, Snaps, and AppImages) inspired by the Linux App Store online service.

 <img src="https://user-images.githubusercontent.com/45159366/106686354-0095c780-657f-11eb-892b-659d3252d6e7.png">
 
 ## Flatpaks

[FlatHub](https://flathub.org/) is a build and distribution service for Flatpak applications.

[FlatHub Forum](https://discourse.flathub.org/)

 <img src="https://user-images.githubusercontent.com/45159366/106686365-055a7b80-657f-11eb-9b58-1de28abe2e5b.png">
 
 ## Snaps

[Snap Store](https://snapcraft.io/store) is a build and distribution service for Snap applications.

[Snapcraft Forum](https://forum.snapcraft.io/)

 <img src="https://user-images.githubusercontent.com/45159366/106686375-08ee0280-657f-11eb-9918-5385d8c09148.png">
 <img src="https://user-images.githubusercontent.com/45159366/106686378-0a1f2f80-657f-11eb-83aa-37ac96c7b032.png">

 
## AppImages

[AppImageHub](https://www.appimagehub.com) is a build and distribution service for AppImage applications.

[AppImage Manager](https://github.com/AppImageCrafters/appimage-manager) is a package manager for AppImages.

[AppImage Forum](https://discourse.appimage.org/)

 <img src="https://user-images.githubusercontent.com/45159366/106686382-0b505c80-657f-11eb-9d74-9a94ec0d0693.png">
