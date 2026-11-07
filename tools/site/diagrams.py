#!/usr/bin/env python3
"""Draw the diagrams for the help site into site/content/img/*.svg.

    python3 tools/site/diagrams.py

Each diagram is a narrow picture (480 wide) so its words stay readable on a
phone, on Trindle's dark blue, with the same colours as the site. Words
are 16 to 20 units high, and every pair of colours used for words is at
least 4.5 to one (the site's checks hold the same pairs). Each file carries
a title and a description for screen readers; the page gives the alt text.
"""
from pathlib import Path
from xml.sax.saxutils import escape

OUT = Path(__file__).resolve().parents[2] / "site" / "content" / "img"

BG, CARD, CARD2 = "#101825", "#16223a", "#1e2d4d"
EDGE, TEXT, MUTED, ACCENT, INK = "#7f8fb0", "#e8edf5", "#b0bace", "#ffc61a", "#14100a"
W = 480


class Svg:
    def __init__(self, h, title, desc):
        self.h, self.title, self.desc, self.parts = h, title, desc, []

    def add(self, s):
        self.parts.append(s)

    def text(self, x, y, lines, size=17, fill=TEXT, anchor="middle", weight="400", gap=1.3):
        if isinstance(lines, str):
            lines = [lines]
        for i, line in enumerate(lines):
            self.add('<text x="%g" y="%g" font-size="%g" fill="%s" text-anchor="%s" font-weight="%s">%s</text>' % (
                x, y + i * size * gap, size, fill, anchor, weight, escape(line)))

    def box(self, x, y, w, h, lines, fill=CARD, stroke=EDGE, ink=TEXT, size=17, weight="400", dash=False):
        self.add('<rect x="%g" y="%g" width="%g" height="%g" rx="14" fill="%s" stroke="%s" stroke-width="2"%s/>' % (
            x, y, w, h, fill, stroke, ' stroke-dasharray="7 5"' if dash else ""))
        n = len(lines) if isinstance(lines, list) else 1
        self.text(x + w / 2, y + h / 2 - (n - 1) * size * 0.65 + size * 0.34, lines, size, ink, weight=weight)

    def arrow(self, x1, y1, x2, y2, colour=MUTED):
        self.add('<line x1="%g" y1="%g" x2="%g" y2="%g" stroke="%s" stroke-width="3" marker-end="url(#a)"/>' % (x1, y1, x2, y2, colour))

    def save(self, name):
        head = ('<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 %d %d" width="%d" height="%d" role="img" '
                'font-family="Helvetica, Arial, sans-serif"><title>%s</title><desc>%s</desc>'
                '<defs><marker id="a" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="7" markerHeight="7" orient="auto-start-reverse">'
                '<path d="M0 0L10 5L0 10z" fill="%s"/></marker></defs><rect width="%d" height="%d" fill="%s"/>' % (
                    W, self.h, W, self.h, escape(self.title), escape(self.desc), MUTED, W, self.h, BG))
        (OUT / name).write_text(head + "".join(self.parts) + "</svg>\n", encoding="utf-8")


def dot(s, x, y, n):
    s.add('<circle cx="%g" cy="%g" r="15" fill="%s"/>' % (x, y, ACCENT))
    s.text(x, y + 6, str(n), 18, INK, weight="700")


