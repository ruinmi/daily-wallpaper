<script setup lang="ts">
import {Delete24Regular} from "@vicons/fluent";
import {useCriteriaStore} from "../stores/useCriteriaStore.ts";
import {batchRemoveFiles, fetchWallpaper} from "../modules/api";
import {useMainStore} from "../stores/useMainStore.ts";
import {rules, orientationOptions} from "../modules/criteria-form"
import {executor, WallpaperInfo} from "../modules/database";
import {useMessage} from 'naive-ui'

const mainStore = useMainStore()
const criteriaStore = useCriteriaStore()
const deleting = ref(false)
const message = useMessage()

const handleFetchWallpaper = async () => {
  await fetchWallpaper()
}

const handleTimeUpdate = (formattedTime: string) => {
  if (formattedTime) {
    const hm = formattedTime.split(':')
    criteriaStore.criteria.updateHours = parseInt(hm[0])
    criteriaStore.criteria.updateMinutes = parseInt(hm[1])
  }
}

const toFormattedTime = () => {
  return `${criteriaStore.criteria.updateHours.toString().padStart(2, '0')}:${criteriaStore.criteria.updateMinutes.toString().padStart(2, '0')}`
}

const handleDeleteWallpaper = async () => {
  deleting.value = true

  const activeItem = mainStore.wallpapers.find(v => v.idx === mainStore.config.activeWpIdx)

  if (!activeItem) {
    message.success(`没有可删除的壁纸`)
    deleting.value = false
    return
  }

  const activeIdx = mainStore.wallpapers.indexOf(activeItem);

  if (activeIdx === 0) {
    message.success(`没有可删除的壁纸`)
    deleting.value = false
    return
  }

  const deletedWallpapers = mainStore.wallpapers.splice(0, activeIdx)
  const oldLocations = deletedWallpapers.map(v => v.location)

  try {
    await batchRemoveFiles(oldLocations)
    await Promise.all(
        deletedWallpapers.map(deletedWp =>
            executor.delete(WallpaperInfo.TABLE_NAME, deletedWp.id, true)
        )
    )
    if (mainStore.firstWpIdx > mainStore.wpIdx) {
      mainStore.wpIdx = mainStore.firstWpIdx;
    }
    message.success(`删除了${oldLocations.length}张壁纸`)
  } catch (error) {
    console.error("删除壁纸时出错:", error)
    message.error("删除壁纸时发生错误，请重试")
  } finally {
    deleting.value = false
  }
}
</script>

<template>
  <div class="form-container">
    <div v-if="mainStore.displayOverlay" class="overlay">
      <n-spin size="medium" description="获取壁纸中..."/>
    </div>
    <n-form
        :model="criteriaStore.criteria"
        label-placement="left"
        label-width="auto"
        require-mark-placement="right-hanging"
        :rules="rules"
    >
      <n-form-item path="keyword" label="关键字">
        <n-input v-model:value="criteriaStore.criteria.keyword" placeholder="键入壁纸关键字"/>
      </n-form-item>
      <n-form-item path="color" label="壁纸主颜色">
        <n-color-picker v-model:value="criteriaStore.criteria.color"/>
      </n-form-item>
      <n-form-item path="orientation" label="壁纸朝向">
        <n-select v-model:value="criteriaStore.criteria.orientation" :options="orientationOptions"/>
      </n-form-item>
      <n-form-item path="updateTime" label="更新时间">
        <n-time-picker @update-formatted-value="handleTimeUpdate" :formatted-value="toFormattedTime()" format="HH:mm"
                       style="width: 100%"/>
      </n-form-item>
      <n-form-item path="perPage" label="每组个数">
        <n-input-number v-model:value="criteriaStore.criteria.perPage" max="30" placeholder="个数" style="width: 100%"/>
      </n-form-item>
      <n-row :gutter="[0, 24]">
        <n-col :span="24">
          <div style="display: flex; justify-content: flex-end">
            <n-button strong secondary style="margin-right: 8px; padding: 10px" :loading="deleting"
                      @click="handleDeleteWallpaper">
              <template #icon>
                <n-icon>
                  <Delete24Regular/>
                </n-icon>
              </template>
              删除旧壁纸
            </n-button>
            <n-button strong secondary
                      :disabled="criteriaStore.criteria.keyword?.trim() === '' || !criteriaStore.criteria.perPage"
                      @click="handleFetchWallpaper">获取一组
            </n-button>
          </div>
        </n-col>
      </n-row>
    </n-form>
  </div>
</template>

<style scoped>
.form-container {
  position: relative;
}

/* 遮罩样式 */
.overlay {
  position: absolute;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
  display: flex;
  align-items: center;
  justify-content: center;
  background-color: rgb(15, 15, 15); /* 半透明背景 */
  z-index: 10;
  pointer-events: all; /* 阻止表单被操作 */
}


</style>