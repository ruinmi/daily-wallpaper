<script setup lang="ts">
import {ArrowLeft12Filled, ArrowRight12Filled, Checkmark12Filled} from "@vicons/fluent";
import {useMainStore} from "../stores/useMainStore.ts";
import {setWallpaper} from "../modules/api";

const mainStore = useMainStore()

const handlePrev = () => {
  if (mainStore.wpIdx - mainStore.firstWpIdx > 0) {
    mainStore.wpIdx -= 1
  }
}

const handleNext = () => {
  if (mainStore.wpIdx - mainStore.firstWpIdx < mainStore.wallpapers.length - 1) {
    mainStore.wpIdx += 1
  }
}

const handleSetWallpaper = async () => {
  await setWallpaper(mainStore.wpIdx)
}

const btnType = computed(() => {
  if (mainStore.wallpapers.find(v => v.idx == mainStore.wpIdx)?.used) {
    return "default"
  } else {
    return "success"
  }
})

</script>

<template>
  <n-carousel
      dot-type="line"
      effect="custom"
      :transition-props="{ name: 'creative' }"
      show-arrow
      :show-dots="false"
      :loop="false"
      style="width: 100%; min-height: 405px; border-radius: 8px; user-select: none"
      :current-index="mainStore.wpIdx - mainStore.firstWpIdx"
  >
    <img class="carousel-img"
         v-for="wp in mainStore.wallpapers"
         :alt="wp.name"
         :src="wp.img_src"
    >
    <template #arrow>
      <div class="custom-arrow">
        <n-button class="custom-arrow-btn" strong secondary @click="handlePrev">
          <n-icon>
            <ArrowLeft12Filled/>
          </n-icon>
        </n-button>
        <n-button class="custom-arrow-btn" strong secondary @click="handleNext">
          <n-icon>
            <ArrowRight12Filled/>
          </n-icon>
        </n-button>
        <n-button strong secondary :type="btnType" @click="handleSetWallpaper"
                  :disabled="mainStore.config.activeWpIdx === mainStore.wpIdx">
          <n-icon>
            <Checkmark12Filled/>
          </n-icon>
        </n-button>
      </div>
    </template>
  </n-carousel>
</template>

<style scoped>
.carousel-img {
  width: 100%;
  min-height: 405px;
  object-fit: cover;
}

.custom-arrow {
  display: flex;
  position: absolute;
  bottom: 25px;
  right: 10px;
}

.custom-arrow button {
  margin-right: 8px;
  width: 30px;
  height: 30px;
  padding: 0;
  transition: background-color 0.3s cubic-bezier(0.4, 0, 0.2, 1);
  cursor: pointer;
}

.custom-arrow-btn:focus,
.custom-arrow-btn:active,
.custom-arrow-btn {
  background-color: #0505055d;
}

.custom-arrow-btn:hover {
  background-color: #31313175;
}

.custom-arrow button:disabled {
  cursor: default;
}

:deep(.creative-enter-from),
:deep(.creative-leave-to) {
  opacity: 0;
  transform: scale(0.8);
}

:deep(.creative-enter-active),
:deep(.creative-leave-active) {
  transition: all 0.3s ease;
}
</style>