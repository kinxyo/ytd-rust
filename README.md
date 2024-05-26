# Youtube Downloader in Rust

    the videos are always downloaded on the client side,
    this is deduced from the fact that once internet goes off;
    you can still see the video; I mean the whole `buffer` reeks of client-side.

    so there must be an API they serves the static content (i.e, the video) the downloads it on client.
    instead of youtube making a call, I can make a call to that API
            **OR**
     maybe they have their serves secured enough that no random person a call
     (which is obviously the case), and so I will intercept it after its download on the client-side?
     maybe use a browser-automation tool? NO, that will be disgusting. I'll see, but sounds a fun project.

---

Just for funn; I always wanted to do this.
