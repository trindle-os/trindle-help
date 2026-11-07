---
title: Plugins
summary: What the plugin messages in Apps mean, and what to do when a plugin will not go in or will not run.
group: Using Trindle
order: 75
---

A plugin is a small extra that adds to Trindle, such as an icon in the ring, a page in Settings, notifications, or something that runs from login. Plugins come only from Trindle's own list, which is signed so nobody can slip one in. Each plugin is tested before it is added, and each runs walled off, allowed nothing until you allow it in Permissions.

You find plugins in Apps, on the Plugins page. [Apps](apps.html#plugins) says how to add and remove them. This page is about the red words you may meet there.

![The Plugins page of Apps. A card named Built in holds Pocketkey with a switch that is off, and a card below explains where plugins come from.](img/apps-plugins.png "What you will see: the Plugins page of Apps.")

## When something goes wrong

When a plugin cannot be fetched, put in or started, Apps says so in red words under the plugin's card or under Where plugins come from. A small round **More** button sits beside them. Press it and Help opens at the page for that message.

- [A plugin was refused by the check](plugins.html#a-plugin-was-refused-by-the-check): the plugin's own description did not pass.
- [A plugin's background part is not running](plugins.html#a-plugin-s-background-part-is-not-running): it is in, but the part that runs all the time did not start.
- [A plugin would not download or go in](plugins.html#a-plugin-would-not-download-or-go-in): fetching, unpacking or putting it in failed.
- [The plugin list could not be used](plugins.html#the-plugin-list-could-not-be-used): the signed list was refused, or its record is damaged.

The Apps page [A plugin is not working](apps.html#a-plugin-is-not-working) covers the rest: a plugin that could not be removed, a permission that could not be kept, and Pocketkey not answering.

## A plugin was refused by the check
Questions: why does it say the plugin does not pass Trindle's plugin check; why was nothing installed; what does Interface mean in a plugin; why does it say the plugin needs a Name; why does it say a program is not one the plugin carries; why does it say a link leads outside the plugin's folder

Every plugin carries a small file called `plugin.ini` that describes it: its name, what it adds, what it asks to be allowed. Trindle checks that file twice. Its own people check it when they build the list, and your computer checks it again as the plugin goes in. If anything does not fit, the plugin is not installed. Nothing is put in half way.

![The Plugins page of Apps. The card of a made-up plugin called Tides has red words saying it does not pass Trindle's plugin check, so nothing was installed, and that it needs a Name. A round help button sits beside them.](img/plugins-check-refused.png "What you will see: A plugin refused by the check.")

The red words show the first thing that is wrong. There may be more behind it. You cannot change a plugin yourself: the fix is the plugin's maker's. What you can do:

1. Press the plugin's button again later. The list is read each time the Plugins page opens, and a corrected plugin comes with the next list.
2. Look at Apps' Updates page. A newer Trindle may know an interface that this one does not.
3. Tell the plugin's maker, or use Report a problem, and quote the red words.

What the check looks at, and the words it uses:

- **The file.** "There is no plugin.ini that can be read at the top of the folder" and "plugin.ini has no [Plugin] part". The file is missing, unreadable, or its first part is not there.
- **The interface.** "[Plugin] needs Interface=1." or "Interface=2 is not one this Trindle knows: it knows Interface=1." The plugin was written for another version of the way plugins work.

![The Plugins page of Apps. The card of a made-up plugin called Tides has red words saying it does not pass the plugin check and that it needs an Interface number, with a round help button.](img/plugins-check-interface.png "What you will see: A plugin with no Interface line.")

- **Its name, version and summary.** "[Plugin] needs an Id of letters, digits, dots and dashes, such as org.example.Weather", "needs a Name", "needs a Version", "Summary is one line". Name, Version and Summary are one plain line each, up to 48, 32 and 200 characters.
- **Permissions.** "Permission ... is not one there is: they are ..." followed by the list. A plugin may ask only for what Trindle has. It may not ask for the network and the home network both.

![The Plugins page of Apps. The card of a made-up plugin called Tides has red words saying a permission it asks for is not one there is, and listing the ones there are. A round help button sits beside them.](img/plugins-check-permission.png "What you will see: A plugin that asks for a permission that does not exist.")

- **The ring icon.** "[Ring] needs an Icon: an SVG or PNG file in the plugin's folder", "Icon names ..., which is not in the plugin's folder", "needs a Label" and "needs Run, what a click starts".
- **A menu on the ring icon.** At most eight items. Each has a plain label up to 32 characters, an id of letters, digits, dashes and underscores, named once, and its own part and its own Run.
- **Its settings page.** "[Settings] needs Fields"; a field name with marks that are not allowed; a field with no part of its own; a missing Label; a kind that is not switch, text, choice or note; a switch whose Default is not true or false; a choice with fewer than two Choices (at most 32, each up to 48 characters) or whose Default is not one of them; and a note that has no TextFrom or has a Default. A file name for ChoicesFrom or TextFrom is letters, digits, dots, dashes and underscores, not starting with a dot, with no folders.
- **ShowWhen.** A field can be shown only when another earlier field holds a value. The words say which rule broke: it must be written `field=value`, name another field, name one that comes earlier, name one that has no ShowWhen of its own; for a switch the value is true or false; for a choice one of its Choices; and a note holds no setting to depend on.
- **Hooks.** "[Background] needs the background permission" and "The plugin hooks into nothing" when it has no ring icon, settings, background part, shortcut or notifications permission.

![The Plugins page of Apps. The card of a made-up plugin called Tides has red words saying the plugin hooks into nothing, and what it needs to hook into. A round help button sits beside them.](img/plugins-check-hooks.png "What you will see: A plugin that hooks into nothing.")

- **Programs.** "Run must name a program inside the plugin's folder"; "Run names ..., which is not a program the plugin carries (missing, or not executable)"; and "Run cannot be read as a command".
- **Links.** "... links outside the plugin's folder". A plugin may carry no link that leads out.

![The Plugins page of Apps. The card of a made-up plugin called Tides has red words saying a file in it links outside the plugin folder, with a round help button.](img/plugins-check-links.png "What you will see: A plugin with a link that leaves its folder.")

- **Shortcuts.** At most four, each with an id, its own part, Keys that are not already a shortcut of Trindle's or of the plugin's other shortcuts, and a Run.
- **Ports to listen on.** "[Network] Listen" needs the network and listen permissions, and listen needs ports to name. At most four, each as `tcp:` or `udp:` and a port from 1024 to 65535, none twice, and none that Trindle or a common service uses.

![The Plugins page of Apps. The card of a made-up plugin called Tides has red words saying it does not pass the plugin check because it listens on the network without the network permission, with a round help button.](img/plugins-check-listen.png "What you will see: A plugin that listens without the network permission.")

- **Unlocking by PIN.** "is only for Flow from Trindle's catalogue" and "needs Run, the command that is given the PIN".

:::expert
The check is `plugins::check`, and it collects every problem it finds. The catalogue install shows only the first, in `plugins-install-check`. It runs on the unpacked folder before anything is put in place, after the archive's SHA-256 has matched the one in the signed list. A plugin with an id other than the one the list named is refused before this ("The archive holds ..., not ..."). [A plugin would not download or go in](plugins.html#a-plugin-would-not-download-or-go-in) describes it.
:::

## A plugin's background part is not running
Questions: why does it say its background part is not running; what is the background part of a plugin; why does it say no desktop session is running; why does it say it did not start within 15 seconds; why does it say trindle-plugin could not be found; why does it say it cannot tell which desktop session to join

Some plugins have a part that runs all the time, out of sight, such as one that waits for a phone to be near. It is called the background part. A plugin has one only if it asks for the **background** permission and you have allowed it. It starts when you log in, and again when the plugin is put in or updated.

![The Plugins page of Apps. The card of a made-up plugin called Weather has red words at its foot saying it is installed but its background part is not running, because no desktop session is running, with a round help button.](img/apps-plugin-not-running.png "What you will see: A plugin whose background part is not running.")

The red words say the plugin is in, then why its background part is not running.

- **"no desktop session is running for it to join, so it was not started. Log in first."** The part joins your desktop. With no desktop session there is nothing to join. Log in, then it starts.
- **"it did not start within 15 seconds."** It was given 15 seconds and did not answer. Log out and in: the part is started again at each login.

![The Plugins page of Apps. The card of a made-up plugin called Weather has red words at its foot saying its background part is not running because it did not start within 15 seconds, with a round help button.](img/plugins-not-running-slow.png "What you will see: A background part that was too slow to start.")

- **"trindle-plugin could not be found to start it."** The program that starts plugins is not where Trindle looks for it, so Trindle may be incomplete on this computer. Look in Apps, Updates, and use Report a problem if it stays.
- **"it cannot tell which desktop session to join (...), so it was not started."** You have more than one desktop session open. The part cannot tell which is yours. Log out of the extra sessions.
- **"... Its background part is not running: ..."** at the end of an update's words. The new version did not go in, and the one that was there is not running either. The reason for each follows. Try the update again after you have dealt with the reason.

![The Plugins page of Apps. The card of a made-up plugin called Tides has red words saying its file is not the one the catalogue lists, and that its background part is not running because no desktop session is running. A round help button sits beside them.](img/plugins-swap-not-running.png "What you will see: An update that was not put in, with the background part not running.")

If you have not allowed **background** in Permissions, the plugin has no background part, and there is nothing to run.

:::expert
The part is started as `trindle-plugin run ID background`, in a group of its own under your service manager (`systemd-run --user --scope`) when there is one. Ending the session that started it does not end it. When there is no service manager the log says so, and the part ends with the session. It is started once, with one keeper to each person, and sees the Wayland screen of the last login that still answers.
:::

## A plugin would not download or go in
Questions: why does it say it could not be downloaded; why does it say the archive could not be unpacked; why does it say its file is not the one the catalogue lists; why does it say the plugin's files could not be copied; why does it say it is not installed so there is nothing to update; why does it say the note that it came from Trindle's catalogue could not be written

Putting a plugin in takes four steps, and nothing is installed unless all four work.

1. Fetch its file from beside Trindle's own list, over a secure link only.
2. Check the file is exactly the one the signed list names, by its SHA-256 (a fingerprint of the file).
3. Unpack it into a temporary place and run the check of its `plugin.ini` there.
4. Put it in, over the old version if it is an update, and leave a note that it came from Trindle's list.

![The Plugins page of Apps. The card of a made-up plugin called Tides has red words saying it could not be downloaded because the host could not be resolved, with a round help button.](img/plugins-download-failed.png "What you will see: A plugin that could not be downloaded.")

- **"It could not be downloaded: ..."** The reason follows, as the downloading program gave it. It is usually the internet: there is no connection, the address did not answer, or the file was too big or took more than half an hour. A file bigger than 500 MB is refused. Check the connection and press the button again.
- **"curl could not be started: ..."** The program that downloads is missing or would not run.
- **"Its file is not the one the catalogue lists (the SHA-256 differs), so nothing was installed."** What came is not what was signed. It was thrown away. This is what protects you from a changed file. Try again later; if it keeps happening, tell Trindle with Report a problem.

![The Plugins page of Apps. The card of a made-up plugin called Tides has red words saying its file is not the one the catalogue lists, so nothing was installed, with a round help button.](img/plugins-hash.png "What you will see: A plugin file that is not the one listed.")

- **"The archive could not be unpacked: ..."**, **"tar could not be started: ..."** The file arrived but would not open. The first line the unpacking program said follows.

- **"The archive has no plugin.ini at its top, or in the one folder at its top."** The file opened, but it is not shaped like a plugin.
- **"The archive holds ..., not ..., so nothing was installed."** The plugin inside has another id from the one the list named. It is refused.
- **"The plugin's files could not be copied."** The last step failed, usually because the disk is full or the plugins' folder cannot be written. Nothing was put in.

![The Plugins page of Apps. The card of a made-up plugin called Tides has red words saying the plugin's files could not be copied, with a round help button.](img/plugins-copy-failed.png "What you will see: Files that could not be copied.")

- **"It was put in, but the note that it came from Trindle's catalogue could not be written: ..."** The plugin is in, but without its note. Remove it and add it again.

![The Plugins page of Apps. The card of a made-up plugin called Tides has red words saying it was put in, but the note that it came from Trindle's catalogue could not be written, with the reason and a round help button.](img/plugins-install-note.png "What you will see: A plugin put in without its note.")

- **"It is not installed, so there is nothing to update."** An update was asked for a plugin that is not there. Install it first.
- **"No scratch folder could be made: ..."** There was nowhere to unpack it. The reason follows, usually a full disk.

:::expert
Archives are `.tar.zst`. Downloads use `curl` with `--proto =https` and `--proto-redir =https`, a size cap and a time cap of 1800 seconds for an archive. `tar` runs with `--no-same-owner --no-same-permissions` and drops a leading `/` and refuses names with `..`, so nothing lands outside the temporary folder. The temporary folder goes when the install ends, whatever the result. The note it came from the list is the one thing that tells Flow from the catalogue apart from any other plugin. That is why it is written only after the SHA-256 has matched.
:::

## The plugin list could not be used
Questions: why does it say Trindle's plugin catalogue was not used; why does it say the catalogue is not signed with Trindle's key; why does it say plugin updates are paused; what does Start again do; why does it say the last good one is shown; why does it say the catalogue is of a newer kind

Trindle reads the list of plugins from its own published address each time the Plugins page opens. Before it trusts the list, Apps checks the list's signature against the key that Trindle carries. A list that fails any check is not used, and a plugin on it is not offered. If an earlier good list is kept, it is shown, and the red words say so.

![The Plugins page of Apps. The Where plugins come from card has red words saying the plugin catalogue is not signed with Trindle's key, so it was not used, and that the last good one is shown. A round help button sits beside them.](img/plugins-list-refused.png "What you will see: A plugin list that was refused.")

- **"This Trindle has no key to check its plugin catalogue with, so it accepts none."** The key is missing from this copy of Trindle. Update Trindle from Apps, Updates.
- **"Trindle's plugin catalogue has no signature, so it was not used."** and **"The signature ... is not the size a signature is"**, **"is not signed with Trindle's key"**. The list came altered, or something between you and the address it is read from changed it. Try again later, and if it keeps happening use Report a problem.
- **"... does not say which number it is"** and **"... is older than the one accepted before (number ..., after ...)"**. Every list carries a rising number so an old list cannot be put back. Try again later.

![The Plugins page of Apps. The Where plugins come from card has red words saying the catalogue is older than the one accepted before, giving both numbers, and that the last good one is shown. A round help button sits beside them.](img/plugins-list-older.png "What you will see: A plugin list that is older than one accepted before.")

- **"... is signed but cannot be read, so it was not used: ..."**. The reason follows: it is not text; it is more than 1 MB, too big to be a list; it has no `[Catalogue]` part with its Format; or it is of a newer kind than this Trindle reads. A newer kind means Trindle needs updating. In every case the plugin list is not used.
- **"... is left out: ..."**. One plugin on the list is left out because one of its entries is wrong. Examples are its id, its name and version, its archive name, its SHA256, a permission there is not, or its icon. The others are still offered.
- **"Plugin updates are paused: the plugin list is damaged."** Trindle keeps a record of the highest list number it has accepted. If that record is damaged, updates stop, rather than risk an old list. **Start again** asks for the administrator's password and begins the record afresh.

![The Plugins page of Apps. The Where plugins come from card has red words saying plugin updates are paused because the plugin list is damaged, with a round help button.](img/plugins-list-paused.png "What you will see: Plugin updates paused.")

- **"Not done: ..."** or **"The record could not be taken away: ..."** after pressing **Start again**. The password box was closed, or the record could not be taken away. Try again.

:::expert
The signature is Ed25519, checked against the public key built into Trindle. The record of the highest number accepted is kept on this computer so a list cannot be rolled back. A list is also refused when its signature checks but its text is not a catalogue (no `[Catalogue]` part with `Format=`). It is refused too when it is larger than the most a list may be (1 MB). Entries in a list for another plugin interface are left out quietly, since a newer list may hold plugins this Trindle cannot run.
:::

## Related

- [Apps](apps.html#plugins): adding, updating and removing plugins.
- [Apps, A plugin is not working](apps.html#a-plugin-is-not-working): removing, permissions and Pocketkey.
- [Flow](flow.html): a plugin that shares one mouse and keyboard.
