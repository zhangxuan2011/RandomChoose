<script setup lang="ts">
import { invoke } from "@tauri-apps/api/core";
import { listen, UnlistenFn } from "@tauri-apps/api/event";
import { onMounted, ref, watch } from "vue";

interface EventPayload {
  value: number;
  remaining_length: number;
}

// The definition of the config.
interface ConfigData {
  essential: ConfigDataEssential,
  optional: ConfigDataOptional,
}

interface ConfigDataEssential {
  min_num: number,
  max_num: number,
}

interface ConfigDataOptional {
  wait_millis: number,
}

// End of config definition
// Begin of the basic func definition

let random_number = ref(0);
let is_listening = ref(false);
let is_invoked = ref(false);
let min_num = ref(1);
let max_num = ref(50);
let remaining_length = ref(1);
let unlisten_func: UnlistenFn | null = null;

// Init pool when min/max changed
watch([min_num, max_num], async () => {
  await invoke("init_random_pool", {
    min: min_num.value,
    max: max_num.value,
  });
  random_number.value = 0;
});

// Init pool when widget mounted
onMounted(async () => {
  // Start listening
  let unlisten_fn = await listen<ConfigData>("config", async (event) => {
    min_num.value = event.payload.essential.min_num;
    max_num.value = event.payload.essential.max_num;

    await invoke("init_random_pool", {
      min: min_num.value,
      max: max_num.value,
    });
  });

  // Invoke this function then
  await invoke("get_config")

  // Unlisten immediately
  unlisten_fn();
});

async function toggle_choose() {
  const btn = document.getElementById("choose_one");
  if (!btn) { return; }

  // To avoid multiple invoking.
  if (is_listening.value) {
    btn?.classList.remove("choosing");
    await invoke("stop_choose")
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
      const length: number = event.payload.remaining_length;
      remaining_length.value = length;
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
      <p style="font-size: 16px;">
        (此次抽选，还没抽到的还有{{ remaining_length - 1 }}人！)
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
    <div class="licence" style="font-size: 12px;">
      <p>
        Copyright (C) <b>zhangxuan2011</b> 2022-2026, All rights reserved. <br>
        Frontend design by <b>longlonger2022</b>
      </p>
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

/* Apply the font for all elements */
@font-face {
  font-family: Genshin;
  src: url("/font.ttf");
}
* {
  font-family: Genshin;
}
</style>
