<script setup lang="ts">
import SideBar from "@/layout/SideBar.vue";
import ActionBar from "@/components/ActionBar.vue";
</script>

<template>
  <div class="container">
    <div class="action-bar">
      <ActionBar/>
    </div>
    <div class="layout">
      <SideBar class="sidebar"/>
      <div class="main-content">
        <router-view/>
      </div>
    </div>
  </div>
</template>

<style scoped>
.container {
  display: flex;
  flex-direction: column;
  height: 100vh;
}

.action-bar {
  width: 100%;
  flex-shrink: 0;
  height: 30px;
}

.layout {
  display: flex;
  flex: 1;
  min-height: 0;
  height: 100%;
}

.layout::-webkit-scrollbar {
  display: none;
}

.main-content {
  margin-left: 160px;
  overflow-y: auto;
  width: 100%;
  height: 100%;
  padding: 0 8px;
}

.main-content::-webkit-scrollbar {
  display: none;
}
</style>