def ring_layout():
    s = Svg(610, "The ring, labelled", "A circle with four numbered parts: the middle, the favourites, the open windows and the tray.")
    cx, cy = 240, 215
    s.add('<circle cx="%d" cy="%d" r="195" fill="%s" stroke="%s" stroke-width="2"/>' % (cx, cy, CARD, EDGE))
    s.add('<circle cx="%d" cy="%d" r="150" fill="%s" stroke="%s" stroke-width="2"/>' % (cx, cy, CARD2, EDGE))
    s.add('<circle cx="%d" cy="%d" r="105" fill="%s" stroke="%s" stroke-width="2"/>' % (cx, cy, CARD, EDGE))
    s.add('<circle cx="%d" cy="%d" r="62" fill="%s" stroke="%s" stroke-width="2"/>' % (cx, cy, BG, EDGE))
    s.text(cx, cy + 4, "10:42", 24, TEXT, weight="700")
    s.text(cx, cy + 28, "Sun 18 Oct", 14, MUTED)
    import math
    # little round buttons in the middle band, favourites, windows
    def ring_dots(r, count, size, fill, start=-90):
        for i in range(count):
            a = math.radians(start + i * 360 / count)
            s.add('<circle cx="%.1f" cy="%.1f" r="%d" fill="%s" stroke="%s" stroke-width="2"/>' % (cx + r * math.cos(a), cy + r * math.sin(a), size, fill, EDGE))
    ring_dots(83, 8, 9, CARD2)
    ring_dots(128, 8, 14, BG, -112)
    ring_dots(173, 5, 14, CARD2, -60)
    # the tray arc at the bottom
    s.add('<path d="M %.1f %.1f A 195 195 0 0 1 %.1f %.1f" fill="none" stroke="%s" stroke-width="10" stroke-linecap="round"/>' % (
        cx + 195 * math.cos(math.radians(65)), cy + 195 * math.sin(math.radians(65)),
        cx + 195 * math.cos(math.radians(115)), cy + 195 * math.sin(math.radians(115)), ACCENT))
    dot(s, cx, cy - 36, 1)
    dot(s, cx - 128 * 0.0, cy - 128 + 0, 2)
    dot(s, cx + 120, cy - 135, 3)
    dot(s, cx, cy + 195, 4)
    rows = [("1", "The middle: time, battery, small round buttons"), ("2", "Favourites: apps you keep close"),
            ("3", "Open windows: click one to switch to it"), ("4", "Tray: icons that some programs show")]
    for i, (n, t) in enumerate(rows):
        y = 468 + i * 36
        dot(s, 40, y - 6, n)
        s.text(68, y, t, 17, TEXT, anchor="start")
    s.save("ring-layout.svg")


def app_folder():
    s = Svg(470, "An app in a folder of its own", "Your computer holds a box for each app. Inside the box are the app and everything it needs. Your files are outside, and only open to the app if you allow it.")
    s.text(240, 34, "Your computer", 20, TEXT, weight="700")
    s.box(24, 52, 432, 236, [""], fill=CARD, stroke=EDGE)
    s.box(44, 76, 250, 190, [""], fill=CARD2, stroke=ACCENT)
    s.text(169, 104, "A folder for one app", 17, ACCENT, weight="700")
    s.box(62, 122, 214, 40, ["The app"], fill=BG)
    s.box(62, 172, 214, 40, ["Its own libraries"], fill=BG)
    s.box(62, 222, 214, 36, ["Its own settings"], fill=BG)
    s.box(318, 76, 120, 84, ["Your", "Documents"], fill=BG)
    s.box(318, 182, 120, 84, ["Your", "Pictures"], fill=BG)
    s.add('<line x1="294" y1="118" x2="318" y2="118" stroke="%s" stroke-width="3" stroke-dasharray="3 6"/>' % MUTED)
    s.text(240, 322, "The dotted line is a permission.", 17, TEXT)
    s.text(240, 346, "It is shut until you switch it on.", 17, TEXT)
    s.box(44, 374, 392, 70, ["Remove the app and the whole", "folder goes. Nothing is left behind."], fill=CARD, stroke=ACCENT, size=17)
    s.save("app-folder.svg")


