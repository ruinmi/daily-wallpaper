<script setup lang="ts">
import {Dismiss24Regular, Subtract24Regular, Maximize24Regular} from '@vicons/fluent'
import {getCurrentWindow} from '@tauri-apps/api/window'

const currentWindow = getCurrentWindow()

</script>

<template>
  <div data-tauri-drag-region class="title-bar">
    <div class="info-area">
      <img src="/src/assets/icon.png" alt="" class="logo"/>
      <span class="title">Daily Wallpaper</span>
    </div>
    <div class="btn-area">
      <n-icon class="title-bar-button" @click="() => currentWindow.minimize()">
        <Subtract24Regular/>
      </n-icon>
      <n-icon class="title-bar-button" id="title-bar-maximize">
        <Maximize24Regular/>
      </n-icon>
      <n-icon class="title-bar-button" id="title-bar-close" @click="() => currentWindow.hide()">
        <Dismiss24Regular/>
      </n-icon>
    </div>
  </div>
</template>

<style scoped>
.title-bar {
  height: 30px;
  background: #000000;
  user-select: none;
  position: relative;
  top: 0;
  left: 0;
  bottom: 0;
  right: 0;
}

.info-area {
  user-select: none;
  -webkit-user-select: none;
  pointer-events: none;
  display: flex;
  .logo {
    display: inline-flex;
    justify-content: center;
    align-items: center;
    height: 30px;
    width: 30px;
    padding: 5px;
    box-sizing: border-box;
  }
  .title {
    display: inline-flex;
    justify-content: center;
    align-items: center;
    font-size: 13px;
  }
}

.btn-area {
  user-select: none;
  position: absolute;
  right: 0;
  top: 0;

  .title-bar-button {
    display: inline-flex;
    justify-content: center;
    align-items: center;
    width: 45px;
    height: 30px;
    user-select: none;
    -webkit-user-select: none;
  }

  .title-bar-button:hover {
    background: #1A1A1A;
  }

  .title-bar-button:active {
    background: #353535;
  }

  #title-bar-maximize {
    pointer-events: none;
  }

  #title-bar-close:hover {
    background: #e61123;
  }

  #title-bar-close:active {
    background: #830b17;
  }
}

</style>