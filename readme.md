# Fancy Toilet

# Roadblocks

- Pi Zero is too slow for `rodio`
- Making own lib would be _very_ time consuming
- Pi 3: `rodio` audio is sped up

## TODO

 - App state derived from sonar inputs (close enough?)
 - Play audio! (its blocking the thread..)
 - Web Interface for configuring?
 - A nice enclosure for it all
 - Off switch?
 - Equalizer?

# Wiring

- HC-SR04
  - Vcc to 5V
  - GND t0 GND
  - Trig to GPIO4
  - Echo to GND with 330 & 470 resistors
  - Echo to GPIO24 between 330 & 470 resistors

- MAX98357
  - Vcc to 5V
  - Gnd to GND
  - DIN to GPIO21
  - LRC to GPIO19
  - BCLK to GPIO18

  # If all is bleak

  - Pops fix: `curl -sS https://raw.githubusercontent.com/adafruit/Raspberry-Pi-Installer-Scripts/master/i2samp.sh | bash`
  - rodio dep: `apt-get install libasound2-dev`
  - eq: `sudo apt-get install -y libasound2-plugin-equal`
  - soloud cmake: `sudo apt-get install cmake`
