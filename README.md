This is a basic utuility that deletes all folders in the directory its run from who's contents are less than 100MB
It does take an additional argument should you choose to set that size to something other than 100MB.
This does very little checking of command line arguments and exits non-gracefully when it gets confused. 
(aka: i did not implement useful error messages, i may in the future.)