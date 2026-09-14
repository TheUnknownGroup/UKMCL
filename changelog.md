# v1
## What changed?
Added library downloading, client jar downloading.

Created a config file that's read from the launch function.

Added the ability to launch an instance.

Added asset downloading.

Added a versions instance that stores the client jar by instance directory so when the instance gets deleted so will the client jar but not every minecraft version with the same client jar will be deleted.

When a user launches one instance, another instance can be made and launched as well.

Made a download bar that shows up for assets, along with shrinking down the average time of downloading from 5 minutes (depending on internet speed) to around 10-20 seconds. The timer on the download bar isn't exactly accurate as it shows how long the file could take if the downloading process wasn't as quick as it is.

Created sub directories under the main directory: `ukmcl` for assets, jsons, versions, and libraries.

Under the instance directory, is the minecraft directory, this holds all of the folders and files that are made by minecraft for minecraft.

The download bar is a window that get's created so it can display the downloading, I would do the same for the libraries but it wouldn't make sense as it would be down around the same time as it takes to download the assets.

## Somethings buggy though...
If you create an instance that has the same minecraft version as a previously made one or something thats more recent with relatively the same assets, the download bar will try to appear but won't work, and will be held at a standstill. 

I don't know if it goes away after around 800s (or ms, no clue once again), but this will be something worth looking at.

## What's expected to come next?
Selection of different types of verions (i.e., Old Alpha's, Old Beta's, Snapshots, and General Releases). Will add microsoft authentication for minecraft accounts and playing.