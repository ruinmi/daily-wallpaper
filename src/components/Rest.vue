<script setup lang="ts">
import {ref, onMounted, onBeforeUnmount} from 'vue';
import {listen} from '@tauri-apps/api/event';

const work_duration = 1800;
// const work_duration = 10;
const rest_duration = 300;
// const rest_duration = 5;


const rest_done = ref(false);
const work_done = ref(false);
const move_continue = ref(false);
const rest_count = ref(0);
const work_count = ref(0);
const mouse_stop_count = ref(0);

// 监听鼠标位置变化事件
const listenAll = () => {
  listen('rest_done', () => {
    rest_done.value = true;
    work_done.value = true;
    move_continue.value = false;
  });
  listen('work_done', () => {
    work_done.value = true;
  });
  listen('move_continue', () => {
    move_continue.value = true;
    work_done.value = true;
    rest_done.value = false;
  });
  listen<number>('wait_mouse_stop', (data) => {
    mouse_stop_count.value = Number(data.payload);
    work_done.value = true;
    rest_done.value = false;
  });
  listen<number>('rest_count', (data) => {
    rest_count.value = Number(data.payload);
    work_done.value = true;
    rest_done.value = false;
    move_continue.value = false;
  });
  listen<number>('work_count', (data) => {
    work_count.value = Number(data.payload);
    rest_done.value = false;
    work_done.value = false;
    move_continue.value = false;
  });
};

// 初始化
onMounted(() => {
  listenAll();
});

// 在组件卸载前清理
onBeforeUnmount(() => {
});

// function formatTime(seconds: number): string {
//     // 如果小于一分钟，直接返回秒数
//     if (seconds < 60) {
//         return `${seconds}秒`;
//     }

//     // 计算小时、分钟和秒数
//     const hours = Math.floor(seconds / 3600); // 小时数
//     const minutes = Math.floor((seconds % 3600) / 60); // 分钟数
//     const remainingSeconds = seconds % 60; // 剩余秒数

//     // 如果小时数大于 0，返回 h:m:s 格式
//     if (hours > 0) {
//         return `${hours}小时${minutes}分钟${remainingSeconds}秒`;
//     }

//     // 如果小时数为 0，只返回 m:s 格式
//     return `${minutes}分钟${remainingSeconds}秒`;
// }
</script>
<template>
  <div>
    <p v-if="!work_done">
      坐着ing
      <n-progress :height="23" class="progress" type="line"
                  :percentage="Number((work_count / work_duration * 100).toFixed(2))"
                  indicator-placement="inside" processing/>
    </p>
    <p v-else-if="move_continue">
      不要再坐着了！
      <n-progress :height="23" class="progress" type="line" :percentage="mouse_stop_count * 10"
                  indicator-placement="inside" processing/>
    </p>
    <p v-else-if="work_done && !rest_done">
      休息ing
      <n-progress :height="23" class="progress" type="line"
                  :percentage="Number((rest_count / rest_duration * 100).toFixed(2))"
                  indicator-placement="inside" processing/>
    </p>
    <p v-else>等待回来</p>
  </div>


</template>

<style scoped>
p {
  font-size: 20px;
  color: #fff;
  text-align: center;
}

.progress {
  margin-top: 10px;
}
</style>