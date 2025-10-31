<script setup lang="ts"></script>

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
    <div class="settings" id="basic-settings">
      <h2 class="title">基本设置项</h2>
      <div class="option">
        <p class="prompt">最小抽选取值：</p>
        <input class="global-input" type="number" />
      </div>
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
  margin-left: 5px;
}

.prompt {
  font-size: larger;
}

.global-input {
  width: 300px; /* 设置宽度 */
  height: 20px; /* 设置高度 */
  padding: 10px; /* 内边距 */
  border: 1px solid #ccc; /* 边框样式 */
  border-radius: 5px; /* 圆角 */
}
</style>
