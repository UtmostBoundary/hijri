# Demo recording for the `hijri` CLI.
# Render with:  vhs assets/demo.tape   (produces assets/demo.gif)

Output assets/demo.gif

Set Shell "bash"
Set FontSize 20
Set Width 960
Set Height 640
Set Padding 24
Set Theme "Dracula"

# Set a clean prompt and clear the screen before recording the visible part.
Hide
Type "export PS1='$ '" Enter
Type "clear" Enter
Show

Sleep 700ms

Type "hijri" Sleep 400ms Enter
Sleep 1600ms

Type "hijri cal" Sleep 400ms Enter
Sleep 2400ms

Type "hijri convert 2026-06-18" Sleep 400ms Enter
Sleep 1800ms

Type "hijri cal ramadan 1448" Sleep 400ms Enter
Sleep 2800ms
