# `@sutorio/arcana_b/system_evolution`

## Overview

## API Documentation links

Just putting these in one place (**NOTE:** these are the public docs, not dev branches):
- [Bevy]()
- [Bevy Rapier 3D](https://docs.rs/bevy_rapier3d/latest/bevy_rapier3d/)
- [Leafwing input manager](https://docs.rs/leafwing-input-manager/0.9.2/leafwing_input_manager/)

## Forks

This currently uses the v0.11 master of Bevy. Because of this, supporting crates may need to be forked
if someone hasn't already done the work.

[This merged PR](https://github.com/bevyengine/bevy/pull/8079) is very important, as it fixes `add_systems`.

- [bevy_rapier 0.11 update](https://github.com/flmng0/bevy_rapier/tree/master)
- [leafwing-input-manager 0.11 update](https://github.com/sutorio/leafwing-input-manager/tree/main)



TODO

## Reference

### Games ref

- [MGS 1 playthrough (YouTube, no commentary)](https://youtu.be/R4I0l4o65JQ)

### General

- [Everything I know (Nikita Voloboev's digital garden -- *very* useful)](https://wiki.nikiv.dev/)
- [MotionCanvas - "visualise ideas programatically" - useful for docs?](https://motioncanvas.io)
- [The book of secret knowledge](https://github.com/trimstray/the-book-of-secret-knowledge)
  > This repository is a collection of various materials and tools that I use every day in my work. 
  > It contains a lot of useful information gathered in one piece. It is an invaluable source of knowledge for me that I often look back on.

### General Rust

- [Awesome Rust](https://github.com/rust-unofficial/awesome-rust)
- [Mini-redis: Tokio's heavily commented idiomatic example of a Redis client for learning purposes](https://github.com/tokio-rs/mini-redis)
- [Everything I know: Bevy](https://wiki.nikiv.dev/games/gamedev/game-engines/bevy)
- [Rust adventure by Chris Biscardi, loads of useful stuff](https://github.com/rust-adventure)
- [So you want to live-reload Rust (fantastic, as is the rest of the blog)](https://fasterthanli.me/articles/so-you-want-to-live-reload-rust)
- [Aquascope - a tool to visualise compile- and run-time behaviour of rust programs](https://cognitive-engineering-lab.github.io/aquascope/)

### Version control

Git is great for text files but assets are generally not that (models, images etc). Perforce is normally used.

- [Git LFS - worth it for quick'n'dirty workflow?](https://git-lfs.com/)
- [Awesome Github actions](https://github.com/sdras/awesome-actions#readme)

### Controllers - movement, physics etc.

There is a dearth of "how to make a character move around in 3D" examples. Lots of tower defence stuff! But that's not super duper useful.

- [Procedural animation, *really* good](https://www.gdcvault.com/play/1020583/Animation-Bootcamp-An-Indie-Approach)

### Bevy-specific

- [Think this might be useful...](https://github.com/Orbsa/ROTS/blob/master/client/src/player.rs)
- [Foxtrot's move logic](https://github.com/janhohenheim/foxtrot/blob/main/src/movement/general_movement.rs)
- [Bevy ECSS for UI stylesheets](https://github.com/afonsolage/bevy_ecss/) - TODO: fork and update

- [A player controller (using bevy 0.4, no physics engine)](https://gitlab.com/-/snippets/2056102)

### Maths/Physics (non-Bevy-specific)

- [Has anyone has created a third person controller and follow camera? (nope, not really)](https://discourse.threejs.org/t/r3f-rapier-third-person-controller-and-follow-camera/48832)
- [React-three-rapier](https://github.com/pmndrs/react-three-rapier)
- [R3f rapier demo](https://react-three-rapier.pmnd.rs/)
- [Rapier/R3F minecraft demo](https://codesandbox.io/s/minecraft-vkgi6)
- [ThreeJS & Rapier3D - Character terrain movement (YouTube)](https://youtu.be/voGmsOuB3Rk)
- [(Youtube, precursor to above, 3rd person character controller in ThreeJS)](https://youtu.be/C3s0UHpwlf8)

- [Godot 3d platformer KinematicBody player script](https://github.com/godotengine/godot-demo-projects/blob/3.5-9e68af3/3d/platformer/player/player.gd)
- [Godot SpringArm docs (useful for 3rd person cameras)](https://docs.godotengine.org/uk/stable/classes/class_springarm.html)
- [Godot KinematicBody docs](https://docs.godotengine.org/uk/stable/classes/class_kinematicbody.html)

- [Math for game developers series on YouTube - ep1: Character movement](https://youtu.be/sKCF8A3XGxQ)
- [Math for game programmers on YouTube - GDC talk on interaction with 3d geometry](https://youtu.be/GpsKrAipXm8)
- [Steering behaviours for autonomous characters](http://www.red3d.com/cwr/steer/)
- [GDC talk on building a better jump (YouTube)](https://youtu.be/hG9SzQxaCm8)
- [Making a physics-based character controller (YouTube, very very good!)](https://youtu.be/qdskE8PJy6Q)
- [Intant "game feel" - secrets of springs (YouTube)](https://youtu.be/bFOAipGJGA0)
- [Adding acceleration to player movement (YouTube)](https://youtu.be/ynHA3hsFWoE)
- [Math for tranforming 3D geometry](https://medium.com/@Jacob_Bell/math-for-transforming-3d-geometry-2817d12dd4a9)


### Other unsorted game programming/graphics stuff

- [Game devloper roadmap](https://github.com/utilForever/game-developer-roadmap)
- [Code incomplete article index](https://codeincomplete.com/articles/)
- [How to make good small games](http://farawaytimes.blogspot.com/2023/02/how-to-make-good-small-games.html)


- [Manually drawing PBR materials (good!)](https://www.kenney.nl/learn/manually-drawing-pbr-materials)
- [For above, "Normal Map Online", simple normal map creator](https://cpetry.github.io/NormalMap-Online/)

- [1D procedural terrain generation](https://arpitbhayani.me/blogs/1d-terrain)
- [Article on pseudo 3D](http://www.extentofthejam.com/pseudo/)
- [How I made a 3D game in 2Kb of JS](https://frankforce.com/how-i-made-a-3d-game-in-only-2k-of-javascript/)

- [Page from Level Design Book (full of good stuff!) on Trenchbroom, the Quake level editor](https://book.leveldesignbook.com/appendix/tools/trenchbroom)
- [The rational design handbook, intro to rational level design](https://www.gamedeveloper.com/design/the-rational-design-handbook-an-intro-to-rld)
- [Blender LevelBuddy plugin](https://matt-lucas.itch.io/level-buddy)
- [Voronoi tiles terrain, lovely](https://www.reddit.com/r/godot/comments/12y6lyv/check_out_the_terrain_in_my_town_defense/)

- [Blender Donut Notes](https://jeremypedersen.com/posts/2022-02-03-blender-01/)

- [Spline, simple and *highly* polished 3d modelling tool](https://app.spline.design/home)
- [Polypizza - clone of the defunct Google 3D models archive](https://poly.pizza/)
- [FlowBetween 2d animation tool](https://github.com/Logicalshift/flowbetween)
- [Bestsnip 2d animation tool (very simple)](https://bestsnip.com/)

### UI/UX and graphic design

- [Inkscape cheat sheet for keybindings](https://defkey.com/inkscape-shortcuts?orientation=landscape&filter=false&cellAlternateColor=%23d6ffef&showPageNumber=true&showPageNumber=false&pdf=True)
- [7GUIs: a GUI programming benchmark (TODO: implement these!)](https://eugenkiss.github.io/7guis/)
- [rx - a Vim-like pixel editor](https://github.com/cloudhead/rx)
- [Fontshare (ITF's free web font service -- amazing)](https://www.fontshare.com/)
- [Ty Finck's fonts](https://tyfromtheinternet.com/fonts/)
- [Variable fonts resource](https://v-fonts.com/tags/)
- [Awesome colour resources](https://github.com/Siddharth11/Colorful#readme)
- [Typedrawers forum (type design critique)](https://typedrawers.com/)
- [2000 letters: which is which](http://www.sanskritweb.net/letters/index.html)
  This is an insane resource.

  > Even the "designer" of a "new" font is usually not able to identify his "new" letters among thousands of 
  > other letters ("Which is which?"), because most "new" fonts do not contain novel and individual letters 
  > that never existed before. This is true of simple forms (e.g. pi_period.pdf), but is also true of complex 
  > forms (e.g. lig_et.pdf), provided you compare similar forms (e.g. 106 and 255 in the above picture).
  > Below we offer 94 huge PDF files, each of which including more than 2000 designs of one letter each 
  > (lowercase or uppercase or umlaut or ligature or number or punctuation or other pi character) typeset 
  > in more than 2000 well-known typefaces. For example, the above picture shows only 15 et-ligatures, but
  > the huge file lig_et.pdf contains more than 2000 et-ligatures ("ampersands") drawn from more than 2000 fonts.
- [Beautiful web type](https://beautifulwebtype.com)
- [Finessing \[SVG's\] FeColorMatrix article - nice look at per-channel colour manipulation](http://alistapart.com/article/finessing-fecolormatrix/)
- [Cubehelix colour space](https://ifweassume.blogspot.com/2013/05/cubehelix-or-how-i-learned-to-love.html)
- [The magic of CSS](https://adamschwartz.co/magic-of-css/)
