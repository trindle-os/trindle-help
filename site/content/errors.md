---
title: Errors
summary: What the words of an error mean, why it happened and how to fix it, for the errors an update or an install can give you.
group: Help
order: 15
---

When something fails, Trindle shows the words the system gave. A quiet More button beside them opens the section here that fits those words. Each section explains what the error means, the likely causes and the steps to fix it. It also shows the exact error message, so you can copy it into a report.

![A drawn diagram of three steps. Red words say what failed. You press More beside them. Help opens at the section for those words.](img/errors-path.svg "What you will see: the way from red words to the fix. This is a drawing, not a screenshot.")

## A package would not configure
Questions: what does dpkg error processing package mean; why does it say errors were encountered while processing; what does subprocess installed post-installation script returned error mean; why does an update stop on one package; what does dpkg was interrupted mean; why is a package half installed

What it means: Debian unpacked the package, but the last step, which sets it up, failed. Debian stops there and leaves the package half done. Every update after that stops at the same place until you put it right.

![The section of Help called A package would not configure, opened from an error. At the top, above the steps, is the exact error message: dpkg: error processing package example-driver. A copy button sits beside it.](img/errors-package-configure.png "What you will see: the exact error message in a box with a copy button, then the steps.")

Likely causes:

- Something cut the update short, such as a power cut, a closed lid or a restart, so the setup step never ran.
- The package's own setup script failed. The lines above the failure in the error text name the file or service it couldn't handle.
- Another package it needs hasn't been set up yet.
- The disk was full, so the setup script couldn't write what it needed.

Steps to fix it:

1. Restart the computer, so that nothing is still using the package.
2. Open Apps, go to Updates, and press Update again. Debian carries on from where it stopped.
3. If it stops at the same package, look for the package's name in the error text. It comes after "processing package".
4. If the disk is nearly full, free up some space, then press Update again.
5. If it still fails, copy the error text and put it in a report. See [Report a problem](report-a-problem.html).

:::expert
To do the same repair by hand, open a terminal and run these as an administrator:

- `dpkg --configure -a` sets up every package that was left half done.
- `apt --fix-broken install` installs anything that is missing.
- `dpkg --audit` lists packages that still aren't set up.
:::

## A driver would not build for the new kernel
Questions: what does dkms bad return status for module build mean; why did a driver not build after an update; why does a new kernel fail to install; what does dkms autoinstall failed mean; why does my Wi-Fi or graphics driver stop after a kernel update; what does module build for kernel failed mean

What it means: some drivers, such as those for certain Wi-Fi chips, graphics cards and virtual machines, are built on your computer each time the kernel changes. DKMS is the part that does this. The build failed for the new kernel, so that kernel has no driver. Trindle still installs the kernel and the rest of the update. It tells you which driver failed and what won't work until a newer one arrives, such as the virtual camera.

![The section of Help called A driver would not build for the new kernel, opened from an error. At the top, above the steps, is the exact error message: a module build that returned a bad status. A copy button sits beside it.](img/errors-driver-build.png "What you will see: the exact error message in a box with a copy button, then the steps.")

Likely causes:

- The driver is older than the new kernel and does not build against it yet.
- The headers for the new kernel weren't installed, so the build had nothing to build against.
- The disk filled up during the build.
- Someone removed the build tools.

Steps to fix it:

1. If the update has only just finished, don't restart yet. Your current kernel still has its driver.
2. In Apps, go to Updates and press Update again. A newer driver that builds often arrives a day or two after a new kernel.
3. The error text names the driver after "dkms:" or "module". Make a note of its name.
4. If you restart and a device stops working, choose the earlier kernel from the boot menu (Advanced options). Everything works again on that kernel.
5. If you don't need the device, remove the driver in Apps, then press Update again.
6. Copy the error text and put it in a report. See [Report a problem](report-a-problem.html).

:::expert
- `dkms status` lists each driver and the kernels it is built for.
- The build log is at `/var/lib/dkms/<name>/<version>/build/make.log`. The first error in it is the real cause.
- `apt install linux-headers-$(uname -r)` installs the headers for the running kernel.
:::

