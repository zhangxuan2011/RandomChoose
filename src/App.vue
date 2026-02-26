<script setup lang="ts">
import { invoke } from "@tauri-apps/api/core";
import { listen, UnlistenFn } from "@tauri-apps/api/event";
import { ref } from "vue";

interface EventPayload {
  value: number;
}

let random_number = ref(0);
let is_listening = ref(false);
let is_invoked = ref(false);
let min_num = ref(1);
let max_num = ref(50);
let unlisten_func: UnlistenFn | null = null;

async function toggle_choose() {
  const btn = document.getElementById("choose_one");
  if (!btn) { return; }

  // To avoid multiple invoking.
  if (is_listening.value) {
    btn?.classList.remove("choosing");
    await invoke("stop_choose");
    if (unlisten_func) {
      unlisten_func();
      unlisten_func = null;
    }

    // Reset the state.
    is_listening.value = false;
    is_invoked.value = false;
  } else {
    btn?.classList.add("choosing");
    if (!is_invoked.value) {
      await invoke("choose_number", {
        min: min_num.value,
        max: max_num.value,
      });
      is_invoked.value = true;
    }

    // Start listening...
    unlisten_func = await listen<EventPayload>("random_number", (event) => {
      const payload: number = event.payload.value;
      random_number.value = payload;
    });

    is_listening.value = true;
  }
}
</script>

<template>
  <nav id="navbar" style="background-color: rgba(255, 255, 255, 0.3)">
    <div class="nav_container">
      <a class="logo"><span>随机抽选</span></a>
      <ul>
        <li>
          <a href="settings.html">设置</a>
          <a
            href="https://github.com/zhangxuan2011/RandomChoose"
            target="_blank"
            >GitHub仓库</a
          >
        </li>
      </ul>
    </div>
  </nav>
  <main class="container" style="margin-top: 95px">
    <h1 id="title" style="font-size: 36px">欢迎使用随机抽选</h1>
    <div class="image">
      <img src="/icon.png" id="logo" />
    </div>
    <div class="mainmsg" style="font-size: 32px">
      <p>抽中了：{{ random_number }}号</p>
      <p style="font-size: 16px">
        (当前抽选范围为{{ min_num }}~{{ max_num }}号)
      </p>
    </div>
    <div class="button">
      <button @click="toggle_choose" id="choose_one" class="btn">
        {{ is_listening ? "点此停止抽选" : "点此开始抽选" }}
      </button>
    </div>
    <div class="reserved" style="height: 10px">
      <!-- For reserved only -->
    </div>
  </main>
</template>

<style lang="css" src="src/global-style.css"></style>
<style scoped>
.choosing {
  box-shadow: #00aeec 0 5px 25px;
  background: none;
  color: #00aeec;
  border: #00aeec solid 3px;
  padding-left: 40px;
  padding-right: 40px;
}

#logo {
  height: 70px;
  border-radius: 10px;
  box-shadow: #00aeec 0 5px 120px;
  transition: all 0.5s ease;
  overflow: visible !important;
}

#logo:hover {
  transform: translateY(-5px);
  box-shadow: #00aeec 0 10px 30px;
}

/* Apply the font for all elements */
@font-face {
  font-family: Genshin;
  src: url("/font.ttf");
}
* {
  font-family: Genshin;
}
</style>
