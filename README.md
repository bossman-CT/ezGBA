# ezGBA

**No hotkeys. No combos. No menus to get lost in. Simple, simple, simple.**

ezGBA turns an Anbernic RG SP into a Game Boy Advance that anyone can pick up and play.
Made for kids, and for grown-ups who just want a simple GBA.

<p align="center">
  <img src="media/shelf-advance-wars.png" width="45%" alt="The shelf: Advance Wars selected, its box art filling the screen above the carts">
  <img src="media/shelf-wario-land.png" width="45%" alt="The shelf: Wario Land 4 selected">
</p>

## See it in action

| Pick a game, play, eject | Brightness on L2/R2 | Close the lid, pick up where you left off |
|:---:|:---:|:---:|
| <img src="media/play-and-eject.webp" width="240" alt="Pressing A starts Zelda; MENU ejects back to the shelf"> | <img src="media/brightness.webp" width="240" alt="L2 and R2 dim and brighten the screen"> | <img src="media/lid-close-and-resume.webp" width="240" alt="Closing the lid mid-game, reopening, then powering on from off and resuming"> |
| A starts it. MENU ejects it. | One press, one step. | Your game is always saved. |

## How it works

| Button | What it does |
|---|---|
| **D-pad** | Pick a game |
| **A** | Play it |
| **MENU** | In a game: save and go back to your games. On the shelf: settings. |
| **L2 / R2** | Screen darker / brighter |
| **Close the lid** | Save and sleep. Open it to keep playing. |

That's everything. Every button does one thing, on its own. No holding, no combos.

## What's different from slot.

ezGBA is built on [slot.](https://github.com/BrandonKowalski/slot). Here's what changed:

- **No hotkeys.** No button combos to learn or hit by accident.
- **L2/R2 control brightness.** In slot. they rewind and fast-forward the game, which is
  easy to press by accident mid-game.
- **One tap of MENU saves and ejects.** slot. needs MENU held for about a second.
- **The game's box art fills the screen.** Scroll to a game and see its cover, so you can
  find games by picture.
- **Your own colors.** Give each console its own background color in `System/theme.txt`.
- **12-hour clock.** Shows `4:39 PM`, not `16:39`.
- **A tiny settings menu.** Just Date & Time and About. Nothing in it can mess anything up.

Everything else, like lid-close saving, wireless trading, and the cartridge shelf, is slot.'s
work, unchanged.

## Settings

Tap **MENU** on the shelf to set the date and time, or see About. It also reminds you that
L2 / R2 change the brightness.

For a couple more options, edit `System/theme.txt` on the SD card:

| Line | What it does |
|---|---|
| `menu off` | Hide the settings menu, so little hands can't change anything |
| `scrim #F7E7CE` | Background color behind the shelf |

## Setup

1. Set up your RG SP using [slot.'s install guide](https://slot.kowalski.io).
2. Download the latest ezGBA from [releases](../../releases).
3. Unzip it and copy it onto the second SD card, just like slot.
4. **Add box art (optional).** Box art isn't included, since it belongs to the publishers.
   Put a 720×480 PNG in `Backdrops/GBA/`, named the same as the game, for example
   `Backdrops/GBA/Pokemon - FireRed Version (USA).png`. Keep the art in the top 270 pixels
   so the carts don't cover it.

## AI disclosure

Built with Claude's help, on top of slot., which was also built with Claude's help. Every
change was tested on real hardware.

## Credit

The hard work - the shelf, the look, the feel - is
[BrandonKowalski/slot](https://github.com/BrandonKowalski/slot). ezGBA is a small layer on top.
