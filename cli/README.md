# Live App Icon for Mac

*Animated app icons in your Dock that can run an arbitrary shell script when clicked.*

<img width="800" src="https://github.com/fujidaiti/live-app-icon/assets/68946713/926a164d-164c-4b82-a254-48db7f8839b2"/>

## Requirements

- macOS 13 (Ventura) or higher
- Xcode (or at least `xcodebuild`) version 14.1 or higher
- Apple Development Certificate (Personal Team ID)
- Not too old `git`

If you have the latest version of Xcode, the above requirements should be met.

## Install laic

***laic (Live App Icon Creator)*** is a simple CLI tool to generate Live App Icons on macOS. There are 2 ways to install laic: using `cargo` from Rust or download a binary.

### Use cargo

If you are a Rust developer, the easiest way to install laic is with `cargo`, since it is published on [crate.io](https://crates.io/crates/laic).

```shell
cargo install laic
```

### Download a binary

There are pre-compiled binraies in [the release page](https://github.com/fujidaiti/live-app-icon/releases). You can manually download and install it in an appropriate location (e.g. `/usr/local/bin`). Select `laic-aarch64-apple-darwin` if you are using Apple Silicon Mac, or `laic-x86_64-apple-darwin` if Intel Mac.

The following one line command will automatically download and install laic to `/usr/local/bin` on Apple Silicon Mac:

```shell
curl -L -o laic https://github.com/fujidaiti/live-app-icon/releases/download/v0.1.0/laic-aarch64-apple-darwin && chmod +x ./laic && sudo mv ./laic /usr/local/bin
```

Or on Intel Mac:

```shell
curl -L -o laic https://github.com/fujidaiti/live-app-icon/releases/download/v0.1.0/laic-x86_64-apple-darwin && chmod +x ./laic && sudo mv ./laic /usr/local/bin
```

## Usage

laic requires at least 3 arguments:

- `--name` : Name of the app to be generated. Used as its display name in the dock and app launcher.
- `--gif` :  Path to an animated GIF to be used as the icon of the app. If the width and height of the image are not equal, the image will be resized to a square according to the specified [resize method](#resize-method).
- `--command` : Shell command to be executed when the generated app is clicked.

The below is an example to generate an animated app icon that is named as "GitHub" that, when clicked, opens the home page of GitHub in your default browser.

```shell
laic --name "GitHub" --gif path/to/your/gif --command "open https://github.com"
```

There are other options that laic accepts:

- `--resize-method` : Specifies how the given GIF will be resized to a square. See [Resize method](#resize-method) for more details.

- `--install-location` : Path to a directory the app to be installed. The default is `$HOME/Applications`.
- `--install-action` : Specify the action to be taken when the app is successfully installed. `launch` launches the app immediately, `open-in-finder` opens the directory where the app is installed in the Finder, and `none` does nothing. The default is `launch`.
- `--verbose` : Noisy logging, including all shell commands executed.

Run `laic --help` for more information.

### Resize methods

If the width and height of the given animated GIF are not equal, laic resizes it to a square according to a given resize method:

- `center-crop` : Crops the center of the image with a square whose sides are the same size as the shorter side of the image.
- `center-fit` : Puts the image on the center of a transparent square whose sides are the same size as the longer side of the image.

The default method is `center-fit`.

## After installation is finished

### Keep the app in Dock

It is recommended enabling the "Keep in Dock" option for the app so that the nice animation always catches your eye 👀.

<img width="418" alt="keep-in-dock" src="https://github.com/fujidaiti/live-app-icon/assets/68946713/cfe5ba4f-88b3-4834-a912-1475c53bc60a">

### Login Items

When the generated app launchs for the first time, it automatically adds itself to Login Items. This is because the animation of the app icon is only available while the app is running. You might notice that if you quit the app, the animation also stops.

<img width="448" alt="add-to-login-items" src="https://github.com/fujidaiti/live-app-icon/assets/68946713/e3ce73af-8284-421c-bb65-bfe8c0bc2efe">


### Allow notifications

In addition, the app will also require the permission to send notifications at the first launch. It will notify you in a notificaation if an error occurs while running the command.

<img width="448" alt="notification-request" src="https://github.com/fujidaiti/live-app-icon/assets/68946713/c49af839-4480-46df-852b-81d2c04c8572">



## Integrate with Shortcuts

Since the generated app can run arbitrary shell commands, it is easy to perform complex workflows created with [Shortcuts](https://support.apple.com/guide/shortcuts-mac/welcome/mac). Did you know that we can also run the shortcuts from the command line? For example, to run a shortcut named "My Shortcut", you could use the following:

```shell
shortcuts run "My Shortcut"
```

Then it is easy to integrate this shortcut with App Live Icon as follows:

```shell
laic --label "My Shortcut" --command "shortcuts run \"My Shortcut\"" --gif path/to/your/gif
```

Notice that you need to escape the doube quotes in `--command` part since the shortcut name contains a whitespace. You can read [Run shortcuts from the command line](https://support.apple.com/guide/shortcuts-mac/run-shortcuts-from-the-command-line-apd455c82f02/mac) for more information about `shortcuts` command.

## Uninstall

### Cargo

If you have installed laic with `cargo install laic`, just run `cargo uninstall laic`.

### Binaries

If you have manually installed a laic executable, you can remove it like `sudo rm /usr/local/bin/laic` , or move it to the Trash by hand.

### Generated apps

Just move it to the Trash. The default install location is `$HOME/Applications` unless you specify a specific directory at creation. You may also need to remove the app from Login Items and the Notifications section in the System Setting.

## Troubleshooting

If you try to run the laic you downloaded, you might see a warning dialog like the one in the image below. This is because the binary is not signed by the identified developer. In this case, you must manually allow the app to run according to this article: [Open Mac app from unidentified developer](https://support.apple.com/guide/mac-help/open-a-mac-app-from-an-unidentified-developer-mh40616/mac).

<img width="676" alt="warning-dialog" src="https://github.com/fujidaiti/live-app-icon/assets/68946713/1c67f1e5-02dd-4780-a6dd-af45b2ab7d90">

## Thanks

- [The original idea](https://twitter.com/neilsardesai/status/1362179114204073984?s=20) by [@neilsardesai](https://twitter.com/neilsardesai)
- [DSFDockTile](https://github.com/dagronf/DSFDockTile) by [@dagronf](https://github.com/dagronf) : A swift library that provides easy APIs for creating animated app icons.
- [Funk Bass Sticker](https://giphy.com/gifs/vulfpeck-vulf-joe-dart-L3iUyZxjNJ080YJnlt) by [@jsot](https://giphy.com/jsot) : The image in this package has been took from https://giphy.com.

[![joe-dart](https://media0.giphy.com/media/L3iUyZxjNJ080YJnlt/giphy.gif)](https://media.giphy.com/media/L3iUyZxjNJ080YJnlt/giphy.gif)

## Contributing

If you find any bugs or have suggestions for improvement, please create an issue or a pull request on the GitHub repository. Contributions are welcome and appreciated!

## TODO

### Animated icons
- [ ] Add option to notify the command outputs
- [ ] Allow more complex animations (e.g. [attention concepts](https://github.com/dagronf/DSFDockTile#attention-concepts))

### CLI tool

- [ ] Validate the arguments
- [ ] Refactor the code
- [ ] Improve the report messages
- [ ] Add tests
- [ ] Check the requirements before run

### Others

- [ ] Create a GUI icon generator
- [ ] Provide a way to create animated icons without Xcode