## There is no space left
Questions: what does no space left on device mean; why does it say not enough disk space; why won't an update install because the disk is full; how do I free disk space for an update; what does ENOSPC mean

What it means: the disk, or the part of it that holds the system, has no room for what the update needs. The update stopped before writing anything half done, but a package may still be only half set up.

![The section of Help called There is no space left, opened from an error. At the top, above the steps, is the exact error message: a write error saying there is no space left on the device. A copy button sits beside it.](img/errors-no-space.png "What you will see: the exact error message in a box with a copy button, then the steps.")

Likely causes:

- The disk really is nearly full, with files in Downloads, old backups or large videos.
- Old kernels and package files from earlier updates have piled up.
- The system part of the disk is small, while your home folder has plenty of room.
- Many small files have used up all the file entries, which also makes the disk look full.

Steps to fix it:

1. Open Folders and look at your largest folders. Delete or move what you no longer need. Empty the Trash, because deleted files still use space until you do.
2. In Apps, open Installed and remove programs you don't use any more.
3. Press Update again. If a package was left half done, the first update after you free up space finishes it.
4. If the system part is the full one and your own files are small, say so in a report, and include the error text.

:::expert
- `df -h` shows each disk and how full it is, and `df -i` shows how many file entries are left.
- `apt clean` deletes downloaded package files that you no longer need.
- `apt autoremove --purge` removes old kernels and libraries that nothing needs.
:::

## The package database is locked
Questions: what does could not get lock var lib dpkg lock mean; why does it say another process is using it; why can't I install while an update runs; what does unable to acquire the dpkg frontend lock mean; why does apt say it can't get the lock

What it means: only one thing can change the installed programs at a time, so Debian sets a lock while it does. Something else has the lock, so Debian wouldn't make this change. Nothing was changed.

