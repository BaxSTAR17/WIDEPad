<script setup lang="ts">
  import { onMounted } from "vue";

  //import { invoke } from "@tauri-apps/api/core";

  import { Theme, Dark, Light, Whitespace, Void, Star } from "./themes.ts";
  import { listen } from "@tauri-apps/api/event";
    import { View } from "./viewmode.ts";

  let main_theme: Theme = Dark;
  let main_view: View = {
    side_panel: true,
    line_numbers: false,
    file_details: false,
    font_selection: false,
    font_color: false,
    zoom: false
  }
//   let transparancy = false;

  function update_theme(theme: Theme) {
    document.documentElement.style.setProperty("--primary-color", theme.primary_color);
    document.documentElement.style.setProperty("--secondary-color", theme.secondary_color);
    document.documentElement.style.setProperty("--tertiary-color", theme.tertiary_color);
    document.documentElement.style.setProperty("--surface-color", theme.surface_color);
    document.documentElement.style.setProperty("--shadow-color", theme.shadow_color);
    document.documentElement.style.setProperty("--text-color", theme.text_color);
    document.documentElement.style.setProperty("--header-color", theme.header_color);
    if(theme.highlights) document.documentElement.style.setProperty("--highlights", theme.highlights);
  }

  function change_theme(theme: string) {
    switch(theme) {
        case "light":
            main_theme = Light
            break;
        case "dark":
            main_theme = Dark
            break;
        case "whitespace":
            main_theme = Whitespace
            break;
        case "void":
            main_theme = Void
            break;
        case "star":
            main_theme = Star
            break;
        default:
            break;
    }
    update_theme(main_theme);
  }

  function update_view(viewitem: string) {
    switch(viewitem) {
        case "side-panel":
            main_view.side_panel
                ? document.querySelector(".sidepanel")?.setAttribute("style", "visibility: hidden; display: none;")
                : document.querySelector(".sidepanel")?.setAttribute("style", "visibility: visible; display: flex;");
            main_view.side_panel = !main_view.side_panel;
            break;
        case "file-details":
            main_view.file_details
                ? document.querySelectorAll(".file-deets")?.forEach((el) => el.setAttribute("style", "visibility: hidden;"))
                : document.querySelectorAll(".file-deets")?.forEach((el) => el.setAttribute("style", "visibility: visible;"))
            main_view.file_details = !main_view.file_details;
            break;
        case "font-selection":
            main_view.font_selection
                ? document.querySelector(".file-font-family")?.setAttribute("style", "visibility: hidden;")
                : document.querySelector(".file-font-family")?.setAttribute("style", "visibility: visible;");
            main_view.font_selection = !main_view.font_selection;
            break;
        case "font-color":
            main_view.font_color
                ? document.querySelector(".file-font-color")?.setAttribute("style", "visibility: hidden;")
                : document.querySelector(".file-font-color")?.setAttribute("style", "visibility: visible;");
            main_view.font_color = !main_view.font_color;
            break;
        case "zoom":
            main_view.zoom
                ? document.querySelector(".file-zoom")?.setAttribute("style", "visibility: hidden;")
                : document.querySelector(".file-zoom")?.setAttribute("style", "visibility: visible;");
            main_view.zoom = !main_view.zoom;
            break;
        default:
            break;
    }
  }
  
//   function enable_transparency() {
//     if(!transparancy) {
//         document.querySelector(".canvas")?.setAttribute("style", "opacity: 0;");
//         document.querySelectorAll(".pane")?.forEach((pane) => pane?.setAttribute("style", "opacity: 0.5;"));
//         document.querySelector(".sidepanel")?.setAttribute("style", "opacity: 0.6");
//         document.querySelector(".footer")?.setAttribute("style", "opacity: 0.6");
//         document.querySelector("main.wrap")?.setAttribute("style", "opacity: 0");
//         document.documentElement.setAttribute("style", "opacity: 0")
//         document.getElementById("app")?.setAttribute("style", "opacity: 0")
//         document.getElementById("appBody")?.setAttribute("style", "opacity: 0")
//         transparancy = true
//     } else {
//         document.querySelector(".canvas")?.setAttribute("style", "opacity: 1;");
//         document.querySelectorAll(".pane")?.forEach((pane) => pane?.setAttribute("style", "opacity: 1;"));
//         document.querySelector(".sidepanel")?.setAttribute("style", "opacity: 1");
//         document.querySelector(".footer")?.setAttribute("style", "opacity: 1");
//         document.querySelector("main.wrap")?.setAttribute("style", "opacity: 1");
//         document.documentElement.setAttribute("style", "opacity: 1")
//         document.getElementById("app")?.setAttribute("style", "opacity: 1")
//         document.getElementById("appBody")?.setAttribute("style", "opacity: 1")
//         transparancy = false
//     }
//   }

  onMounted(async () => {
    update_theme(main_theme);

    listen("themes", (event) => { change_theme(event.payload as string) });
    listen("view", (event) => { update_view(event.payload as string) });
    // listen("themes:transparent", enable_transparency)
  });

