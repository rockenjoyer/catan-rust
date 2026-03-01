# The Settlers of Catan - Rust Edition
### LMU WS25/26 Rust SEP: Multiplayer Game Development
A digital implementation of "The Settlers of Catan", built in Rust using the Bevy game engine, with experimental multiplayer support.

![Board](docs/images/board.png)
## Quickstart
1. git clone https://github.com/rockenjoyer/catan-rust.git
2. cd catan-rust
3. cargo run

Alternatively, run the compiled .exe file (if available).

## Gameplay features


## Menu features and overview
### Main Menu
The Main Menu serves as the central "navigation hub" of the game. It provides access to gameplay, multiplayer functionality, rules, settings and game exit.

![Main Menu](docs/images/main_menu.png)
#### Start Game
The "Start Game" button launches a local singleplayer session, where up to 4 players can technically play against each other on a single machine. The game transitions directly into the setup phase and initializes all required systems (board generation, assets, panels, audio etc.).

#### Multiplayer
Upon clicking on "Multiplayer", the multiplayer submenu opens.
From here, players can host or join a game (see below in the multiplayer section).

#### Rules
Players have the option to read the rules to stay up-to-date. The "Rules" button opens a purely informational rules overview, which is presented in structured sections. Players can read about:

  1. Setup
  2. Resources
  3. Turn Structure
  4. Robber
  5. Victory Points

#### Settings
In the bottom left corner, upon clicking on the "🔧" icon, the settings menu opens. It is also available ingame. Available options are:

  - Music and SFX volume slider
  - Window Mode: Windowed, Borderless Fullscreen & Fullscreen
  - Resolution Presets: 1280 x 720, 1920 x 1080 & 2560 x 1440

When accessed during gameplay, an additional option to "Return to Main Menu" is available.
Settings are applied immediately, there's no need to restart the game.

#### Quit Game
The "Quit Game" button closes the application immediately, which is also available on the endscreen. Of course, it is also possible to simply close the window.

### Endscreen
The Endscreen appears once a player reaches the victory condition of 10 Victory Points. It appears similar to the main menu, with changes to the available buttons. 

#### Return to Main Menu
A "Return to Main Menu" button is now available, which also resets the current game state.

#### Credits
By clicking on the "Credits" button, various project credits will be displayed:
  - Development Team
  - Bevy Game Engine & Rust Programming Language
  - Original Game Inspiration

#### Stats
The "Stats" button displays statistics of the previous gameplay. These contain:
  - Winner
  - Victory Points of each player
  - Settlement-, City-, Road- & Resource count of each player
  - Achievements: Longest Road & Largest Army

## Multiplayer
### Hosting a game
From the multiplayer menu, click "Host". This launches the lobby, where a "Join Code" is displayed.

Note: The planned dynamic join code functionality is not yet fully implemented. Currently, the "Join Code" is simply the local IP address of the host machine. Clients require this IP address to connect directly to the host.

Once at least one client is connected in the lobby, the host can start the game. The lobby includes a chat functionality for communication between players.

### Joining a game
In order to join a lobby, enter the "Join Code" provided by the host and then click "Join".

If the code is correct, you will join the host’s lobby. The host controls when the game starts, and all connected players will be prompted to begin simultaneously.

### Limitations
#### Current state
Multiplayer functionality is not fully operational and has several limitations.

#### Input handling
Connected players cannot interact directly with the game in multiplayer mode. Input handling for in-game actions (e.g., building roads, settlements, or rolling dice) is not implemented.

The only way to play in multiplayer is through the chat system. Players must manually type their moves and communicate them to others.

While this is not ideal, it is currently the only available method for multiplayer interaction.

![Chat](docs/images/chat.png)

#### Lack of features and safety nets
No graceful disconnect: If a player or host disconnects unexpectedly, the game may crash or behave unpredictably.

No host migration: If the host leaves, the game cannot automatically assign a new host. This will have no immediate effect on the clients, since the launched game runs locally, but chat communication will stop working.

Some state transitions are not handled correctly. Issues arise when: a host leaves the lobby, or a client disconnects and attempts to rejoin.

These limitations can lead to frustrating errors, program panics, or unstable behavior.

#### Unused or Incomplete Code
Due to shifting priorities and time constraints, some planned features were deprioritized or dropped in favor of core functionality. As a result, there may be unused code snippets, structs, or functions that remain in the codebase but are not fully implemented or integrated.

While every effort has been made to document the project thoroughly, some unresolved or unused code fragments may still exist. These remnants do not currently impact functionality or stability.

## Misc
### Cargo features
*Find the list and description in [cargo.toml](Cargo.toml)*


### Limitations

## Credits

### Game Assets
#### Sounds
- [Placing down](
https://freesound.org/people/Jaszunio15/sounds/421243/
)
- [Click](
https://pixabay.com/sound-effects/film-special-effects-computer-mouse-click-352734/
)
- [Dice](
https://freesound.org/people/Code_E/sounds/575176/
)
- [Win](
https://freesound.org/people/el_boss/sounds/677859/
)
- OST:
    - background_music0.ogg: https://www.youtube.com/watch?v=LVqyrKUia58
  - background_music1.ogg: self-made

#### Art
- [Catan Logo](
https://www.catan.de/catan-universe 
)
- Cards: Matt Mocarski
- Rest: self-made

## Disclaimer
The use of generative AI was limited to the [Mistral Ai Le Chat](https://chat.mistral.ai/chatmodel).

Generative AI was exclusively employed for:
- Error handling of non-descriptive panics or crashes
- Documentation searches for continuously evolving libraries that lacked comprehensive or up-to-date documentation

No generative AI was used for core logic, gameplay mechanics, or creative decision-making.