<script setup lang="ts">
import {invoke} from '@tauri-apps/api/core'
import * as path from '@tauri-apps/api/path';
import {useMainStore} from "../stores/useMainStore.ts";

const mainStore = useMainStore()

const openWallpaperFolder = async () => {
  const appData = await path.appDataDir()
  const wallpaperPath = await path.join(appData, "wallpapers")
  await invoke('open_folder', {path: wallpaperPath})
}

const openCurrentImageInFolder = async () => {
  const wp = mainStore.wallpapers.find(v => v.idx == mainStore.wpIdx)
  if (wp) {
    await invoke('open_file_in_folder', {path: wp.location})
  }
}
</script>

<template>
  <div id="dashboard">
    <n-grid :cols="6">
      <n-gi span="1">
        <n-statistic>
          <template #default>
            <div class="wp-statistic" v-show="mainStore.wallpapers.length !== 0">
              <span class="wp-idx" @click="openCurrentImageInFolder">{{mainStore.wpIdx - mainStore.firstWpIdx + 1}}</span>
              <span class="wp-count" @click="openWallpaperFolder">{{ mainStore.wallpapers.length }}</span>
            </div>
          </template>
          <template #label>
            <span style="user-select: none">壁纸数</span>
          </template>
        </n-statistic>
      </n-gi>
      <n-gi span="5">
        <n-statistic>
          <template #default>
            <n-ellipsis tooltip>
              {{ mainStore.config.activeWpName }}
            </n-ellipsis>
          </template>
          <template #label>
            <span style="user-select: none">当前壁纸</span>
          </template>
        </n-statistic>
      </n-gi>
    </n-grid>
  </div>
</template>

<style scoped>
#dashboard {
  margin-top: 10px;
}

.wp-statistic {
  position: relative;
  user-select: none;
}
.wp-idx {
  top: 0;
  left: 0;
  font-size: 16px;
  color: #c5c5c5;
  cursor: pointer;
}
.wp-count {
  top: 9px;
  left: 13px;
  font-size: 23px;
  cursor: pointer;
}
.wp-count,
.wp-idx {
  position: absolute;
}
</style>