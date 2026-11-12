---
title: Settings
summary: Every setting in one place, page by page, with search and a help button on every page.
group: Using Trindle
order: 40
---

Open Settings from the ring. A side bar down the left lists the pages. A search box above it lets you find a setting by name.

Each page has a small i beside its name. Press it to open the part of Help about that page.

![The Ring page in Settings. The side bar lists the pages in groups. The page has cards for opening the ring, notifications and the clipboard.](img/settings.png "What you will see: Settings, on the Ring page.")

Settings save as soon as you change them. There are no Save buttons to look for.

## The pages

**Personal**

- **Ring.** Settings for how the ring opens, notifications, the clipboard history, voice, and where things open.
- **Wallpaper.** Choose your wallpaper here. A still picture can be up to 4K (3840 by 2160) and 20 MB. A moving one can be up to 200 MB. Trindle leaves out a bigger file as soon as you pick it, and a label tells you why.
- **Appearance.** Your theme and highlight colour.
- **Sounds.**

**Devices**

- **Display.** Screen size and other display settings.
- **Audio, Bluetooth and Printers.**
- **Keyboard and mouse.** Layouts, and changing what any key or button does.
- **Power.** Battery, screen dimming and locking.

**Network**

- **Network, Tailscale and Blocking.**

**System**

- **Users, Snapshots, Region and language, and Date and time.**

**About**

- **About this computer, Thanks and Help.**

Some pages appear only when they are needed, such as the page for your plugins and the one for crash reports.

## Appearance and themes

A theme changes how everything in Trindle looks: colours, corners, sizes and spacing. Each theme has a small picture of its look. Press one and every window changes at once.

Five come with Trindle: its own deep blue look, Bold, Compact, Midnight and Paper. A theme can include both a dark look and a light one. If it does, Dark and Light appear beside its picture.

![The Appearance page in Settings, showing theme cards for Trindle, Bold and Compact, each with a small picture of the look.](img/settings-appearance.png "What you will see: the Appearance page, with a card for each theme. Keep appears at the bottom of the window.")

A new theme is only a trial at first. If you don't press Keep within 30 seconds, your old look comes back by itself. So a theme you can't read never stays.

A theme can't make words too small or too faint to read. Your highlight colour stays the one you chose.

## Keyboard and mouse

You can change any key or mouse button here. A key can become another key or button, several keys pressed together, a run of keys, or nothing. It can also do one thing when you tap it and another when you hold it.

Ready-made swaps, such as Ctrl and Caps Lock, are a single switch each. A change applies everywhere, starting from the login screen.

![The Keyboard and mouse page in Settings, with cards for the layout, other keys, keys and buttons, programs' keyboards and game controllers.](img/settings-keyboard.png "What you will see: the Keyboard and mouse page.")

These rules stop you locking yourself out.

- Ctrl, Alt and an F key always open a text console, whatever you change. A console is a plain screen of words. Type `trindle-settings --keys default` there to put every key back.
- Trindle never changes Backspace, Tab, Enter, Escape and Shift on the lock screen and the login screen. You can always type your password.
- Keyboards and mice that a program creates, such as remapping tools, still type. They can't press protected buttons or type into password prompts. You can switch on Counts as you for one of them, by name.
- Each game controller has its own profile on the Games page.
- Trindle doesn't have layers yet. A layer would let one key switch all the other keys to a second set.

## Power and locking

Power has the battery and the power mode. It also has the settings for keeping the screen awake, when the screen dims, and when the moving clock shows.

You switch the screen lock on in the Locking card, and set how long Trindle waits before it locks.

![The Power page in Settings with a battery picture, the power mode, screen dimming times and a Locking card.](img/settings-power.png "What you will see: the Power page. Locking is at the bottom.")

- [Read about the lock screen](lock-screen.html)

## AI programs

Settings has a page called AI programs. It stops programs such as Claude Code and Codex reaching your personal files.

In everyday use, this keeps AI programs away from your personal files, for example if one is tricked into collecting your bank statements and sending them out. The protection has limits and is not a guarantee. A determined program, a flaw in the computer's software, or you approving a request or sharing a folder can still let something through.

![The fence around an AI program: inside it are the program, the system, an empty home and the folders you shared; your documents, ssh keys and passwords are outside.](img/ai-fence.svg "What you will see: the fence around an AI program, with your own files outside it.")

Here is what tests of this fence have shown, using a made-up home folder:

- A fenced program sees an empty home folder. Your documents, ssh keys, keyring and browser logins aren't there for it to read.
- It sees only the folders you share, in the same place as they are for you.
- Paths that climb up with two dots, and links in a shared folder that point outside it, lead nowhere.
- It can't see your other programs, reach your session, or make another box inside its box.
- Any program it installs and runs is fenced too.
- It can't get keys and tokens from your session. It gets only a key that you type on the page.
- It can't give itself a folder by writing a settings file.

It can still see the rest of the system, so it can read logs and run commands. Its network access is open unless you switch on the limit on the page.

To share a folder, right-click it in Folders and choose Share with AI programs. Hidden folders and folders that hold keys or mail can never be shared.

Only the programs listed on the page start inside the fence. A program started some other way isn't fenced. You can switch the fence off on the page. If you do, AI programs can see everything you can.

## Folders and Settings

Folders has its own settings inside Folders, behind the cog at the end of its top bar. Settings has no page for Folders.

- [Read about Folders](folders.html)

## Help inside Trindle

The Help page works even when you're offline. It uses the same plain wording as this site, and it has a search box for words such as Wi-Fi or printer.
