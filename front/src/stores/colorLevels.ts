import { defineStore } from 'pinia'

export type ColorLevels = {
    levels: ColorLevel[]
}
type ColorLevel = {
    color: string[],
    count: number
}


export const useColorLevelsStore = defineStore('levels', {
    state: () => ({
        levels: null as ColorLevels | null
    }),

    actions: {
        updateLevels(levels: ColorLevels) {
            this.levels = levels
        }
    }
})