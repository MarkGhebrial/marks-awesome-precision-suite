# MAPS

MAPS aims to be a tool for quickly, precisely, and repeatably quantifying the accuracy of nerf blasters.

# How it works

Here's the general idea:
1. Tape a rectangular target of white paper to a wall.
2. Hang a layer of carbon (not carbonless) copy paper over the target. You may need to put another layer of (normal) paper over the carbon paper in order to prevent it from tearing.
3. Shoot darts at the target. The darts will leave marks on the paper target when they hit the layer of carbon paper. Here's what that looks like: ![](images/testtarget15.jpg)
4. Take a good picture of the target (like the one above).
5. Run that picture through MAPS. MAPS will compute the location of the dots on the target and do some math to quantify the accuracy of the blaster.

# Build

Initial setup:
```bash
meson setup builddir
```

Compilation:
```bash
cd builddir
meson compile
src/MAPS <path_to_image> # run the program
```