---
title: Troubleshooting
summary: What to do when something doesn't work, and the labels that tell you why.
group: Help
order: 20
---

Whenever Trindle stops you doing something, or closes a page by itself, it tells you why. The reason appears in a label right where you are working. Read that label first.

![A drawn diagram of three steps: read the label, look at Troubleshooting, and if you're still stuck, write a report.](img/help-path.svg "What you will see: the order to try things in. This is a drawing, not a screenshot.")

This page covers the common problems.

## The computer will not start from the USB stick

1. Plug in the stick, switch on the computer and hold the boot menu key. It's F12 on most Lenovo and Dell computers, F9 on HP, and Escape or F11 on others.
2. If Secure Boot is on, switch it off in the firmware settings.
3. Write the image to the stick again. Each time you do, it wipes everything on the stick.

## Trindle feels very slow in a virtual machine

A virtual machine is a pretend computer that runs inside a real one. If it has little memory and no 3D graphics, everything runs very slowly.

- Give it at least 4 GB of memory, or 8 GB if you can.
- Give it at least 2 processors.
- Switch 3D acceleration on.

Trindle skips its animations when there are no 3D graphics. The login screen has a Light drawing switch, which is on to begin with.

## The installer cannot share the disk

- Back up the other system first.
- Shut Windows down fully instead of letting it hibernate. If you don't, the installer can't make its partition smaller.
- If a disk has no boot partition to share, the installer can't put Trindle beside what is already on it. It tells you so.

## The ring has gone

If the ring stops working, a label with a small alert sign appears in the middle of the screen. The label reads: "The ring stopped working. Press Super to start it again." Press Super.

![The ring open on a dark screen, with a clock in the middle and small round buttons around it.](img/ring.png "What you will see: the ring, as it looks when it is working again.")

## I cannot open the ring by double-clicking the edge

Over a full-screen window, double-clicking never opens the ring. That way the ring can't interrupt a game or film. Press Super instead.

Anywhere else, double-click right on the edge of the screen, not a little way in from it.

- [Read about the ring](ring.html)

## A button in the ring did nothing

Important buttons respond only when you press them yourself. That includes Power, Permissions, installing in Apps, and the windows for saving and opening files. A program pretending to be you can't switch things off or take your files.

If anything but you pressed a power button, the label under the ring reads: "Only your own press can shut down this computer." Press the button yourself.

## A key does not do what I expect

You can change any key or mouse button in Settings, under Keyboard and mouse. A change applies everywhere.

If you've locked yourself out of your own keyboard, press Ctrl, Alt and an F key. That opens a text console, which always works. Type `trindle-settings --keys default` there to put every key back.

## The screen changed and looks wrong

- If you change the screen size and then do nothing, Settings puts the old size back after fifteen seconds.
- If a new theme turns out to be unreadable, Trindle goes back to your old look after 30 seconds, unless you press Keep.

## I cannot find an app in Discover

The first time you open Apps, the list takes about a minute to load. Discover waits for the list to arrive.

If you have no internet, Discover tells you and tries again by itself. Installed, Updates, Plugins, Software sources, Move your apps and the settings for apps all work without the internet.

![The Installed page of Apps with tabs across the top and an empty list.](img/apps-installed.png "What you will see: the Installed page, which works without the internet.")

## An update is waiting and will not install

Install all in Apps never installs a plugin update that asks for something new. The plugin waits for you, and a label says so. Its row lists exactly what is new. Open the row and choose whether to allow it.

## A plugin does not work

A plugin you haven't allowed yet shows "needs your permission to work" at the top of its card. It has one button, Allow. Press it and switch on each thing the plugin asks for.

- [Read about Apps](apps.html)

## An app will not remove

You can't remove Trindle's own parts, or anything the computer needs to run. The menu tells you why. For everything else, you press Remove twice. Press it the second time within three seconds.

## A wallpaper is left out

Wallpapers can be up to 4K (3840 by 2160). A still picture can be up to 20 MB and a moving one up to 200 MB. Trindle leaves out a bigger file as soon as you pick it, and a label tells you why.

## A drive change did nothing

Every change to a drive needs an administrator's password. If you close the password box, a label reads: "No password was given, so nothing changed." If a change stopped halfway, the row lists which steps were done and which weren't.

![The Drives window showing a disk with its parts as a bar, and a list of drives.](img/drives.png "What you will see: the Drives window. Each row shows what was done.")

- [Read about Drives](drives.html)

## A copy in Folders is slow

While a big copy or move is running, a quiet line tells you that copying puts a heavy load on the disk. Other programs may feel slower. That stops when the copy finishes.

## A shared folder asks for a login again

Folders remembers the login for a shared folder for 90 days. It sends the login only to the address you used before. If the computer answers at a new address, Folders asks for the login again and tells you why.

## An nfs:// address does not open

Trindle opens NFS shares using version 4 only. Only very old NAS boxes use version 3. If the server doesn't offer version 4, a label under the files reads "does not offer NFS version 4". Switch version 4 on in the NAS's own settings, then try again. If a server has no version 4 at all, Trindle can't open it.

## The screenshot tool froze

Trindle closes it for you, and a label tells you so. The Try again button starts it again.

## Still stuck

Write what happened in the Report a problem window and press Send. The report contains only your words, part of Trindle's log, the Trindle version and a summary of the computer. Trindle blanks out your names before it shows you the report, and nothing is sent until you press Send.

- [Read about Report a problem](report-a-problem.html)
