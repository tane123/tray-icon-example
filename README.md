You can run each of these examples via these commands

```
cargo run --example native
cargo run --example tao
cargo run --example hack_fix
```

Native is a simple implementation of a tray icon written without any third party tray icon libraries. Written to test weather this is an bug in MacOS or tray-icon crate.

Hack_fix is a very crude and hacky fix. Calling setVisible with false outside of the event loop and setting it to true inside the event loop seems to fix the issue. SetVisible recreates the icon?

Tao is a a stripped down copy of the Tao example in the tray-icon github. All copyright is attributed to them

##To reproduce:

1. Have another application in full screen mode off to the side
2. Run `cargo run --example tao` to run broken example
3. Look near you apple icon to see superimposed tray item.
   1. Note: if you have multiple screens it will be on you main display
   2. Moving focus to another screen will make it disappear

##Notes:
The text can sometimes be really hard to see as sometimes it is white and other times it is black.

It seems to be white when you have an application off to side.

If you have an application like terminal in fullscreen on the display that is not your main display and run the broken example from the fullscreen terminal window. It will be black. 🤔

##Screenshots:

Tao example with full screen app in focus:
![Full screen app](image.png)
Tao example with full screen app to the side:
![Alt text](image-1.png)
