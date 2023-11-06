<script setup lang="ts">
import { storeToRefs } from 'pinia'
import {
  Chart as ChartJS,
  CategoryScale,
  LinearScale,
  PointElement,
  LineElement,
  Title,
  Tooltip,
  Legend,
  BarElement
} from 'chart.js'
import { Bar } from 'vue-chartjs'
import { computed } from 'vue'
import { useColorLevelsStore } from '@/stores/colorLevels'

ChartJS.register(
  CategoryScale,
  LinearScale,
  PointElement,
  LineElement,
  BarElement,
  Title,
  Tooltip,
  Legend
)

const store = useColorLevelsStore()

const storeData = storeToRefs(store)

const data = computed(() => ({
  labels: storeData.levels.value?.colorLevels.map((x) => x.color),
  datasets: [
    {
      label: 'Colors',
      data: storeData.levels.value?.colorLevels.map((x) => x.count),
      backgroundColor: storeData.levels.value?.colorLevels.map((x) => x.color),
      borderColor: storeData.levels.value?.colorLevels.map((x) => x.color),
      borderWidth: 1
    }
  ]
}))
const options = {
  responsive: true,
  maintainAspectRatio: false
}
</script>

<template>
  <!-- @vue-ignore -->
  <Bar :data="data" :options="options" />
</template>