def lock_flow():
    s = Svg(560, "How the lock screen lets you back in", "The screen locks. You type your password or PIN, or tap yes on your phone if Pocketkey is on. The screen opens. Notifications wait until then.")
    s.box(60, 20, 360, 56, ["The screen locks"], size=19, weight="700")
    s.arrow(240, 76, 240, 108)
    s.box(60, 110, 360, 76, ["It shows only the time,", "the date and a password box"], fill=CARD2)
    s.arrow(240, 186, 240, 218)
    s.box(20, 220, 210, 96, ["Type your", "password or", "four-digit PIN"], fill=CARD)
    s.box(250, 220, 210, 96, ["Or tap yes on", "your phone", "(Pocketkey, if on)"], fill=CARD, dash=True)
    s.arrow(125, 316, 200, 350)
    s.arrow(355, 316, 280, 350)
    s.box(60, 352, 360, 56, ["The screen opens"], fill=CARD2, stroke=ACCENT, weight="700", size=19)
    s.box(40, 440, 400, 94, ["While it is locked, no notification", "shows, not even its words.", "They wait for you in the ring."], fill=CARD, size=17)
    s.save("lock-flow.svg")


def report_flow():
    s = Svg(640, "Where a report goes", "Two routes. In the Report a problem window you write what you were doing and press Send, which posts the report to Buildmorphic, or Save or Copy under Show details. After a crash Trindle asks whether to send a report, and you press Send or Don't send. Nothing is sent until you press Send.")
    s.text(240, 30, "Report a problem window", 19, ACCENT, weight="700")
    s.box(30, 48, 420, 50, ["You write what you were doing"], fill=CARD2)
    s.arrow(240, 98, 240, 126)
    s.box(30, 128, 200, 70, ["Send", "to Buildmorphic"], fill=CARD)
    s.box(250, 128, 200, 70, ["Save or Copy", "under Show details"], fill=CARD)
    s.arrow(130, 198, 200, 230)
    s.arrow(350, 198, 280, 230)
    s.box(30, 232, 420, 56, ["Buildmorphic keeps it for the project"], fill=CARD2, stroke=ACCENT)
    s.text(240, 318, "Nothing is sent until you press Send.", 17, TEXT)
    s.add('<line x1="30" y1="346" x2="450" y2="346" stroke="%s" stroke-width="2"/>' % EDGE)
    s.text(240, 384, "After a crash", 19, ACCENT, weight="700")
    s.box(30, 402, 420, 50, ["A Trindle program crashes"], fill=CARD2)
    s.arrow(240, 452, 240, 480)
    s.box(30, 482, 420, 50, ["Trindle asks: Send a report?"], fill=CARD, dash=True)
    s.arrow(240, 532, 240, 560)
    s.box(30, 562, 420, 50, ["You press Send or Don't send"], fill=CARD)
    s.save("report-flow.svg")


def install_steps():
    s = Svg(560, "Five steps from the image to a running Trindle", "Download the image, write it to a USB stick, start the computer from the stick, try Trindle, then install it.")
    steps = [("1", "Download the image", "One file, about 1 GB"), ("2", "Write it to a USB stick", "Everything on the stick is replaced"),
             ("3", "Start from the stick", "Hold the boot menu key at switch-on"), ("4", "Try Trindle", "Nothing is written to the disk"),
             ("5", "Press Install Trindle", "Nothing changes until the last page")]
    for i, (n, a, b) in enumerate(steps):
        y = 14 + i * 108
        s.box(20, y, 440, 84, [""], fill=CARD2 if i == 4 else CARD, stroke=ACCENT if i == 4 else EDGE)
        dot(s, 58, y + 42, n)
        s.text(90, y + 38, a, 19, TEXT, anchor="start", weight="700")
        s.text(90, y + 64, b, 16, MUTED, anchor="start")
        if i < 4:
            s.arrow(240, y + 84, 240, y + 106)
    s.save("install-steps.svg")


def drives_password():
    s = Svg(520, "Every drive change asks for a password", "You ask for a change. Drives asks for an administrator password every time. With the password the change is made. Without it nothing changes.")
    s.box(60, 20, 360, 56, ["You ask to change a drive"], size=19, weight="700")
    s.arrow(240, 76, 240, 108)
    s.box(40, 110, 400, 76, ["A box asks for an administrator's", "password, naming the drive"], fill=CARD2)
    s.arrow(150, 186, 120, 238)
    s.arrow(330, 186, 360, 238)
    s.box(20, 240, 210, 100, ["Password given", "The change is made,", "and the row says what"], fill=CARD, stroke=ACCENT)
    s.box(250, 240, 210, 100, ["Box closed", "Nothing changes,", "and a label says so"], fill=CARD)
    s.box(40, 380, 400, 110, ["This holds for the system disk, a", "second disk and a USB stick. A", "password given earlier does not count."], fill=CARD, size=17)
    s.save("drives-password.svg")


