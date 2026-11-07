---
title: Apps
summary: Find, install, update and remove programs, and see what each one is allowed to do.
group: Using Trindle
order: 20
---

Trindle comes almost empty, with only the basics. Apps is where you add the rest. Open it from the ring.

![The Installed page of Apps. A list of tabs reads All, Flathub, Flatpak files, AppImages, Deb files and Plugins, with a search box under them. The list is empty.](img/apps-installed.png "What you will see: Apps, on the Installed page. Nothing has been added yet on this computer.")

## The pages

The side bar on the left has four sections.

- **Apps:** Discover, Installed, Updates and Plugins.
- **Settings for apps:** Default apps, Start at login and Permissions.
- **Categories:** Sound and video, Graphics and photos, Office, Internet, Development, Learning, Games, Science, Utilities and System.
- **More ways to get apps:** From GitHub, Software sources and Move your apps.

For Steam and games, see [Playing games](games.html).

## Finding and installing an app

1. Open Apps and choose Discover.
2. Look through the tiles, or type a name in the search box at the top.
3. Press a tile to read about the app first.
4. Press the round button at the tile's far edge. It says Install, or Open if the app is already there.

Discover shows apps from Flathub. Flathub is an online shop of apps that each run apart from the rest.

![Discover in Apps: a grid of app tiles, each with a name and a round button at its edge.](img/apps-discover.png "What you will see: Discover with made-up apps. Each tile has one round button.")

The first time Apps opens, the list of apps takes a minute to arrive. Until then Apps says so. Pages that do not need the list work at once. Those are Installed, Updates and Plugins.

## Everything you have added, in one list

Installed lists everything you added. That covers apps from Flathub, Flatpak files, AppImages, programs from `.deb` files, and plugins. What came with Trindle is left off, so the list is only what you chose.

Each row has its picture, its name, where it came from and its version. Remove is always the button furthest from the edge. It asks twice, so a slip never takes an app away. Press it again within three seconds.

To add an AppImage, press Add an AppImage at the top of the page. An AppImage is a program kept in one file.

## Apps that leave nothing behind

Each app you install lives in a folder of its own, with everything it needs.

![A drawn picture of a folder for one app, holding the app, its libraries and its settings, beside your own Documents and Pictures folders, joined only by a dotted line that means a permission.](img/app-folder.svg "What you will see: a diagram of an app in its own folder. It is drawn to explain, not a screenshot.")

Remove the app and it is gone completely. Nothing is left behind and nothing else breaks. A `.deb` you download from anywhere arrives in its own sealed box, with its own libraries, instead of spreading files across the system. Libraries are the shared pieces of code a program needs.

Only the parts the computer itself needs are installed for the whole system. Those are graphics, sound, networking and the login screen. Apps refuses drivers and background services, and says why in plain words.

## What an app may do

When you install an app, Trindle lists what it wants in plain words. For example, "your Documents and Pictures folders" or "your microphone". Each line says what happens without it. Each has its own switch, off until you choose.

If a request looks strange, it comes with a warning. A photo app that wants your keyboard is an example.

- An app allowed to see your files sees Documents, Downloads, Pictures and the rest. It never sees the hidden settings files that control how your computer starts things.
- An app allowed to play sound cannot listen to you unless you also allow the microphone.
- You can change any of this later under Permissions.

## Updates

Security updates install themselves every day, without asking, unless you switch off Check for updates in Settings, About this computer, which stops them too. Other updates wait for you on the Updates page. You can update one thing or press Install all.

Install all never puts in a plugin update that asks for something new. That one waits for you, with a label saying so.

![The Updates page of Apps with made-up updates. A card of apps lists Notes, Clock and a shared part, each with its size and round buttons to hold it back or install it. Cards for Trindle and Debian, firmware and plugins sit beside and below it.](img/apps-updates-found.png "What you will see: The Updates page.")

## Updates removed

After an update, Trindle takes away programs that nothing needs any more, and says which. It never takes away a screen reader, speech, braille or the libraries they use, even when nothing else needs them. If something you use was removed, install it again from Debian; the update will not remove it a second time.

## Plugins

Plugins are small extras that add to Trindle, such as Flow and Text. Each lives in a folder of its own and is allowed nothing until you say.

![The Plugins page of Apps. A card named Built in holds Pocketkey with a switch that is off, and a card below explains where plugins come from.](img/apps-plugins.png "What you will see: the Plugins page. Pocketkey is there, with its switch off.")

Plugins come only from Trindle's own list, which is signed, so nobody can slip one in. Each one is tested before it is added. Nothing is installed until you press Install.

If a plugin needs your say-so to work, its card says so at the top, with one button, Allow.

Pocketkey comes with Trindle and is off at first. When it is on, your phone answers password boxes with one tap. The screen locks when the phone goes out of range. Your password always still works.

## Without the internet

Apps opens without the internet. Installed, Updates, Plugins, Software sources, Move your apps and the settings for apps all work. Only Discover waits, because it is made of the online list. It tells you so.

