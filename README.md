# RandomChoose

It's been a long time not update this project, because i am too busy (especially i am JH3 student).

Now this project is updated. it uses the new structure, and its GUI is more pretty than before.

So, please allow me to introduce this project's changes, compare to the previous version:

## Changed

 - Uses [Rust](https://www.rust-lang.org/) to do this project **instead of** Python;
 - Uses [tauri](https://tauri.app/) to build this project **instead of** PyQt6;
 - Uses [Vue](https://vuejs.org/) as the frontend framework, and [Rust](https://www.rust-lang.org/) as the backend framework.

## New Features

 - Uses `stroll-choosing` instead of `one-click-choosing`;
 - GUI refactor, now it's more pretty than before;
 - HTML-like, now you can use HTML tags to format your text;
 - Allow to refresh the page to reset the current page.

## Build

Before building this project, you should install these components on your computer:
 - Rust (stable is enough);
 - Node.js (v25.6.1, with `npm`);

After doing these, you can follow these steps to build this project:

 1. Check your components version:
 ```bash
 rustc --version
 node --version
 npm --version
 ```

 2. Clone this project to your local computer:

 ```bash
 git clone https://github.com/zhangxuan2011/RandomChoose.git
 cd RandomChoose
 git switch withRust
 ```

 3. Install the dependencies;

 run this: `npm install`

 4. Build the project;

 run this: `npm run tauri build`

If you want to run this project without building, you can run this: `npm run tauri dev`