def flow_two():
    s = Svg(470, "How Flow is meant to work", "Two computers side by side. The pointer leaves the right edge of the first screen and appears on the left edge of the second. The keyboard and the clipboard follow. This is the plan: Flow is not built yet.")
    s.text(240, 30, "The plan: Flow is not built yet", 18, ACCENT, weight="700")
    s.box(20, 56, 190, 130, ["Computer A", "keyboard and mouse", "are here"], fill=CARD)
    s.box(270, 56, 190, 130, ["Computer B", "no keyboard", "or mouse of its own"], fill=CARD)
    s.arrow(210, 120, 268, 120, ACCENT)
    s.text(240, 100, "pointer", 15, TEXT)
    s.box(40, 222, 400, 56, ["The keyboard and the clipboard follow"], fill=CARD2)
    s.box(40, 298, 400, 56, ["Push the pointer back to come home"], fill=CARD2)
    s.box(40, 374, 400, 70, ["Hold Ctrl, Alt and Shift for a", "second and they come home"], fill=CARD, stroke=ACCENT)
    s.save("flow-two.svg")


def flow_allow():
    s = Svg(470, "What Flow may and may not do", "Flow may control your keyboard, mouse and clipboard once you allow it. It cannot see your screen or your other windows. You can take it back in three ways.")
    s.text(240, 30, "The plan: Flow is not built yet", 18, ACCENT, weight="700")
    s.box(20, 52, 210, 110, ["Flow may,", "once you allow it:", "use your keyboard,", "mouse and clipboard"], fill=CARD)
    s.box(250, 52, 210, 110, ["Flow cannot,", "even when allowed:", "see your screen or", "your other windows"], fill=CARD, dash=True)
    s.text(240, 206, "You can always take it back", 19, ACCENT, weight="700")
    s.box(20, 224, 440, 56, ["A mark in every screen's corner names Flow"], fill=CARD2)
    s.box(20, 292, 440, 56, ["Locking the screen hands them back"], fill=CARD2)
    s.box(20, 360, 440, 70, ["Hold Ctrl, Alt and Shift for a second"], fill=CARD2, stroke=ACCENT)
    s.save("flow-allow.svg")


def disk_parts():
    s = Svg(440, "A disk and its parts", "One disk drawn as a bar of three parts: an older system that is kept, Trindle, and free space for a new part. Each part is called a partition.")
    s.text(240, 30, "One disk, drawn as a bar (an example)", 18, ACCENT, weight="700")
    s.box(20, 52, 190, 70, ["Another", "system, kept"], fill=CARD)
    s.box(214, 52, 150, 70, ["Trindle"], fill=CARD2, stroke=ACCENT, weight="700")
    s.box(368, 52, 92, 70, ["Free", "space"], fill=BG, dash=True)
    s.text(240, 166, "Each block is a partition,", 17, TEXT)
    s.text(240, 190, "one named part of the disk.", 17, TEXT)
    s.box(20, 224, 440, 78, ["The installer, or Drives, can make a", "part smaller to open up free space"], fill=CARD)
    s.box(20, 322, 440, 78, ["Both systems then show in the", "boot menu when the computer starts"], fill=CARD2)
    s.save("disk-parts.svg")


