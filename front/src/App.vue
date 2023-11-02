<script setup lang="ts">
import Greetings from './components/Greetings.vue'
import InputZone from './components/InputZone.vue';
import Chart from './components/Chart.vue';
</script>

<template>
  <header>
    <div class="wrapper">
      <Greetings />
    </div>
  </header>

  <main>
    <div class="content-wrapper">
      <Chart />
      <InputZone />
    </div>
  </main>
</template>

<style scoped>
header {
  line-height: 1.5;
}

.content-wrapper {
  padding: auto;
  display: flex;
  justify-content: space-around;
  align-items: center;
}
</style>