</script>

<template>
  <main class="wrap">
    <section class="sidepanel">
        <div class="newfile sidepanel-large-button">+ New File...</div>
        <div class="openfile sidepanel-large-button">Open Existing File</div>
        <hr class="separator">
        <section class="file-list">
            <div class="file one"><span>🗎</span> text.txt 
                <div class="sidepanel-control-file">
                    <div class="lock small-button">🔒</div>
                    <div class="visible small-button">👁</div>
                    <div class="pin small-button">📌</div>
                </div>
            </div>
            <div class="file two"><span>🗎</span> main.c
                <div class="sidepanel-control-file">
                    <div class="lock small-button">🔒</div>
                    <div class="visible small-button">👁</div>
                    <div class="pin small-button">📌</div>
                </div>
            </div>
        </section>
    </section>
    <section class="canvas" id="canvas">
        <div class="pane pane-1">
            <div class="tab">
                <div class="tab-title"><span>🗎</span> text.txt</div>
                <div class="tab-control">
                    <div class="minimize small-button">__</div>
                    <div class="exit small-button">X</div>
                </div>
            </div>
            <div class="content">
                <div class="file-details">
                    <div class="file-deets file-directory">User > text.txt</div>
                    <div style="color: gray">|</div>
                    <div class="file-deets file-size">28 KB</div>
                    <div style="color: gray">|</div>
                    <div class="file-font-family">Font Family: </div>
                    <div style="color: gray">|</div>
                    <div class="file-font-color">Font Color: <input type="color"></div>
                    <div style="color: gray">|</div>
                    <div class="file-zoom" style="display: flex;">🔎<input type="range"></div>
                    <div style="color: gray">|</div>
                </div>
                <textarea name="pane-1-text" id="pane-1-text" class="textblock"></textarea>
            </div>
        </div>
        <div class="pane pane-2">
            <div class="tab">
                <div class="tab-title"><span>🗎</span> main.c</div>
                <div class="tab-control">
                    <div class="minimize small-button">__</div>
                    <div class="exit small-button">X</div>
                </div>
            </div>
            <div class="content">

            </div>
        </div>
        <div class="resizer pane-1 pane-2"></div>
    </section>
    <aside class="footer">WIDEPad Alpha v1.1</aside>
  </main>
</template>

