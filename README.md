Run the project:

```bash
# Release mode must be enabled for the camera to decode
# MJPEG compressed frames in a reasonable amount of time 
cargo build --release

# Link the systemd service so that the robot runs on startup
sudo systemctl enable $PWD/robot.service
sudo systemctl start robot
```
