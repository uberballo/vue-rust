<script setup lang="ts">
import Greetings from './components/Greetings.vue'
import InputZone from './components/InputZone.vue'
import LevelsChart from './components/LevelsChart.vue'
</script>

<template>
  <header>
    <div class="wrapper">
      <Greetings />
    </div>
  </header>

  <main>
    <div class="content-wrapper">
      <InputZone />
      <LevelsChart />
    </div>
  </main>
</template>

<style scoped>
header {
  line-height: 1.5;
}

.content-wrapper {
  display: flex;
  justify-content: space-around;
  align-items: center;
  gap: 10%;
  padding: 10%;
  flex-direction: row-reverse;
}
</style>
