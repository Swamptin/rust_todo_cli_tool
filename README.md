# A simple commandline TODO app.

This is a copy of the rust todo from
https://www.freecodecamp.org/news/how-to-build-a-to-do-app-with-rust/  
  
The app, as was detailed in that blog post is what I committed as the initial
commit. I will periodically add features to this when time suits. The blog did
not include a print, or show feature. I have a Minimum Viable Product of 'show'
present.  
  
To run this app ensure you have rust installed.
Clone the repo and cd into the top folder.
Run in the following way:
cargo run -- {action} "{value}"  
For example `cargo run -- add "make coffee"`
Possible action are add, complete, and show.
  
To execute 'show' you must run `cargo run show all` The 'all' isn't strictly
checked, but without it it errors due to only having one input. I will be
working to fix that as I can.  
