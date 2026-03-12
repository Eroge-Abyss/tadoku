# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [1.7.5](https://github.com/Eroge-Abyss/tadoku/releases/tag/v1.7.5) - 2026-03-12

### Added

- add manual game entry outside VNDB
- add hiding nsfw images to backend
- make presence title in jp according to setting
- add selected categories commands
- add commands for jp option
- add script to add missing alt titles
- add time played today
- add debug logging env var
- *(linux)* add support for running scripts as game exe
- add notes to game details
- *(frontend)* add playtime type setting
- *(backend)* add setting to switch playtime mode
- basic exstatic support
- move first played to game launch instead of close
- add first played to game details
- *(backend)* add discord presence options
- add random button toggle to settings
- add sorting setting storage to backend
- add nsfw presence toggle commands
- add hiding presence on nsfw on backend
- add settings store
- add updates
- characters tab
- add basic ability to close games
- add image and data saving for game characters
- create vndb service and add get characters
- add categories to games
- add categories api
- improve durability of playtime saving
- add basic detection for nsfw images
- release 1.0.2
- add discord icon and fix image loading
- 1.0.0 release
- add sidebar icons backend
- improve error handling throughout backend
- add pinned games to backend
- use optionals in state for state reuse
- add playtime stats
- add game id to state
- add game opener and global state to backend
- add storage capabilites
- add basic command for game saving
- add description, error handling and typed responses
- add basic vndb request command

### Fixed

- add more default fields for game
- fixed the comments, chnaged the progress bar
- convert result to option in icon url
- additional 1min time count
- exstatic time ui updates
- missing ui update on close
- ui not updated on playtime change
- classic playtime 60x count bug
- default icon
- compile errors
- *(linux)* playtime recording using wine
- high cpu usage in exstatic mode
- *(backend)* issues in exstatic tracker
- wrong import in windows
- slow launch issues
- wrong default theme when starting app with no settings
- rendering bugs
- bas csp config
- wrong import in playtime service
- missing state update after presence config change
- updater dialog showing when theres no update
- broken image icons
- characters would not download if original name or image are missing
- UI and BE bugs
- ui and windows compilation bugs
- remove args if not needed
- support args in opener
- fix some games not opening and improve lnk support
- update lib name
- remove unstable cargo features
- improve initial window size
- add missing dep
- fix cargo.lock corruption
- update comments and remove extra clone

### Other

- remove duplicate edition
- add release plz
- use Fetchable type and improve setup fetching
- simplify save_image util and save_game image handling
- use game dto and fix ui bug
- whatever
- 1.7.5
- move unncessary logs to debug mode
- game struct and add reset stats
- rename discord method names
- settings struct
- make icon url optional
- 1.7.0
- 1.6.0
- add pagination to vndb api call
- 1.5.7
- 1.5.6
- 1.5.3
- 1.5.2
- classic playtime
- 1.5.1
- 1.5.0
- Merge branch 'main' into logging
- 1.4.2
- update to 1.4.1
- 1.4.0
- 1.3.0
- use ingame instead of menus mode
- services and discord nsfw setting
- add prelude refactor discord service and prepare for discord modes
- fix clippy warnings
- 1.2.4
- remove devtools in prod
- 1.2.2
- 1.2.0
- Merge branch 'main' into settings-page-latest
- Merge branch 'main' into settings-page-latest
- rename store.rs to games_store.rs
- add settings button to the sidebar, add global css vars to sidebar title
- Update tauri.conf.json
- Update tauri.conf.json
- Update Cargo.toml
- 1.1.3
- bump the version up
- 1.1.0
- draft commit (DO NOT OPEN)
- fix broken Cargo.lock
- Merge branch 'main' into feat-improve-detection
- Add ability to edit exe after saving game
- Fix several issues and improve search quality
- Fix windows bugs
- Fix bugs
- Edit tauri.conf.json
- Add Discord presence and add game functionality
- create services file
- improve code structure and quality
- update cargo.toml
- update cargo.lock
- update app name
- LGTM
- intial commit
