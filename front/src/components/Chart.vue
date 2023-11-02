<script>
import { Bar } from 'vue-chartjs'
import { Chart as ChartJS, Title, Tooltip, Legend, BarElement, CategoryScale, LinearScale } from 'chart.js'

ChartJS.register(Title, Tooltip, Legend, BarElement, CategoryScale, LinearScale)

export default {
    name: 'Chart',
    components: { Bar },
    data() {
        return {
            chartData: {
                labels: ['January', 'February', 'March'],
                datasets: [{ data: [40, 20, 12] }]
            },
            chartOptions: {
                responsive: true
            }
        }
    }
}
</script>

<template>
    <div class="chart-wrapper">
        <Bar id="my-chart-id" :options="chartOptions" :data="chartData" />
    </div>
</template>

<style scoped>
.chart-wrapper {
    max-width: 600px;
}
</style>