def installer_questions():
    s = Svg(560, "The questions the installer asks", "Four questions in order: language, the disk, encryption and you. Then a last page called Ready that says the whole plan back to you. Nothing is written until you press Install.")
    rows = [("1", "Language", "The keyboard and time follow it"), ("2", "The disk", "Whole disk, or share it"),
            ("3", "Encryption", "A password, the TPM chip, or none"), ("4", "You", "Your name, sign-in name, password"),
            ("5", "Ready", "The whole plan, in sentences")]
    for i, (n, a, b) in enumerate(rows):
        y = 14 + i * 100
        s.box(20, y, 440, 76, [""], fill=CARD2 if i == 4 else CARD, stroke=ACCENT if i == 4 else EDGE)
        dot(s, 58, y + 38, n)
        s.text(90, y + 34, a, 19, TEXT, anchor="start", weight="700")
        s.text(90, y + 58, b, 16, MUTED, anchor="start")
        if i < 4:
            s.arrow(240, y + 76, 240, y + 98)
    s.text(240, 540, "Nothing is written until you press Install.", 17, TEXT)
    s.save("installer-questions.svg")


def help_path():
    s = Svg(560, "What to do when something goes wrong", "Read the label that Trindle shows where you are working. If that does not help, look on the Troubleshooting page. If you are still stuck, write a report.")
    steps = [("1", "Read the label", "It says why, right where you work"), ("2", "Look at Troubleshooting", "The common problems and fixes"),
             ("3", "Still stuck? Write a report", "Report a problem, in the ring")]
    for i, (n, a, b) in enumerate(steps):
        y = 20 + i * 126
        s.box(20, y, 440, 96, [""], fill=CARD2 if i == 2 else CARD, stroke=ACCENT if i == 2 else EDGE)
        dot(s, 58, y + 48, n)
        s.text(90, y + 44, a, 19, TEXT, anchor="start", weight="700")
        s.text(90, y + 72, b, 16, MUTED, anchor="start")
        if i < 2:
            s.arrow(240, y + 96, 240, y + 124)
    s.box(20, 410, 440, 120, ["A report holds only your words, part", "of Trindle's log, the version and", "a summary of the computer."], fill=CARD, size=17)
    s.save("help-path.svg")


def two_panes():
    s = Svg(430, "Folders with one pane or two", "Single: one list of files. Dual: two lists side by side with a strip of buttons between them.")
    s.text(120, 30, "Single", 19, ACCENT, weight="700")
    s.box(20, 44, 200, 190, ["One list", "of files"], fill=CARD)
    s.text(360, 30, "Dual", 19, ACCENT, weight="700")
    s.box(250, 44, 90, 190, ["Left", "list"], fill=CARD)
    s.box(344, 44, 26, 190, [""], fill=CARD2)
    s.box(374, 44, 90, 190, ["Right", "list"], fill=CARD)
    s.text(240, 272, "The strip between the panes can show this", 16, TEXT)
    s.text(240, 294, "folder in the other pane, swap the panes,", 16, TEXT)
    s.text(240, 316, "pick what differs, link them and compare.", 16, TEXT)
    s.box(40, 346, 400, 60, ["F3 switches between one and two"], fill=CARD2, stroke=ACCENT)
    s.save("two-panes.svg")


def ai_fence():
    s = Svg(440, "The fence round an AI program", "An AI program runs in a fence. Inside it sees the system, an empty private home and the folders you shared. Your documents, keys and passwords are outside, out of its sight.")
    s.text(240, 32, "Your computer", 20, TEXT, weight="700")
    s.box(24, 50, 190, 250, [""], fill=CARD2, stroke=ACCENT)
    s.text(119, 78, "The fence", 18, ACCENT, weight="700")
    s.box(40, 96, 158, 44, ["The AI program"], fill=BG)
    s.box(40, 150, 158, 44, ["The system"], fill=BG)
    s.box(40, 204, 158, 44, ["An empty home"], fill=BG)
    s.box(40, 258, 158, 32, ["Shared folders"], fill=BG, size=16)
    s.box(280, 70, 176, 56, ["Your documents"], fill=CARD, dash=True)
    s.box(280, 140, 176, 56, ["Your ssh keys"], fill=CARD, dash=True)
    s.box(280, 210, 176, 56, ["Your passwords", "and browser"], fill=CARD, dash=True)
    s.add('<line x1="214" y1="168" x2="280" y2="168" stroke="%s" stroke-width="3" stroke-dasharray="3 6"/>' % MUTED)
    s.text(240, 336, "Outside the fence, out of its sight.", 17, TEXT)
    s.box(44, 358, 392, 60, ["You choose which folders to share."], fill=CARD, stroke=ACCENT, size=17)
    s.save("ai-fence.svg")

