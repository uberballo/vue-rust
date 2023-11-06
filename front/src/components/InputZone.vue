<script setup lang="ts">
import { getHealth, postAnalyzeColorLevels, postInvertColors } from '../services/imageService'
import { useImageStore } from '../stores/image'
import { useColorLevelsStore } from '../stores/colorLevels'
import { ref, type Ref } from 'vue'
const imageStore = useImageStore()
const colorLevelsStore = useColorLevelsStore()
const fileName: Ref<String | undefined> = ref(undefined)

const handleDragEvent = (event: DragEvent) => {
  if (event.dataTransfer?.files[0]) {
    let file = event.dataTransfer.files[0]
    uploadImage(file)
  }
}

const handleInputEvent = (event: Event) => {
  const target = event.target as HTMLInputElement
  if (target.files && target.files[0]) {
    let file = target.files[0]
    uploadImage(file)
  }
}

const uploadImage = (file: File) => {
  fileName.value = file.name

  const formData = new FormData()

  let reader = new FileReader()
  reader.readAsDataURL(file)

  reader.onload = (e) => {
    if (e && e.target) {
      const preview = e.target.result as string
      formData.append('file', preview)
      const file = formData.get('file')
      if (file) {
        imageStore.addImage(file)
      }
    }
  }
}

const getColorLevels = async () => {
  if (imageStore.image) {
    const result = await postAnalyzeColorLevels(imageStore.image)
    if (result) {
      colorLevelsStore.updateLevels(result)
    }
  }
}

const invertImage = async () => {
  if (imageStore.image) {
    const newImage = await postInvertColors(imageStore.image)
    if (newImage) {
      imageStore.addImage(newImage)
    }
  }
}

const healthCheck = async () => {
  const message = await getHealth()
  alert(message.data || message)
}
</script>

<template>
  <main>
    <div class="input-wrapper">
      <div class="dropzone" @dragover.prevent accept="image/png, image/jpeg" @dragenter.prevent @dragstart.prevent
        @drop.prevent="handleDragEvent($event)">
        <input id="file-input" type="file" accept="image/png, image/jpeg" required @change="handleInputEvent($event)" />
        <h2 for="file-input">Click or Drag N Drop Image</h2>
        <img v-if="imageStore.image" v-bind:src="imageStore.image as string" />
        <h3 v-if="imageStore.image">File name: {{ fileName }}</h3>
      </div>
      <div class="button-wrapper">
        <button type="submit" :disabled="!imageStore.image" v-on:click="getColorLevels">Get color levels</button>
        <button type="submit" :disabled="!imageStore.image" v-on:click="invertImage">Invert colors</button>
        <button type="submit" @click="healthCheck">HealthCheck</button>
      </div>
    </div>
  </main>
</template>

<style scoped>
.dropzone {
  height: fit-content;
  min-height: 200px;
  max-height: 700px;
  width: 600px;
  background: #fdfdfd;
  border-radius: 5px;
  border: 2px dashed #000;
  display: flex;
  flex-direction: column;
  justify-content: center;
  align-items: center;
  margin: 0 auto;
}

input[type='file'] {
  position: absolute;
  opacity: 0;
  width: inherit;
  min-height: 200px;
  max-height: 400px;
  cursor: pointer;
}

img {
  width: 50%;
  height: 50%;
}

button {
  background-color: transparent;
  border: 2px solid #000000;
  border-radius: 1em;
  color: #030101;
  cursor: pointer;
  align-self: center;
  font-size: 1rem;
  margin: 10px;
  padding: 1.2em 2.4em;
  text-align: center;
  text-transform: uppercase;
  font-family: 'Montserrat', sans-serif;
  font-weight: 700;
  width: 100%;
  padding: auto;
}

button:disabled {
  color: #666;
  border-color: #666;
  pointer-events: none;
}

.input-wrapper {
  display: flex;
  flex-direction: column;
  width: 50%;
}

.button-wrapper {
  width: 100%;
  table-layout: fixed;
  border-collapse: collapse;
  align-items: center;
  margin-left: 50%;
}
</style>