![The section of Help called The package database is locked, opened from an error. At the top, above the steps, is the exact error message: it couldn't get the lock, and it names the program that has it. A copy button sits beside it.](img/errors-locked.png "What you will see: the exact error message in a box with a copy button, then the steps.")

Likely causes:

- An update is already running. Apps, the automatic updater or another window may have started it.
- An earlier update is still finishing in the background just after you restarted.
- An update was cut short and left the lock behind. This is rare, because the lock goes away when the program that set it stops.

Steps to fix it:

1. Wait a few minutes. Updates in the background usually finish by themselves.
2. Close any other window that is installing or updating, then press the button again.
3. If it's still locked after ten minutes, restart the computer. A restart ends everything that has the lock.
4. If it's locked straight after a restart, copy the error text. It gives the number of the program that has the lock.

:::expert
- `fuser -v /var/lib/dpkg/lock-frontend` names the program that has the lock.
- Never delete the lock files by hand while that program is running. You could damage the package database.
:::

## A download failed
Questions: what does failed to fetch mean; why does it say hash sum mismatch; what does 404 not found mean in an update; why does it say some index files failed to download; what does temporary failure resolving mean; why do updates fail with could not resolve

What it means: Trindle couldn't download a file the update needs, or it got a different file from the one it expected. Nothing was installed from that file.

![The section of Help called A download failed, opened from an error. At the top, above the steps, is the exact error message: a failed download with a 404 Not Found. A copy button sits beside it.](img/errors-download.png "What you will see: the exact error message in a box with a copy button, then the steps.")

Likely causes:

- The internet connection dropped, is very slow, or hasn't started yet after waking up.
- Trindle couldn't look up a website name, because the network's name service didn't answer.
- The mirror, which is the server that stores the files, is busy, or is half way through updating its own lists. This causes "Hash Sum mismatch" and "File has unexpected size".
- "404 Not Found" means a newer file has replaced the one on your list.
- A network that changes web pages, such as a hotel or office network, sent back a page instead of the file.

Steps to fix it:

1. Check your connection by opening any website.
2. Wait a few minutes and press Update again. A mirror that was updating its lists is usually fine again quickly.
3. For "404 Not Found", press Update again in Apps. It downloads fresh lists first, so it finds the new file.
4. If the network asks you to log in first, log in, then try again.
5. If the same file keeps failing for more than a day, copy the error text into a report. See [Report a problem](report-a-problem.html).

:::expert
- `apt update` downloads the lists again and prints which address failed.
- `apt clean` removes half-downloaded files that would cause the "size mismatch" to happen again.
:::

## Updates were kept back
Questions: why does it say kept back; what does not all updates went in mean; why was the kernel not updated; why was firmware not updated; why does apt keep packages back

What it means: Debian installed what it safely could and left the packages named in the message as they were. Each one needs something that isn't there yet, such as a newer helper package, so changing it now would break something.

![The section of Help called Updates were kept back, opened from an error. At the top, above the steps, is the exact error message: the list of packages that were kept back. A copy button sits beside it.](img/errors-kept-back.png "What you will see: the exact error message in a box with a copy button, then the steps.")

Likely causes:

- The package needs a newer package that comes from a different place, and nothing has asked for that one yet.
- Debian's lists have changed since the last update, and one more update will sort it out.
- You are holding the package back yourself.

Steps to fix it:

1. Press Update in Apps again. Many packages install on the second try.
2. If the same packages are named again, copy the error text into a report. See [Report a problem](report-a-problem.html).
3. Meanwhile your computer keeps working as it is, and the older start-up kernel stays in the boot menu.

:::expert
- `apt-get -s --with-new-pkgs upgrade` shows what is kept back without changing anything.
- `apt-cache policy <name>` shows which version is wanted and where it comes from.
:::

## Packages have unmet dependencies
Questions: what does unmet dependencies mean; why does it say you have held broken packages; what does depends but it is not going to be installed mean; why will a package not install because another is missing; what does apt fix-broken install mean

What it means: a program needs another program, at a certain version, and that one is missing, too old or in the way. Debian won't make the change, so that your computer isn't left with a program that can't run.

![The section of Help called Packages have unmet dependencies, opened from an error. At the top, above the steps, is the exact error message, which names the package and the one it needs. A copy button sits beside it.](img/errors-dependencies.png "What you will see: the exact error message in a box with a copy button, then the steps.")

Likely causes:

- A package from outside Debian, such as a downloaded .deb file, asks for something Debian doesn't have, or asks for a different version of it.
- An update was cut short, so some parts are new and some are old.
- You are holding an update back, and something else needs the newer version.
- Debian's lists are out of date, so it looks for a version that has since been replaced.

Steps to fix it:

1. Press Update in Apps so that it downloads the lists again, then try again.
2. If you downloaded the program yourself, remove it in Apps, Installed, then try again.
3. Look at the updates you are holding back, and stop holding them back for now.
4. The error text names the package that can't be installed and what it needs. If the steps above didn't help, copy it into a report. See [Report a problem](report-a-problem.html).

:::expert
- `apt --fix-broken install` asks Debian to repair what it can.
- `apt-cache policy <name>` shows which versions are available and why Debian picks one.
:::

## An error with no page of its own
Questions: what do I do about an error that Help does not list; why does it say something I do not understand; what does an error mean when Help has no page for it; how do I report an error; what is the error text for

What it means: Help doesn't recognise this error by its words yet, so there's no page written for it. Trindle shows the words it or the system gave, exactly as they were, so you can search for them or send them with a report.

![The section of Help for an error with no page of its own, opened from an error. At the top, above the steps, is the exact error message, word for word. A copy button sits beside it.](img/errors-unlisted.png "What you will see: the exact error message in a box with a copy button, then the steps.")

Likely causes:

- It was a one-off, such as the connection dropping, or a program being busy at that moment.
- The disk, the connection or the password was the real problem, and the words only mention what failed because of it.
- A rare fault that nobody has come across yet.

Steps to fix it:

1. Read the error text. The first line that names a file, a program, a disk or an address is usually the one that matters.
2. Try once more. Many errors are only a brief problem.
3. Check the three common causes: the connection, the free space on the disk, and whether anything else is updating.
4. Press the round Copy button next to the error text and put the text in a report. See [Report a problem](report-a-problem.html). Say what you were doing when it happened.
