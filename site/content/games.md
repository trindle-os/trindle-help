---
title: Playing games
summary: What works, what Trindle does for you, what to try first when a game will not start, and what cannot run.
group: Using Trindle
order: 55
---

This page is about Steam and other game launchers. It says only what has been checked. Where something is not built yet, it says so.

## What works

Questions: can I play games on Trindle; does Steam work on Trindle; can I play Windows games; does Proton work; which launchers work

- **Steam.** Install it from Apps. It runs as a Flatpak, in a box of its own.
- **Windows games.** Steam runs them through Proton. One was played start to finish on a real computer.
- **Launchers.** Ubisoft Connect was checked. Others, such as EA, Epic and Battle.net, usually work inside Steam, but each changes often and can break after its own update.
- **Not checked yet.** Games made for Linux, DirectX 12 games and older 32-bit games.

## What Trindle does for you

Questions: what does Trindle do for games; why does my game crash on load; does Alt-Tab work in a game; will my screen go to sleep in a game; will the mouse stay in the game

- **Memory limit.** Raised, so big Windows games do not crash when they load.
- **File limit.** Raised, so a game may open far more files at once than the usual limit allows.
- **Mouse.** A game that holds the mouse keeps it. It never crosses to a second screen.
- **Alt-Tab.** The mouse is free while you are away, and held again when you come back.
- **Full screen.** A full-screen game stays full screen behind the window you switched to. It does not go black.
- **Screen.** It stays awake while a full-screen game is in front.
- **Resolution.** A game that changes it is shown at the size it chose, scaled to fit. The screen itself never changes, so nothing needs putting back if the game crashes.
- **Controllers.** See Controllers below.

:::expert
The kernel's memory map limit (vm.max_map_count) is 2147483642, the figure Valve names for Proton. The most files a game may open is 1048576. The usual limit of 1024 stays the starting point, and a program may raise itself as far as that ceiling.
:::

## A game will not start

Questions: why will my game not start; a steam game will not launch; steam game shows a black screen; my game crashes at start; ubisoft connect does not appear

Try these in order.

![A drawn diagram of four steps in order: restart Steam, wait while the first start builds its shaders, try another Proton version, and sign in to the game's own launcher.](img/game-start.svg "What you will see: the four things to try, in order. It is drawn to explain, not a screenshot.")

1. Restart Steam. Close it fully and open it again. A Steam that has run for days can stop games starting. After a restart they did.
2. Wait. The first start of a game builds its shaders. The screen can stay black for a few minutes. Let it finish.
3. Check the Proton version. In Steam, open the game's Properties, then Compatibility, and try another version, such as Proton Experimental.
4. Sign in to the game's own launcher, such as Ubisoft Connect. Its window can hide behind the full-screen game. Press Alt-Tab to find it.

## Games with anti-cheat

Questions: do games with anti-cheat work; can I play Valorant on Linux; does Easy Anti-Cheat work; does BattlEye work; why does my game kick me out

- **Kernel-level anti-cheat cannot run on Linux at all.** Vanguard is one. Nothing Trindle does can change that.
- **Easy Anti-Cheat and BattlEye** work only for a game whose publisher has allowed Linux.
- **Look before you buy.** The site areweanticheatyet.com lists games one by one.
- A game that starts and then throws you out may be refusing Linux.

## Controllers

Questions: does my controller work; does a PlayStation controller work; does an Xbox controller work; what does missing permissions for input devices mean; why does Steam Input not work

- **The keyboard and mouse work.**
- **PlayStation, Switch and many Xbox controllers, and Steam Input, may not work in games yet.** Steam may say "Missing permissions for input devices".
- **Trindle has a small service for game controllers.** The parts that use it, in the desktop and in Steam, are not finished.
- **Settings, Games** keeps your controller's button changes. They do not reach games yet.

## HDR and variable refresh

Questions: does HDR work; does variable refresh rate work; does freesync or gsync work

- **HDR.** Not yet.
- **Variable refresh rate.** Not yet.
