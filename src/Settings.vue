<script setup lang="ts">
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import { ask, message } from "@tauri-apps/plugin-dialog";
import { onMounted, ref } from "vue";

// The definition of the config.
interface ConfigData {
  essential: ConfigDataEssential;
  optional: ConfigDataOptional;
}

interface ConfigDataEssential {
  min_num: number;
  max_num: number;
}

interface ConfigDataOptional {
  wait_millis: number;
}

// End of config definition
// The definition of some essential arguments
let min_num = ref(0);
let max_num = ref(100);
let wait_millis = ref(5);
let config_data = ref<ConfigData | undefined>();

// Automatically get the config
onMounted(async () => {
  // Start listening
  let unlisten_fn = await listen<ConfigData>("config", (event) => {
    min_num.value = event.payload.essential.min_num;
    max_num.value = event.payload.essential.max_num;
    wait_millis.value = event.payload.optional.wait_millis;
  });

  // Invoke this function then
  await invoke("get_config");

  // Unlisten immediately
  unlisten_fn();
});

// Call this when the "Apply" button clicked
async function apply_changes() {
  const answer = await ask("确认应用更改吗？\n这将覆盖默认配置！！！", {
    title: "确认提示",
    kind: "warning",
  });
  if (!answer) {
    return;
  }

  // If wait_millis is empty, then use 5 as the default value.
  if (typeof wait_millis.value !== "number") {
    wait_millis.value = 5; // Default = 5
  }

  // Check the current value's type
  if (typeof min_num.value !== "number" || typeof max_num.value !== "number") {
    await message("请输入正确的数值！", { title: "错误", kind: "error" });
    return;
  }

  // Check the option's number is valid
  // Check min/max is overflow
  if (
    min_num.value < -2147483648 ||
    min_num.value > 2147483647 ||
    max_num.value < -2147483648 ||
    max_num.value > 2147483647
  ) {
    await message("无效的最小/最大范围取值（不能超过i32范围大小）！", {
      title: "错误",
      kind: "error",
    });
    return;
  }

  // Check the wait_millis
  if (wait_millis.value < 0 || wait_millis.value > 1000) {
    await message("数据更新间隔取值范围只能在0~1000！", {
      title: "错误",
      kind: "error",
    });
    return;
  }

  // Make it as the interface
  config_data.value = {
    essential: {
      min_num: min_num.value,
      max_num: max_num.value,
    },
    optional: {
      wait_millis: wait_millis.value,
    },
  };

  // Invoke the func to write to config file
  await invoke("write_config", { config: config_data.value }).then((msg) => {
    console.log(msg);
  });
}
</script>

<template>
  <nav id="navbar" style="background-color: rgba(255, 255, 255, 0.3)">
    <div class="nav_container">
      <a class="logo"><span>随机抽选</span></a>
      <ul>
        <li>
          <a href="index.html">主页</a>
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
    <h1 class="title" style="font-size: 36px">随机抽选设置</h1>
    <div class="image">
      <img src="/icon.png" id="logo" />
    </div>
    <div class="settings" id="basic-settings">
      <h2 class="title">基本设置项</h2>
      <div class="option">
        <p class="prompt">最小抽选取值：</p>
        <input
          v-model="min_num"
          class="global-input"
          type="number"
          min="-2147483648"
        />
      </div>
      <div class="option" style="margin-top: 5px">
        <p class="prompt">最大抽选取值：</p>
        <input
          v-model="max_num"
          type="number"
          class="global-input"
          max="2147483647"
        />
      </div>
      <h2 class="titie">可选设置项</h2>
    </div>
    <div class="settings" id="optional-settings">
      <div class="option">
        <p class="prompt">数据更新间隔：</p>
        <input
          v-model="wait_millis"
          type="number"
          class="global-input"
          placeholder="5"
          min="0"
          max="1000"
        />
        <p>ms</p>
      </div>
    </div>
    <div class="button">
      <button @click="apply_changes" class="btn">应用更改</button>
    </div>
    <div class="licence" style="font-size: 12px">
      <p>
        Copyright (C) <b>zhangxuan2011</b> 2022-2026, All rights reserved.
        <br />
        Frontend design by <b>longlonger2022</b>
      </p>
    </div>
  </main>
</template>

<style lang="css" scoped src="src/global-style.css"></style>

<style lang="css" scoped>
/* Apply the font for all elements */
@font-face {
  font-family: Genshin;
  src: url("/font.ttf");
}
* {
  font-family: Genshin;
}

.option {
  display: flex;
  flex-direction: row;
  align-items: center;
  gap: 10px;
  margin-left: 5px;
}

.prompt {
  font-size: larger;
  width: 140px;
  margin: 0;
}

.global-input {
  width: 150px; /* 设置宽度 */
  height: 20px; /* 设置高度 */
  padding: 10px; /* 内边距 */
  border: 1px solid #ccc; /* 边框样式 */
  border-radius: 5px; /* 圆角 */
}
</style>
