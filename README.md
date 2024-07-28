# Meditationtimer

This is an installable Progressive Web App (PWA) written in Rust. It's a simple app which allows you to set multiple intervalls of timers. The timer keeps running even when the screen is locked or the app is running in the background. See the deployed app at [https://meditime.netlify.app/](https://meditime.netlify.app/). Tested on Android with Firefox and Chrome.

It's been built using [MoonZoon](https://github.com/MoonZoon/MoonZoon). To get started yourself, you have to install the `mzoon` executable:

```
cargo install mzoon --git https://github.com/MoonZoon/MoonZoon --locked
```

This installs the latest available version. If you encounter problems you might want to install the exact same version that is specified in the `Cargo.lock` file for the package `zoon`.

Since this is a fontend-only single page application, we should then follow the instructions in the [MoonZoon README for "Frontend-only"](https://github.com/MoonZoon/MoonZoon/#frontend-only). It's basically a:

```
mzoon build --release --frontend-dist netlify
```

This leaves you with a folder `frontend_dist` which can be deployed statically.
