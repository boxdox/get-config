# get-configs
pull your dot configs from a version controlled gists.

### why?
i was tired of copying my javascript configurations each time i started a new project.
i wanted a way to selectively pull some of my configs, from a interactive cli.
i created a npm package `@boxdox/config` but soon it became difficult to update and work with, also, calling it wil `npx` became a bit slow.
so this project was born. you can pass your github gist id and optionally a token (if you run this multiple times), and this package will show you a beautiful ui with all the files listed. then you can select your configs and the package will write them to your current directory.

### license
[mit 2022](https://boxdox.mit-license.org/)