<script setup lang="ts">
import {darkTheme} from 'naive-ui'
import Dashboard from "./components/Dashboard.vue";
import Showcase from "./components/Showcase.vue";
import WallpaperCriteria from "./components/WallpaperCriteria.vue";
import Rest from "./components/Rest.vue";
import {useMainStore} from "./stores/useMainStore.ts";
import TitleBar from "./components/TitleBar.vue";
import { themeOverrides } from './modules/theme-overrides';
import {startWallpaperChanger, stopWallpaperChanger} from "./modules/wallpaper-changer";
import {useCriteriaStore} from "./stores/useCriteriaStore.ts";

const mainStore = useMainStore()
const criteriaStore = useCriteriaStore()

onBeforeUnmount(() => {
  stopWallpaperChanger()
})

onMounted(async () => {
  if (!import.meta.env.DEV) {
    document.oncontextmenu = (event) => {
      event.preventDefault()
    }
  }
  await mainStore.init()
  await criteriaStore.init()
  startWallpaperChanger()
})
</script>

<template>
  <n-config-provider :theme="darkTheme" :theme-overrides="themeOverrides">
    <n-global-style/>
    <div id="app">
      <TitleBar/>
      <div id="container">
        <n-tabs tab-style="user-select: none" :animated="false" justify-content="space-evenly">
          <n-tab-pane name="Home" tab="主页">
            <Showcase/>
            <Dashboard/>
          </n-tab-pane>

          <n-tab-pane name="wallpaper form" tab="壁纸设置">
            <n-message-provider placement="bottom-right">
              <WallpaperCriteria/>
            </n-message-provider>
          </n-tab-pane>
          
          <n-tab-pane name="standup" tab="休息">
            <Rest/>
          </n-tab-pane>
        </n-tabs>

      </div>
    </div>
  </n-config-provider>
</template>
<style>
:root {
  --background-color: #0f0f0f;
  --background-menu: #212121;

  --text-primary: #f1f1f1;

  --outline: #303030;
  --outline-hover: #ffffff14;
  --outline-focus: #888888;
  --outline-active: #888888;
}
</style>
<style scoped>
#app {
  background: var(--background-color);
  color: var(--text-primary);
  height: 100vh;
  width: 100vw;
  position: relative;
}

#container {
  position: absolute;
  top: 35px;
  left: 20px;
  width: calc(100% - 40px);
  height: calc(100% - 38px);
}
</style>
