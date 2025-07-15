<template>
  <i :class="['iconfont', iconName]" />
</template>

<script setup>
import { defineProps, computed } from 'vue'

const props = defineProps({
  name: {
    type: String,
    required: true
  }
})

// 自动添加 icon- 前缀，比如 name="home" => class="icon-home"
const iconName = computed(() => `icon-${props.name}`)
</script>

<style scoped>
.iconfont {
  font-size: 16px;
  color: inherit;
}
</style>