def errors_path():
    s = Svg(520, "From red words to the fix", "When something fails, red words say so where you are working, with a round More button beside them. More opens Help at the section for those words: what they mean, the likely causes, the steps to fix it, and the error exactly as the system said it.")
    steps = [("1", "Red words where it failed", "The system's own words, in Apps"), ("2", "Press More beside them", "The small round button"),
             ("3", "Help opens at that section", "Meaning, causes and steps")]
    for i, (n, a, b) in enumerate(steps):
        y = 20 + i * 126
        s.box(20, y, 440, 96, [""], fill=CARD2 if i == 2 else CARD, stroke=ACCENT if i == 2 else EDGE)
        dot(s, 58, y + 48, n)
        s.text(90, y + 44, a, 19, TEXT, anchor="start", weight="700")
        s.text(90, y + 72, b, 16, MUTED, anchor="start")
        if i < 2:
            s.arrow(240, y + 96, 240, y + 124)
    s.box(20, 410, 440, 90, ["The error is shown as the system said", "it, so it can be copied into a report."], fill=CARD, size=17)
    s.save("errors-path.svg")


def game_start():
    s = Svg(560, "When a game will not start", "Try four things in order: restart Steam, wait while the first start builds its shaders, try another Proton version, and sign in to the game's own launcher.")
    steps = [("1", "Restart Steam", "Close it fully, then open it again"), ("2", "Wait", "A first start builds its shaders"),
             ("3", "Try another Proton", "Properties, then Compatibility"), ("4", "Sign in to its launcher", "Alt-Tab finds a hidden window")]
    for i, (n, a, b) in enumerate(steps):
        y = 20 + i * 132
        s.box(20, y, 440, 100, [""], fill=CARD)
        dot(s, 58, y + 50, n)
        s.text(90, y + 46, a, 19, TEXT, anchor="start", weight="700")
        s.text(90, y + 74, b, 16, MUTED, anchor="start")
        if i < 3:
            s.arrow(240, y + 100, 240, y + 130)
    s.save("game-start.svg")


def in_use_dots():
    s = Svg(500, "What the in-use dots mean", "Seven colours, one for each thing a program can be using: orange the microphone, green the camera, blue the screen being recorded, violet the mouse and keyboard, aqua the files you copied, magenta the home network, yellow scanners and USB devices.")
    key = [("#fe9700", "Microphone"), ("#019587", "Camera"), ("#2295f1", "Screen being recorded"), ("#9b6fd0", "Mouse and keyboard"),
           ("#26b5a8", "Files you copied"), ("#d4537e", "Home network"), ("#e8c11a", "Scanners and USB devices")]
    s.text(240, 36, "Each colour means one thing", 19, ACCENT, weight="700")
    for i, (colour, words) in enumerate(key):
        y = 62 + i * 60
        s.box(20, y, 440, 48, [""], fill=CARD)
        s.add('<circle cx="52" cy="%g" r="10" fill="%s"/>' % (y + 24, colour))
        s.text(80, y + 30, words, 18, TEXT, anchor="start")
    s.save("in-use-dots.svg")



if __name__ == "__main__":
    OUT.mkdir(parents=True, exist_ok=True)
    for f in (ring_layout, app_folder, lock_flow, report_flow, install_steps, drives_password, flow_two, flow_allow, disk_parts, installer_questions, help_path, two_panes, ai_fence, errors_path, game_start, in_use_dots):
        f()
    print("diagrams written to", OUT)
