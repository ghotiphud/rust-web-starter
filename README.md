# Rust Web Starter
Pre-configured starter template for web apps using React ([create-react-app w/ Typescript](https://github.com/Microsoft/TypeScript-React-Starter)) on the frontend and Rust ([Rocket](https://rocket.rs), [Diesel](http://diesel.rs), & [PostgreSQL](https://www.postgresql.org/)) on the backend.

# Quick Start

Install [Docker](https://docs.docker.com/engine/installation/) & [Docker-Compose](https://docs.docker.com/compose/install/)

Then:

```bash 
# clone template into new directory
git clone https://github.com/ghotiphud/rust-web-starter.git my-new-project
cd my-new-project

# start it up
docker-compose up
```

Open [http://localhost:3000](http://localhost:3000) (or `{docker ip}:3000` on windows) to view it in the browser.

NOTE: If you're on windows you may get an error about bash/r, this has to do with line endings which can be fixed by `git config --global core.autocrlf false` then cloning again which will clone with unix line endings... Not sure if this is the best answer.

The page will reload if you make edits in the `/web` folder.

The api server will restart if you make edits in the `/api_server` folder.

The api layer can be viewed at `localhost:3000/api` due to the webpack-dev-server proxying setup. (which also means that you can directly use `fetch("/api/{whatever}")` from the React side and not worry about cross-origin request issues.

# Making it your own

After you've tried it out a bit, feel free to steal it! Just remember where it came from and contribute anything really cool back. ;-)

```bash
# remove the git repo (could also change remote url if you wanted to keep the history)
rm -rf .git
# init new repo
git init
git add .
git commit -m "initial commit of template from https://github.com/ghotiphud/rust-web-starter.git"
```

# Workflow

I like to use a couple terminals (tabbed) one to run the containers and watch stdout, the other to run any other commands.

### Terminal #1
1. `docker-compose up` to start
2. `Ctrl+C` to stop
3. Sometimes `docker-compose down` to dispose of containers

### Terminal #2
Examples of useful commands.

* `docker-compose exec api_server bash`
    * `cargo upgrade`
    * `diesel setup`
    * `diesel migration generate {name}`
    * `diesel migration run`
    * `diesel migration redo`
    * See [Rocket](https://rocket.rs) & [Diesel](http://diesel.rs) docs for more.
* `docker-compose exec web bash`
    * `yarn outdated`
    * `yarn upgrade`
    * See [create-react-app](https://github.com/facebookincubator/create-react-app) docs for more.

# Examples:

A small example of Rocket & Diesel usage can be found in `/api_server/src/posts.rs` on [this branch](https://github.com/ghotiphud/rust-web-starter/tree/diesel_blog).

Calling the api from React can be seen in `/web/src/App.tsx`

# Troubleshooting

If Rocket ever fails to build with an error concerning nightly version date, this can be fixed by rebuilding the docker container with the latest nightly `docker-compose build --no-cache api_server`.
