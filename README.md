### This uses a template I've set up

### Warning!
#### This project automatically builds all artifacts here: `../nannou_shared_directory`, so if you blindly `cargo build` this sketch on your machine it will create this directory. To avoid this behaviour simply delete the `target-dir= "../nannou_shared_directory"` in `.cargo/config.toml`

### Converting images to a video
- While in the dir of all the saved frames, use this command
- start_number is the starting frame 
`ffmpeg -r 24 -start_number 4 -i %03d.png output.mp4`