![The Installed page of Apps on a computer with no connection. The list of what is installed still shows, with its tabs and search box, though Discover has nothing to show.](img/apps-offline-installed.png "What you will see: Installed with no connection.")

## When something goes wrong

When Apps cannot do what you asked, it says so in red words under the thing you were doing, and a small round **More** button sits beside them. Press it and Help opens at the page for that message. Each page below says what happened, why, and what to do.

![A drawn diagram of three steps: red words say what failed, you press More beside them, and Help opens at the section for those words.](img/errors-path.svg "What you will see: the way from red words to the fix. It is drawn to explain, not a screenshot.")

- [Apps could not reach the internet](apps.html#apps-could-not-reach-the-internet): Flathub, ratings, pictures, GitHub and the list of apps.
- [An app would not install or remove](apps.html#an-app-would-not-install-or-remove): Flatpak refused, an AppImage, a Flatpak reference, a dropped file.
- [An update did not finish](apps.html#an-update-did-not-finish): an app, Debian's and Trindle's updates, or a check that stopped.
- [Holding an update back did not work](apps.html#holding-an-update-back-did-not-work): the Hold back and Release buttons.
- [Firmware did not update](apps.html#firmware-did-not-update): a device's firmware, or the list of new firmware.
- [GitHub would not answer](apps.html#github-would-not-answer): the limit, a refusal, or a project with nothing to install.
- [The GitHub key was not kept](apps.html#the-github-key-was-not-kept): a key that GitHub refused, or the keyring could not hold.
- [A backup was not made](apps.html#a-backup-was-not-made): Move your apps, Back up.
- [A backup was not put back](apps.html#a-backup-was-not-put-back): Move your apps, Put a backup back.
- [A backup link is not encrypted](apps.html#a-backup-link-is-not-encrypted): a backup that fetches its apps over an address that is not secure.
- [A deb file would not install](apps.html#a-deb-file-would-not-install): the window for one package.
- [A plugin is not working](apps.html#a-plugin-is-not-working): a plugin, Pocketkey, or the plugin list.
- [A software source was refused](apps.html#a-software-source-was-refused): Software sources.

## Apps could not reach the internet
Questions: why does Apps say this computer is not connected to the internet; why is Discover empty; why does Apps say it could not reach Flathub; why are the pictures missing; why are there no ratings; what does the catalogue could not be read mean; how do I make Apps try again

Apps keeps a copy of Flathub's list of apps on this computer. If the copy is more than a day old, Apps fetches a fresh one when it opens. Discover is made from that list, so Discover waits for it. Pictures, ratings and what people say about an app come over the internet when you open the app's page.

![Discover in Apps with no connection. A line under the heading says this computer is not connected to the internet and that Apps tries again by itself. A yellow Try again button sits at the top right, and a card says which pages still work without the internet.](img/apps-discover-offline.png "What you will see: Discover on a computer with no connection.")

There are four things Apps can say.

- **"This computer is not connected to the internet."** This is in plain words, not red, because it is not a fault. Apps asks the computer whether it has a way out to the internet. When it has none, Apps says so. As soon as a connection comes back, Apps notices and asks again by itself, for the list and for updates.
- **"Apps could not reach Flathub."** The computer has a way out, but Flathub's list could not be fetched. The red words under it give the reason. Press **Try again**. If the connection is poor or the Flathub site is down, wait a little and press it again.
- **"The list of apps could not be fetched from Flathub."** There is no copy of the list on this computer and fetching one failed. Check the connection in Settings, Network, then press **Try again**.
- **"The catalogue could not be read."** A copy of the list is there, but it could not be read, and the reason follows. Press **Try again**. If it says the same again, the copy may be damaged.

Pages that do not need the list keep working: Installed, Updates, Plugins, Software sources, Move your apps and the settings for apps.

![The page of a made-up app called Music. Where its picture would be, a line says pictures cannot be fetched while this computer is not connected to the internet, with a round help button at the right.](img/apps-picture-offline.png "What you will see: An app page with no connection.")

On an app's page:

- **"This picture could not be fetched."** The picture's address did not answer. The other pictures may still come.
- **"Pictures cannot be fetched while this computer is not connected to the internet."** Open the page again once you are connected.
- **"No ratings to show." and "What people say cannot be fetched while this computer is not connected to the internet."**: ratings and what people write come from GNOME's open review service, over the internet. The app itself is not affected.

On From GitHub, **"GitHub was not asked: there is no internet to ask it over."** says the same: nothing was searched. Connect, then search again.

![The From GitHub page of Apps on a computer with no connection. A line under the heading says GitHub cannot be searched while this computer is not connected to the internet.](img/apps-offline-github.png "What you will see: From GitHub with no connection.")

:::expert
The list is Flathub's appstream data, kept by Flatpak for your user. It is refreshed when it is more than 24 hours old, with the same refresh that Flatpak does for its own `flathub` remote. If the remote is missing, for instance because the computer was first set up offline, Apps adds it again. It uses the copy of `flathub.flatpakrepo` that Trindle carries. Only when that copy is not there does it fall back to `https://dl.flathub.org/repo/flathub.flatpakrepo`. "Connected" means the kernel has a route to everywhere, either the usual kind or the newer IPv6 kind; it does not prove that the internet answers, which is why a connected computer can still say Flathub could not be reached. Ratings are fetched from `https://odrs.gnome.org/1.0/reviews/api`.
:::

## An app would not install or remove
Questions: why does it say Flatpak refused; why did my app not install; what does could not be installed mean; why will an AppImage not add; why does Apps not take the file I dropped; why can I not uninstall this; what does it did not finish and gave no reason mean

When you press Install or Remove, Flatpak does the work. The ring round the button runs until it is done. If Flatpak stops, the ring ends with "Not done" and Apps says why in red under the app, in the last message Flatpak gave. If Flatpak gave none, you see "Flatpak refused."

![The page of a made-up app called Music that was not installed. Under its name and Install button, red words say Flatpak refused, with a round help button beside them.](img/apps-install-refused.png "What you will see: An install that Flatpak refused.")

The reason is Flatpak's own, so it changes from case to case. A full disk, or a download that stopped, are the kinds of thing it says. If it is the download, check the connection first: see [Apps could not reach the internet](apps.html#apps-could-not-reach-the-internet). Then press Install again.

Other words you may see:

- **"It did not finish, and gave no reason."** The job ended without success and without saying why. Press the button again.
- **"{ name } is not in the Flathub catalogue."** Apps was asked to open an app by its identifier and Flathub's list does not have it. Search for it by name in Discover.

![Search results in Apps. Red words above the tiles say a made-up app is not in the Flathub catalogue, with a round help button.](img/apps-results-problem.png "What you will see: Words about an app Flathub does not have.")

- **"There is no web browser to open the address. Get one in Apps first."** You pressed a link in Apps, such as an app's website, and the computer found nothing to open it with.

An AppImage is added from Installed, with **Add an AppImage**.

- **"There is no session bus to ask which file."** and **"The file chooser could not be asked."** The window that lets you pick a file could not be started. Try once more, or drop the file on the Apps window instead.
- **"The AppImage could not be added."** The file could not be copied into the AppImages folder or given a launcher. The red words under the Installed list give the reason, such as no room left.

A Flatpak reference is a short `.flatpakref` file that names an app and where it comes from.

- **"That is not a Flatpak reference this computer can use."** Apps takes only a reference that names an app (not a runtime) and an `https://` address. Anything else is turned away.
- **"Flatpak would not install it."** or **"{ name } could not be installed."** with a reason: Flatpak tried the address in the file and stopped. The reason is Flatpak's own last message.

![The Installed page of Apps. Under the row of a made-up program, red words say it could not be removed because a folder is in use, with a round help button.](img/apps-remove-failed.png "What you will see: A program that could not be removed.")

Dropping files on the Apps window: Apps takes `.deb` files, `.AppImage` files and `.flatpakref` files, and looks inside each before it believes the name. A file that ends in `.AppImage` but is not a program is turned away. So is a `.flatpakref` larger than 64 KB. Anything else gets "Only .deb, .AppImage and .flatpakref files can be dropped here."

In the ring, Uninstall can be greyed. Its tip says why: "Can't uninstall: part of Trindle", "the computer needs it" or "part of Trindle needs it". Those programs are what Trindle is made of, so taking them away would break the computer.

:::expert
Installing runs `flatpak install --user -y flathub <id>` under a gentle scheduling class so it does not slow your work. Removing runs `flatpak uninstall` in the app's own place, with `--delete-data` only when you chose Delete its data too. The red words are the last line of Flatpak's output that looks like a message and not progress. A `.flatpakref` is installed with `flatpak install --user --noninteractive -y --from=<file>`.
:::

## An update did not finish
Questions: why does it say not updated; what does Flatpak said mean; why were Debian's and Trindle's updates not installed; why does Apps ask for a password to update; why does it say updates could not be checked; what does it took too long to answer mean; why does an update say it did not finish

Updates for the apps you installed yourself need no password. Debian's and Trindle's updates change the whole computer for everyone who uses it, so they need an administrator's password.

![The Updates page of Apps. In the card of apps, the row for a made-up app called Clock has red words under it: Not updated, and Flatpak said there was not enough disk space. A round help button sits beside the words.](img/apps-update-not-updated.png "What you will see: An app that did not update.")

What the red words can say:

- **"Not updated. Flatpak said: ..."** Flatpak gave a reason and Apps repeats it. It is the last message that was not progress. It might be a full disk or a network that dropped, for example.
- **"Not updated, and Flatpak gave no reason."** The update ended without a message. Press Update again.
- **"Not installed, because no administrator's password was given."** You closed the password box. Nothing was installed. Press Install all and give the password. The same row can say "because the administrator's password was not accepted". It can also say "because this login is not allowed to do that, whatever password is given" or "because nothing was there to ask for the administrator's password". Or it can say "because the administrator's password was not accepted, or this login is not allowed to do that".
- **"Not installed, because the lists of what is new could not be fetched."** Before it installs, Apps fetches Debian's lists of what is new. When that fails, nothing is installed from the old lists, and the reason follows. Check the connection: see [Apps could not reach the internet](apps.html#apps-could-not-reach-the-internet).
- **"It did not finish. ..."** or **"It did not finish, and gave no reason."** The step ran and stopped. The reason is what the system gave.

![The Updates page of Apps. In the Trindle and Debian card, red words say a made-up package could not be set up, with a round help button.](img/apps-updates-failure.png "What you will see: A Debian update that stopped on one package.")

When Apps looks for updates, each card on the Updates page has its own result.

- **"Could not check. Flathub could not be reached."** The apps card could not fetch Flathub's list.
- **"Could not check. Debian's lists could not be fetched."** The Trindle and Debian card could not fetch Debian's lists. If a reason follows, it is what the system said.
- **"Could not check. It took too long to answer."** A check has three minutes. After that Apps stops waiting so no spinner turns for ever. If the answer comes late, the card puts itself right.

![The Updates page of Apps. In the apps card, red words say Flathub could not be reached, so Apps could not check, with a round help button.](img/apps-updates-could-not-check.png "What you will see: A check that could not be made.")

With no internet, none of these is red. The page says plainly that it cannot look.

If you hold an update back, it stays out of Install all. See [Holding an update back did not work](apps.html#holding-an-update-back-did-not-work).

:::expert
An app update runs `flatpak update --user -y` for that app or shared part. A check runs `flatpak update --user --appstream flathub` and then reads what is waiting from Flatpak's own record. The system's part runs in the order: fetch Debian's lists, count what waits, then install. A list that cannot be fetched, or a password box that is closed, ends the part there. You are never asked for a second password after the first was refused. When nothing waits after the lists are fetched, no password is asked for.
:::

## Holding an update back did not work
Questions: why does it say not held back; why was the held back list not kept; why could Flatpak not be told; why does it say not released

Hold back keeps one app, plugin or device's update out of Install all, on this page and on the ring, until you release it. Trindle keeps its own list of what you hold. For a Flatpak app it also tells Flatpak to leave that app out ("mask" it), so nothing else that updates apps can put it in behind your back.

![The Updates page of Apps. Under the row of a made-up app called Notes, red words say it is held back in Apps but Flatpak itself could not be told, with the reason Flatpak gave and a round help button.](img/apps-held-not-told.png "What you will see: A hold that Flatpak was not told about.")

- **"Not held back. The list of what is held back could not be kept: ..."** The list could not be written to your folder, for instance because the disk is full. Nothing changed. The app is still updated by Install all.
- **"Not released. The list of what is held back could not be kept: ..."** The same the other way: it is still held back.
- **"Held back in Apps, but Flatpak itself could not be told."** Trindle's list is changed and Apps will leave the app alone. Flatpak did not take the mask, so another program that updates apps could still update it. Flatpak's own message follows when it gave one. Apps tells Flatpak again about everything held each time it checks for updates, so a mask that was lost is put back then.

![The Updates page of Apps with updates held back. Held rows are greyed with their pin button lit, and a Held back card below lists four things that are held, each with a button to release it.](img/apps-updates-held-back.png "What you will see: The Updates page with updates held back.")

- **"Released in Apps, but Flatpak itself could not be told and may still leave it out."** Hold it back and release it again.

Trindle's and Debian's updates cannot be held back, because they carry security fixes for the whole computer.

:::expert
Trindle's list is the truth and is written first. A Flatpak is then masked with `flatpak mask --user <pattern>` and unmasked with `--remove`. Releasing something Flatpak never masked ("No current mask") is not an error. Each time Apps checks for updates it tells Flatpak again about every Flatpak held, so a mask somebody took off by hand is put back.
:::

## Firmware did not update
Questions: why does firmware not update; why does it say fwupd could not fetch the newest list; why does the firmware row say not installed; do I need to restart after a firmware update

Firmware is the small program inside a part of your computer, such as the main board. New firmware comes from fwupd and the Linux Vendor Firmware Service. It changes the computer itself, so it needs an administrator's password.

![The Updates page of Apps. In the Firmware card, the row of a made-up keyboard has red words under it saying it was not installed because no administrator password was given, with a round help button.](img/apps-firmware-failed.png "What you will see: A firmware update that did not go in.")

- **"Could not check. fwupd could not fetch the newest list."** fwupd could not download the list of what is new. Check the connection and press Check for updates. fwupd also fetches the list by itself twice a day.
- **A row in red under a device, such as "Not installed, because no administrator's password was given."** The firmware for that device did not go in. Why is in the words, as for any other update. See [An update did not finish](apps.html#an-update-did-not-finish) for the other reasons.
- **"It did not finish. ..."** The device or fwupd stopped the update. The reason is what they said.

After firmware goes in, the card says "Restart to finish". New firmware is put into use when the computer starts again.

:::expert
Devices are listed with `fwupdmgr get-updates --json`. The list is refreshed with `fwupdmgr refresh --force --no-authenticate`. Each device is updated in turn through `trindle-admin firmware-update <device>`, which asks for the administrator's password. A failure for one device does not stop the next.
:::

## GitHub would not answer
Questions: why does GitHub say it has stopped answering; what is the GitHub limit; why does it say no AppImage or Flatpak file for this computer; why was a project not looked at; why did the download not finish; what does GitHub refused this mean

From GitHub finds apps that people publish as AppImage or Flatpak files in their GitHub releases. Nobody checks these files, so install only from people you trust. Apps asks GitHub about each project it finds.

![The From GitHub page of Apps. Under the search box, red words say GitHub has stopped answering for now, that it answers 60 questions an hour without a key, and when it will answer again. A round help button sits beside them.](img/apps-github-limit.png "What you will see: The limit GitHub sets used up.")

- **"GitHub has stopped answering for now."** GitHub limits how many questions one computer may ask. The words say which limit was used up and when it answers again. It is one of these: 60 questions an hour from one address without a key, 10 searches a minute, 30 searches a minute with a key, or 5,000 questions an hour with a key. Wait for the time it names, or "Try again later". A [key](apps.html#the-github-key-was-not-kept) raises the limit from 60 to 5,000 an hour.
- **"GitHub refused this: ..."** or **"GitHub refused this, though it has not run out of answers."** GitHub said no for a reason other than its limit, such as a project it has shut. Its own words follow.
- **"GitHub could not be reached."** Nothing came back at all. Check the connection: see [Apps could not reach the internet](apps.html#apps-could-not-reach-the-internet).



- **"GitHub answered 500."** and the like: GitHub sent a code and no reason.
- **"This project's releases have no AppImage or Flatpak file for this computer."** and **"GitHub has no releases for this project."** and **"Nothing in its newest release installs here."**: this is an answer, not a fault. The project has nothing Trindle can install on this kind of computer.
- **"Only projects with an AppImage were searched for: GitHub did not answer the search for Flatpaks."** Half of the search worked.
- **"3 projects found could not be looked at, so they are not listed: ..."** The count of projects Apps could not read, with the reason for the last one.

![The From GitHub page of Apps with made-up projects found. A card lists projects from AppImageHub and another lists projects found on GitHub, each with its name, its stars and round buttons.](img/apps-github-results.png "What you will see: From GitHub with projects found.")

- **"The download did not finish."** The file stopped part way. Try again.

The files are downloaded over a secure link. If the maker publishes no signature, the page says "Downloaded securely, but nobody has checked this file: its maker publishes no signature."

:::expert
Questions go to the GitHub API with curl over https only, with a 20-second limit and a cap on the size of the answer. A reply of 403 or 429 is a limit when the count of questions left is nought, when GitHub says how long to wait, or when its words say so. A 403 with questions left and no word of a limit is shown as a refusal with GitHub's own words, because advice about a key would be wrong then. The wait is the time to the reset that GitHub reports, rounded up to whole minutes.
:::

## The GitHub key was not kept
Questions: do I need a GitHub key; why did GitHub not accept the key; why does it say the key has a space in it; why is my key still in a plain file; why can the keyring not be reached

You almost certainly do not need a key. Without one, GitHub answers 60 questions an hour from this computer; with one it answers 5,000. The key is used to look at public files and nothing else.

To make one, press the key at the top right of the Search GitHub card. Press **Make a key on GitHub**, give it any name and leave every permission alone. Press Generate token, then paste what it shows into the box and press **Keep the key**. Trindle keeps it in the computer's keyring, not in a file.

![The GitHub key box open over the From GitHub page. It says no key is stored and has a box to paste a key into. Red words at the foot say GitHub did not accept the key, with a round help button.](img/apps-github-key-refused.png "What you will see: A GitHub key that was not accepted.")

- **"GitHub did not accept the key. Make a new one, or remove it: press the key at the top right of Search GitHub."** The key is wrong, has run out or was cancelled on GitHub. Make a new one, or press **Remove the key**. Searching works without one.
- **"A key is one line with no spaces in it, and this has a line break or a space inside it. Paste the key alone."** The paste broke across lines. Paste it again, on its own.

![The GitHub key box open over the From GitHub page. It says a key is stored, and has buttons to make a key on GitHub and to remove the stored key.](img/apps-github-key-stored.png "What you will see: The GitHub key box with a key stored.")

- **"A GitHub key is stored, but it cannot be sent: ..."** An older Trindle kept whatever was pasted, a space included. GitHub is asked without a key until you remove it and paste it again.
- **"Your GitHub key is still in a plain file on this computer, and is used from there: it could not be moved into the keyring."** The keyring would not take it, and the reason follows. The key still works. Remove it and paste it again when the keyring is available.
- **"The keyring could not be reached." and similar.** The keyring is locked, missing or slow. GitHub is asked without a key meanwhile, and the page says so. Unlock the keyring, then try again.
- **"Could not keep it" and "Could not remove it"** with a reason: the keyring or the file refused. Nothing changed.

![The GitHub key box open over the From GitHub page with no key stored. It says a key is almost never needed and has a box to paste one into, with buttons to make a key on GitHub and to keep it.](img/apps-github-key-open.png "What you will see: The GitHub key box before a key is kept.")

:::expert
The key is read from the keyring on a thread of its own, so a keyring that hangs does not hold the window. It is sent to curl through its standard input as a header, never on the command line, and is left out of backups. A key with a space or line break in it is not sent at all.
:::

## A backup was not made
Questions: why was the backup not made; what is in a backup; where does the backup go; how big is a backup

Move your apps makes one file with your apps and settings, to take to another computer. It holds the list of your Flatpak apps (the other computer installs them fresh from Flathub, so the file stays small). It also holds each app's own data without its caches, your AppImages whole, and Trindle's settings: the ring, wallpaper choice, colours, keyboard, power, default apps and your picture. Your documents and pictures are not in it: copy those over yourself. You can leave any part out.

The file goes in your home folder and is named `trindle-backup-` and the date, ending `.fmpack`.

![The Move your apps page. Under the Back up card, red words say the backup was not made because there is no room left on this computer, with a round help button.](img/apps-backup-not-made.png "What you will see: A backup that was not made.")

**"The backup was not made, because ..."** The reason follows, in plain words. The ones Apps gives are:

- **this computer would not let Trindle write there.**
- **there is no room left on this computer**, or **the space allowed for this account is used up.**
- **that part of this computer is read-only.**
- **a file or program it needs is missing.**
- **it gave no reason.**

Free some space, or leave out the part that is large (Apps' own data is usually the largest, and Apps shows its size), then press Make the backup again.

## A backup was not put back
Questions: why was nothing put back; why does it say part of the backup was not put back; what does something is already where part of the backup needs to go mean; why was my account picture not put back; why is that file not a Trindle backup; why did these apps not install

Put a backup back installs the apps in a backup, puts back their data and settings, and puts back Trindle's settings. It reads a backup made by Trindle, or one made by Trindle Flat. A backup is only a file, and anyone can write one, so Apps checks all of it before it moves anything. If it finds one problem, it refuses the whole backup and puts nothing back, so a restore never stops half done for something that could be seen beforehand.

![The Move your apps page. In the Put a backup back card, red words say nothing was put back because that file is not a Trindle backup. A round help button and a round i button sit beside the words.](img/apps-backup-nothing-put-back.png "What you will see: A backup that was refused.")

**"Nothing was put back, because ..."** Reasons Apps gives:

- **the backup file could not be read.** The file is missing, damaged or not yours to read.
- **that file is not a Trindle backup**, **that file does not look like a Flat backup**, **that file does not hold a Flat app list**; **there are no apps in that file**, or **the backup is damaged and its list could not be read.** It is not the kind of file the page expects, or it is cut short. Make the backup again.
- **this was made by a newer Flat than Trindle can read.** Update Trindle, then try again.
- **it is too big to be read whole.**

![The Move your apps page. A Back up card has four switches, for apps, their data, AppImages and Trindle settings. A Put a backup back card sits below it.](img/apps-move-your-apps.png "What you will see: The Move your apps page.")

- **the backup holds a link or something else that would reach outside the folder it goes into**, or **its settings in the backup reach outside its own folder.** The file is built in a way a backup made by Trindle is not, so it is refused whole.
- **something on this computer is already where part of the backup needs to go.** The round **i** beside the words lists the paths in the way. Move or rename them, then try again.

![The Move your apps page after a backup was refused because something is already in the way. Red words say nothing was put back, and the round i button has opened the list of the two made-up files in the way.](img/apps-backup-behind-open.png "What you will see: What the round i button shows.")

- **there is nowhere to unpack the backup**, **there is no room left on this computer**; **this computer would not let Trindle write there**, or **that part of this computer is read-only.**
- **its settings in the backup do not match their checksum**, or **it has no name.** Something changed in the file after it was made.

**"Part of the backup was not put back, because ..."** Most of it went in. The words say which part and why.

- **"Put back, except your account picture."** The picture could not be written. Everything else went in.
- **"Settings and data were put back, but these apps were not installed, because Flatpak refused them."** The apps are listed with Flatpak's reason for each. Install them from Discover.
- **"Not put back: ..."** A list, each with its reason: a source that is not encrypted and so was not used, a source the backup does not say where to find, or a name that is not valid.

![The Move your apps page after a restore. In the Put a backup back card, red words say settings and data were put back, but two made-up apps were not installed because Flatpak refused them. A round help button sits beside the words.](img/apps-backup-not-installed.png "What you will see: Apps that were not installed after a restore.")

An app's data goes back as it was. Trindle's settings and AppImages go back as plain files and folders only.

:::expert
Nothing is unpacked where it is to go. It is unpacked first into a folder made new for it, inside the folder it is to go into. Each thing is then moved into place by its name with `renameat`, so no link is followed at any step. Everything is looked at twice before anything is moved: the first pass touches nothing. What is read into memory is read through a limit. A backup made by Trindle Flat is read by the same rules. The account picture is set through the administrator's helper, as in Settings, so it asks for the administrator's password. When that cannot be started, or the password box is closed, the picture is the part that is not put back, and the reason is in the words.
:::

## A backup link is not encrypted
Questions: why does the backup warn me that a link is not encrypted; what does not encrypted mean; why must I tick I understand; is it safe to put back this backup

A backup names the places its apps come from. Most use `https://`, which is encrypted. If one uses `http://`, what is sent can be read by anyone on the way, and could be swapped for a different app.

![The Move your apps page with a card called Not encrypted. It warns that the backup address is not encrypted, has a switch labelled I understand and accept this risk, and two buttons, Restore and Cancel. A round help button sits beside the warning.](img/apps-backup-http-card.png "What you will see: The card for a backup link that is not encrypted.")

- **On your own network, you get one line and the restore goes ahead:** "The link to the apps is not encrypted." Your own network means this computer, a name ending `.local`, and the addresses a home router hands out.
- **Over the internet, the restore waits.** A card called **Not encrypted** says "This address is not encrypted. Someone on the way could see what is sent, and could swap the backup for a different one." Tick **I understand and accept this risk**, then press **Restore**. Or press **Cancel**. Nothing is refused outright.

If you do not know why a backup uses an `http://` address, press Cancel.

:::expert
Home addresses are: this computer's own name, `localhost`, 127.x, `::1`, 10.x, 172.16 to 172.31, 192.168, 169.254, `fc00::/7`, `fe80::/10`, and names ending `.local`. A name that only looks like an address in another notation counts as the internet. When a backup lists several addresses, the worst one decides.
:::

## A deb file would not install
Questions: why is this not a Debian package that can be installed; why does it say this is for another kind of computer; why can a driver not be installed; why does it say this is Trindle itself; why does it say these parts could not be found

A `.deb` file is a program packed the Debian way. Apps does not spread its files across the computer. It opens the file in a window of its own, then puts the program in its own folder, with the libraries it needs, restricted until you allow more. Install only packages from makers you trust.

![The window for a made-up package called example, before it is installed. It shows where the file came from, a note that a package from outside Debian goes in its own folder, the package facts, and a button to install it.](img/apps-package-new.png "What you will see: a package window before the install.")

- **"This file is not a Debian package that can be installed. It may not have finished downloading."** The file could not be read as a package, or its name is not a valid package name. Download it again.

![The window of a file that is not a package. Red words say the file is not a Debian package that can be installed and may not have finished downloading.](img/apps-package-not-a-package.png "What you will see: A file that is not a package.")

- **"This package is for arm64 computers, so it cannot go on this one."** The package is made for another kind of processor. Get the one for this computer.
- **"This is Trindle itself. It updates from Apps' Updates page, not from a file."** Packages named `trindle` or starting `trindle-` are Trindle's own.

![The window of a package called example that is Trindle itself, said in words, with the facts about it below.](img/apps-package-trindle.png "What you will see: A package that is Trindle itself.")

- **"{ name } is a driver or a background service, which cannot work from its own folder, so it cannot be installed here."** Apps looks inside the package. Some packages put files in places such as kernel modules, firmware, systemd services, udev rules, printer or scanner drivers, or the system's message bus. A package like that cannot work from a folder of its own. Apps keeps drivers and background services out of its own folders, as [Apps that leave nothing behind](apps.html#apps-that-leave-nothing-behind) says.

![The window for a made-up package called example-driver. Under its name, red words say it is a driver or a background service, which cannot work from its own folder, so it cannot be installed here. A round help button sits beside them.](img/apps-deb-refused.png "What you will see: A package that cannot be installed here.")

- **"Installed in its own folder, but these parts could not be found, so it may not start: ..."** The package needs parts that Apps could not find. The list names them.
- **"... could not be unpacked."** or **"... is not a Debian package that can be installed."** The file is damaged. Download it again.
- **"That program is not kept in a box."** You tried to change what a program may do. It is not one of the programs Apps keeps in its own folder.

Removing a `.deb` program takes everything it saved with it.

:::expert
The package is read with `dpkg-deb -f` and listed with `dpkg-deb -c` without being unpacked, and its name is checked against Debian's rule for package names before anything is run. A package fits if its architecture is this computer's, `all`, or one of the foreign architectures dpkg has set up. A package is turned away as a service if it has files in kernel module, firmware, systemd, udev, modprobe, D-Bus system service, CUPS or SANE places, or a DKMS configuration.
:::

## A plugin is not working
Questions: why does it say its background part is not running; why was the plugin not removed; why is Pocketkey not answering; why was the plugin catalogue not used; why does it say the file is not the one the catalogue lists; why are plugin updates paused

Plugins are small extras that add to Trindle. They come only from Trindle's own list, which is signed so nobody can slip one in, and each is tested before it is added.

![The Plugins page of Apps. A card named Built in holds Pocketkey with a switch that is off, and a card below explains where plugins come from.](img/apps-plugins.png "What you will see: the Plugins page of Apps.")

A plugin that was refused by the check, would not download or go in, or whose background part is not running has a page of its own on [Plugins](plugins.html#when-something-goes-wrong). So does a list that could not be used.

![The Plugins page of Apps. The card of a made-up plugin called Weather has red words at its foot saying it is installed but its background part is not running, because no desktop session is running, with a round help button.](img/apps-plugin-not-running.png "What you will see: A plugin whose background part is not running.")

- **"It is installed, but its background part is not running: ..."** The plugin is in, but the part that runs all the time did not start. The reason follows. It can be one of these: no desktop session is running (log in first); it did not start within the seconds it was given; the program that starts plugins could not be found; or there are several desktop sessions and Trindle cannot tell which to join.
- **"Not done: ..."** Under Where plugins come from, after you pressed **Start again**: the record could not be taken away, and the reason follows.
- **"It could not be removed: ..."** The plugin's folder could not be taken away. Nothing else changed.
- **"It could not be downloaded: ..."**, **"The archive could not be unpacked: ..."**, **"It does not pass Trindle's plugin check, so nothing was installed: ..."**, **"The plugin's files could not be copied."** and **"It could not be saved: ..."** The step it names failed, and nothing was installed.
- **"Its file is not the one the catalogue lists (the SHA-256 differs), so nothing was installed."** The downloaded file does not match the list. It was not installed. This protects you from a changed file.

![The Plugins page of Apps. The card of a made-up plugin called Tides has red words saying its file is not the one the catalogue lists, so nothing was installed, with a round help button.](img/plugins-hash.png "What you will see: A plugin file that is not the one listed.")

- **"Pocketkey is not answering, so the switch has not changed."** Pocketkey is built in. Its background part did not answer when you pressed the switch. Try again in a moment.
- **Under Where plugins come from, "Trindle's plugin catalogue ... was not used."** Apps checks the list of plugins before it trusts it. It was not used for one of these reasons: it has no signature; the signature is the wrong size; it is not signed with Trindle's key; it does not say which number it is; it is older than the last good one; it is signed but cannot be read; or this Trindle has no key to check it with. The last good list is shown when there is one.
- **"Plugin updates are paused: the plugin list is damaged."** A **Start again** button asks for the administrator's password and begins the list afresh.

![The Plugins page of Apps. The Where plugins come from card has red words saying plugin updates are paused because the plugin list is damaged, with a round help button.](img/plugins-list-paused.png "What you will see: Plugin updates paused.")

- **"Not held back" or "Not released"** on a plugin: the list of what is held back could not be kept. See [Holding an update back did not work](apps.html#holding-an-update-back-did-not-work).

:::expert
The catalogue is read from trindle.org each time the Plugins page opens. It is accepted only if its signature checks with the key that Trindle carries and its number is not older than the last one accepted. A plugin's archive is accepted only if its SHA-256 matches the one in the catalogue.
:::

## A software source was refused
Questions: why does it say both addresses must start with https; why is that not a package name; why can I not add a source; what is a suite and a component

A software source lets a maker's own repository install and update a program for the whole computer. Apps knows some makers. You can add another under Software sources, with **Add this source**.

![The Software sources page of Apps. In the Add another source card, the address box holds an address that starts with http, and red words under the form say both addresses must start with https, with a round help button.](img/apps-source-refused.png "What you will see: A software source that was refused.")

- **"Both addresses must start with https://"** The repository's address and the signing key's address must both be secure links. They also may not hold a quote mark, a semicolon or a dollar sign.
- **"The suite and components must be plain words."** Suite is usually `trixie` or `stable`. Components are usually `main`. They may hold only letters, digits, dots, slashes and dashes, and none may start with a dash.
- **"That is not a package name."** The optional program to install must be a Debian package name: lower-case letters, digits, plus, dot and dash, starting with a letter or digit.
- A source whose job failed shows the reason under its row. Adding or removing a source asks for an administrator's password, so a closed password box ends the job with no change.

A source can install updates for the whole computer, so add only makers you trust.

:::expert
Sources you add from the list of known makers are kept as `trindle-<name>.sources` files for apt, and Already on this computer lists the ones found. The checks above run before anything is asked of the administrator.
:::

## Related

- [Flow](flow.html): a plugin that shares one mouse and keyboard.
- [Glossary](glossary.html): Flatpak, AppImage and `.deb`.
- [Troubleshooting](troubleshooting.html): an app that will not install.
