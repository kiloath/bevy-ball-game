# Bevy Ball Game
此文章參考[KiloathDoc v1.0](https://bloodcity.netlify.app/rack_about/doc_spec.htm)製作。
# {1!} 過程
## {1.1!} 從原Episode-6產生新支線kiloath-6
* 指令
  ```
  git checkout -b kiloath-9 --no-track origin/Episode-9-Fixes
  ```
* 升到Bevy 0.11  
  Cargo.toml:
  ```
  bevy = "0.11"
  ```
* 更新套件
  ```
  cargo update
  ```
* 修改以下發生的升級問題
  * [in_set](https://bevy-cheatbook.github.io/programming/intro-code.html)
  * [坐標改了](https://bevyengine.org/learn/migration-guides/0.10-0.11/#consistent-screen-space-coordinates)
  * [add_system淘汰](https://bevyengine.org/learn/migration-guides/0.10-0.11/#schedule-first-the-new-and-improved-add-systems)
  * [Audio不相容](https://bevyengine.org/learn/migration-guides/0.10-0.11/#bevy-audio-ecs-based-api-redesign)
  * [Event不相容](https://bevyengine.org/learn/migration-guides/0.10-0.11/#require-derive-event-on-all-events)
  * player_movement要用FixedUpdate
  不然會出現按下時會超出畫面放開時才校正回邊界
* 執行
  ```
  cargo run
  ```
# 原文

This project is the companion code to my YouTube video series on learning the basics of the [Bevy Game Engine](https://bevyengine.org), version 0.10.

This project starts in [Learn Bevy 0.10 - EP2 - First Game Setup + Bundles and Resources - Bevy Tutorial](https://youtu.be/izhFutJiZgo)

## Playlist

[Learn Bevy Engine 0.10 Beginner Tutorial Series](https://youtube.com/playlist?list=PLVnntJRoP85JHGX7rGDu6LaF3fmDDbqyd)

# Rust

Please ensure you have the latest version of the [Rust Programming Language](https://www.rust-lang.org) installed.

# Cargo

Build the project with `cargo build`

Run the project with `cargo run`

# Assets 

We're going to be using assets from the [kenney.nl](kenney.nl) website today for our project.
- Kenney provides amazing assets free to use with a public domain license.
- If you use their assets often, I highly recommend becoming a [Patreon](https://www.patreon.com/bePatron?u=102131) or [donating](https://kenney.itch.io/kenney-donation).

We're going to be using the following assets:
- https://www.kenney.nl/assets/rolling-ball-assets
	- `rollingballassets_kenney/PNG/Default/ball_blue_large.png`
	- `rollingballassets_kenney/PNG/Default/ball_red_large.png`
	- `rollingballassets_kenney/PNG/Default/star.png`
- https://www.kenney.nl/assets/sci-fi-sounds
	- `sci-fi-sounds/Audio/explosionCrunch_000.ogg`
	- `sci-fi-sounds/Audio/laserLarge_000.ogg`
- https://www.kenney.nl/assets/interface-sounds
	- `kenney_interfacesounds/Audio/pluck_001.ogg`
	- `kenney_interfacesounds/Audio/pluck_002.ogg`
