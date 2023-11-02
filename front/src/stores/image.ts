import { defineStore } from 'pinia'

export type UploadImage = {
    file: FormDataEntryValue | string | null
}


export const useImageStore = defineStore('image', {
    state: () => ({
        image: null as FormDataEntryValue | null
    }),

    actions: {
        addImage(image: FormDataEntryValue) {
            this.image = image
        }
    }

})