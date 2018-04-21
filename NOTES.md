Notes
===

1.  The fact that you can have a `rust-toolchain` file is not well documented.
2.  Mixing stable and nightly isn't fun - neither is using old nightlies.
    -   Feature annotations/lack of feature annotations in particular.
3.  Volume shadows and self shadowing - particularly on curved surfaces - looks very weird.
4.  Automation is a really good idea.
5.  Profiling with Xcode works fairly well
6.  Be careful about dropping buffers


MVP
===

## Gameplay
[x] Basic chess rules
[ ] Detect checkmate/stalemate and end game
[x] Allow selling chess pieces
[x] Allow buying some chess pieces
[ ] AI for enemy player

## Art
[x] Basic 3D models for every piece
[x] Usable UI

## Sound
[ ] Some simple music - anything at all as long as it's listenable

## Screens
[ ] Title screen
[ ] Victory/defeat screen

## Packaging
[ ] macOS app
    [x] Fix resolution in release
[ ] Windows app

## Later on...
[ ] Sound effects
[ ] Linux app
[ ] 2-player support
[ ] Better lighting model
[ ] Do AI in separate thread
[ ] More complex chessy things (castling, en-passant...)
[ ] More interesting background
[ ] Prettier UI
