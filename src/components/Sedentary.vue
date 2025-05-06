<script setup lang="ts">
import {ref, onMounted, onBeforeUnmount} from 'vue';
import {listen} from '@tauri-apps/api/event';
import {useMainStore} from "../stores/useMainStore.ts";

const break_done = ref(false);
const work_done = ref(false);
const move_continue = ref(false);
const break_count = ref(0);
const work_count = ref(0);
const mouse_stop_count = ref(0);

// 监听鼠标位置变化事件
const listenAll = () => {
  listen('break_done', () => {
    break_done.value = true;
    work_done.value = true;
    move_continue.value = false;
  });
  listen('work_done', () => {
    work_done.value = true;
  });
  listen('move_continue', () => {
    move_continue.value = true;
    work_done.value = true;
    break_done.value = false;
  });
  listen<number>('wait_mouse_stop', (data) => {
    mouse_stop_count.value = Number(data.payload);
    work_done.value = true;
    break_done.value = false;
  });
  listen<number>('break_count', (data) => {
    break_count.value = Number(data.payload);
    work_done.value = true;
    break_done.value = false;
    move_continue.value = false;
  });
  listen<number>('work_count', (data) => {
    work_count.value = Number(data.payload);
    break_done.value = false;
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

const mainStore = useMainStore()
</script>
<template>
  <div>
    <n-form
        :model="mainStore.config"
        label-placement="left"
        label-width="auto"
        require-mark-placement="right-hanging"
    >
      <n-form-item path="workDuration" label="久坐时长">
        <n-input-number v-model:value="mainStore.config.workDuration" :show-button="false">
          <template #suffix>
            s
          </template>
        </n-input-number>
      </n-form-item>
      <n-form-item path="breakDuration" label="休息时长">
        <n-input-number v-model:value="mainStore.config.breakDuration" :show-button="false">
          <template #suffix>
            s
          </template>
        </n-input-number>
      </n-form-item>
      <n-form-item path="idleTimeout" label="IDLE超时">
        <n-input-number v-model:value="mainStore.config.idleTimeout" :show-button="false">
          <template #suffix>
            s
          </template>
        </n-input-number>
      </n-form-item>
      <n-form-item path="progressBarHeight" label="进度条宽度">
        <n-input-number v-model:value="mainStore.config.progressBarHeight" :show-button="false">
          <template #suffix>
            s
          </template>
        </n-input-number>
      </n-form-item>
    </n-form>
    
    <p v-if="!work_done">
      坐着ing
      <n-progress :height="23" class="progress" type="line"
                  :percentage="Number((work_count / mainStore.config.workDuration * 100).toFixed(2))"
                  indicator-placement="inside" processing/>
    </p>
    <p v-else-if="move_continue">
      不要再坐着了！
      <n-progress :height="23" class="progress" type="line" :percentage="mouse_stop_count * 10"
                  indicator-placement="inside" processing/>
    </p>
    <p v-else-if="work_done && !break_done">
      休息ing
      <n-progress :height="23" class="progress" type="line"
                  :percentage="Number((break_count / mainStore.config.breakDuration * 100).toFixed(2))"
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