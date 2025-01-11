<script setup lang="ts">
import {ArrowLeft12Filled, ArrowRight12Filled, Checkmark12Filled} from "@vicons/fluent";
import {useMainStore} from "../stores/useMainStore.ts";
import {setWallpaper} from "../modules/api";

const mainStore = useMainStore()
const currentSrc = ref("")
const thumbnail = ref(""); // 缩略图的 Base64 数据
const original = ref(""); // 原始图片路径

const handlePrev = async () => {
  if (mainStore.wpIdx - mainStore.firstWpIdx > 0) {
    mainStore.wpIdx -= 1
    await loadImage()
  }
}
const loadImage = async () => {
  // 生成缩略图
  try {
    thumbnail.value = await generateThumbnail(currentWallpaper.value?.img_src);
    currentSrc.value = thumbnail.value; // 先显示缩略图
  } catch (error) {
    console.error("缩略图生成失败:", error);
  }

  // 设置原始图片路径
  original.value = currentWallpaper.value ? currentWallpaper.value.img_src : ""

  // 预加载原始图片
  const img = new Image();
  img.src = original.value;
  img.onload = () => {
    currentSrc.value = original.value; // 替换为原始图片
  };
  img.onerror = () => {
    console.error("原始图片加载失败");
  }
}

const handleNext = async () => {
  if (mainStore.wpIdx - mainStore.firstWpIdx < mainStore.wallpapers.length - 1) {
    mainStore.wpIdx += 1
    await loadImage()
  }
}

const handleSetWallpaper = async () => {
  await setWallpaper(mainStore.wpIdx)
}

const currentWallpaper = computed(() => {
  return mainStore.wallpapers.find(v => v.idx == mainStore.wpIdx)
})

const btnType = computed(() => {
  if (currentWallpaper.value?.used) {
    return "default"
  } else {
    return "success"
  }
})

const generateThumbnail = (imagePath: string | undefined, maxWidth = 72, maxHeight = 40.5): Promise<string> => {
  return new Promise((resolve, reject) => {
    if (!imagePath) {
      reject()
    } else {
      const img = new Image()
      img.src = imagePath
      img.crossOrigin = "anonymous"

      img.onload = () => {
        const originalWidth = img.naturalWidth;
        const originalHeight = img.naturalHeight;
        let width = maxWidth;
        let height = maxHeight;
        if (originalWidth > originalHeight) {
          height = (maxWidth / originalWidth) * originalHeight;
        } else {
          width = (maxHeight / originalHeight) * originalWidth;
        }
        const canvas = document.createElement("canvas")
        canvas.width = Math.round(width);
        canvas.height = Math.round(height);

        const ctx = canvas.getContext("2d")
        if (ctx) {
          ctx.drawImage(img, 0, 0, width, height)

          // 将缩略图转为 base64
          const thumbnail = canvas.toDataURL("image/jpeg", 0.8) // 0.8 为压缩质量
          resolve(thumbnail);
        }
      }

      img.onerror = () => {
        reject(new Error("图片加载失败"))
      }
    }
  })
}
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
         :src="currentSrc"
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