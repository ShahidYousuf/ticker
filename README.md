# ticker
A scalable and performant near real time directory watcher for file change notifications and logging.

# How to run
``` ticker </path/to/directory/to/watch> <frequency>```
``` Here frequency is a number from 1 (default) to 9.```

# Notes:
#### ticker is able to watch for changes to thousands of files in near real time.
#### If the files are large in number, say more than 5000, frequency needs to be increased to say 6 to reduce the CPU consumption.
#### By default ticker will watch only non-hidden files, currently if you need to watch for hidden files, change frequency to 9.
#### The worst case default delay for file change logging/notification is under 1 sec. Increasing the frequency number increases the time also, needed in order not to DoS the CPU.
#### Improvements are continuosly taking place.