<style scoped>
    :root {
        --primary-color: rgb(0,0,0);
        --secondary-color: rgb(0,0,0);
        --tertiary-color: rgb(0,0,0);
        --surface-color: rgb(0,0,0);
        --shadow-color: rgb(0,0,0);
        --text-color: rgb(0,0,0);
        --header-color: rgb(0,0,0);
        --highlights: rgb(0,0,0);
    }
  main.wrap {
      display: flex;
      width: 100%;
      height: 100dvh;
      box-sizing: border-box;
      justify-content: flex-start;
  }
  .footer {
      position: absolute;
      bottom: 0;
      width: 100%;
      text-align: end;
      background-color: var(--tertiary-color);
      height: 25px;
      color: var(--text-color);
      box-sizing: border-box;
      padding-right: 10px;
      font-weight: bold;
      font-size: 0.9rem;
      align-content: center;
  }
  .sidepanel {
      height: calc(100% - 20px);
      width: 250px;
      background-color: var(--primary-color);
      display: flex;
      box-sizing: border-box;
      padding: 10px;
      flex-direction: column;
      gap: 6px;
      border-right: 2px solid var(--header-color);
  }
  .sidepanel-large-button {
      width: 100%;
      height: 25px;
      border: 1px solid var(--header-color);
      text-align: center;
      color: var(--text-color);
      background-color: rgba(0,0,0,0);
      transition: all 0.1s linear;
  }
  .sidepanel-large-button:hover {
      box-shadow: 0px 0px 4px var(--shadow-color);
      cursor: pointer;
      backdrop-filter: blur(5px);
      background-color: rgb(100,100,100,0.5);
  }
  .sidepanel-large-button:active {
      box-shadow: 0px 0px 7px var(--shadow-color);
      cursor: pointer;
      backdrop-filter: blur(6px);
      background-color: rgb(100,100,100,0.9);
  }
  .sidepanel-control-file {
      display: flex;
      gap: 3px;
  }
  .sidepanel-control-file .small-button {
      padding: 0;
  }
  .separator {
      border: none;
      border-top: 2px double var(--header-color);
      align-items: flex-start;
      width: 100%;
      display: flex;
      justify-content: flex-start;
      position: relative;
      overflow: visible;
      box-sizing: border-box;
  }
  .separator::after {
      background-color: var(--primary-color);
      content: '2 FILES';
      color: var(--header-color);
      font-weight: bold;
      font-size: 1rem;
      position: absolute;
      top: -13px;
      box-sizing: border-box;
      padding-right: 5px;
  }
  .file-list {
      display: flex;
      flex-direction: column;
      width: 100%;
      gap: 2px;
      overflow-y:auto;
      box-sizing: border-box;
      overflow-x:hidden
  }
  .file {
      flex-grow: 1;
      resize: horizontal;
      height: 25px;
      text-align: start;
      box-sizing: border-box;
      padding-left: 5px;
      padding-bottom: 4px;
      color: var(--text-color);
      background-color: rgba(0,0,0,0);
      transition: all 0.1s linear;
      display: flex;
      justify-content: space-between;
  }
  .file:hover {
      background-color: rgba(100,100,100, 0.5);
      cursor: pointer;
  }
  .canvas {
      background-color: var(--secondary-color);
      width: 100%;
      height: calc(100% - 25px);
      display: grid;
      grid-template-columns: 50% 50%;
      /* flex-wrap: wrap; */
      overflow:hidden;
      position: relative;
  }
  .pane {
      width: 100%;
      height: 100%;
      background-color: var(--primary-color);
      display: flex;
      flex-direction: column;
      box-sizing: border-box;
      position: relative;
      cursor:default;
      border: 1px solid var(--secondary-color);
      border-top: none;
  }
  .pane .content {
      width: 100%;
      background-color: var(--primary-color);
      height: 100%;
      box-sizing: border-box;
      padding: 8px;
      padding-left: 20px;
      display: flex;
      flex-direction: column;
      gap: 10px;
      color: var(--surface-color);
  }
  .tab {
      background-color: var(--highlights);
      width: 100%;
      height: 25px;
      box-sizing: border-box;
      display: flex;
      justify-content: space-between;
      color: var(--text-color);
  }
  .tab-title {
      display: flex;
      justify-content: space-around;
      width: 70px;
  }
  .tab-control {
      display: flex;
  }
  .resizer {
      position: absolute;
      height: 100%;
      width: 10px;
      left: calc(50% - 5px);
      cursor: ew-resize;
      z-index: 1;
  }
  .small-button {
      font-size: 1rem;
      transition: all 0.1s linear;
      width: 20px;
      height: 100%;
      background-color: rgba(0, 0, 0, 0);
      transition: all 0.1s linear;
      text-align: center;
      padding-right: 10px;
      padding-left: 10px;
  } .small-button:hover {
      text-shadow: 0px 0px 6px var(--shadow-color);
      cursor: pointer;
      background-color: rgba(75, 75, 75, 0.5);
  } .small-button:active {
      text-shadow: 0px 0px 8px var(--shadow-color);
      cursor: pointer;
      background-color: rgba(75, 75, 75, 0.9);
  }
  .textblock {
      width: 100%;
      height: 100%;
      resize: none;
      background: transparent;
      color: var(--shadow-color);
      margin: 0;
      padding: 0;
      border: none;
      text-indent: 0;
      font-size: 1.1rem
  }
  .textblock:focus {
      border: none;
      box-shadow: none;
      outline: none !important
  }
  .file-details {
      display: flex;
      gap: 10px;
      color: var(--shadow-color);
      font-weight: bold;
      align-items: center;
      border-bottom: 2px solid var(--tertiary-color);
      width: 100%;
      height: fit-content;
      box-sizing: border-box;
      padding-bottom: 5px;
      flex-wrap: wrap;
      opacity: 0.5;
      font-size: 0.8rem;
  }
  .file-directory {
      font-style: italic;
  }
  .file-details * { visibility: hidden; }
</style>