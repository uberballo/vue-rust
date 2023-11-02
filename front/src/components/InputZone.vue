
<script setup lang="ts">
import { getRequest, postAnalyzeColorLevels, postInvertColors } from '../services/imageService';
import { useImageStore } from '../stores/image'
import { useColorLevelsStore } from '../stores/colorLevels'
const imageStore = useImageStore()
const colorLevelsStore = useColorLevelsStore()

const handleFileChange = (event: Event) => {
    const target = event.target as HTMLInputElement;
    if (target && target.files) {
        console.log(target.files[0])
        let file = target.files[0]
        //Unsued
        const fileName = file.name;

        const formData = new FormData();

        let reader = new FileReader();
        reader.readAsDataURL(file);

        reader.onload = (e) => {
            if (e && e.target) {

                const preview = e.target.result as string;
                formData.append("file", preview);
                const file = formData.get("file");
                if (file) {
                    imageStore.addImage(file)
                }

            }
        };
    }
}

const getColorLevels = async () => {
    console.log(imageStore.image)
    if (imageStore.image) {
        const levels = await postAnalyzeColorLevels(imageStore.image)
        console.log(levels)
        if (levels) {
            colorLevelsStore.updateLevels(levels)
        }
    }
}

const invertImage = async () => {
    console.log(imageStore.image)
    if (imageStore.image) {
        const newImage = await postInvertColors(imageStore.image)
        if (newImage) {
            imageStore.addImage(newImage)
        }
    }
}
</script>

<template>
    <main>
        <div class="input-wrapper">
            <div class="dropzone" @dragover.prevent @dragenter.prevent @dragstart.prevent
                @drop.prevent="handleFileChange($event)">
                <input id="file-input" type="file" accept="image/png, image/jpeg" required
                    @change="handleFileChange($event)" />
                <h2 for="file-input">Click or Drag N Drop Image</h2>
                <img v-if="imageStore.image" v-bind:src="(imageStore.image as string)" />
                <!-- <h3 v-if="preview">File name: {{ fileName }}</h3> -->
            </div>
            <div class="button-wrapper">
                <button type="submit" v-on:click="getColorLevels">Get color levels</button>
                <button type="submit" v-on:click="invertImage">Invert colors</button>
                <button type="submit" @click="getRequest">HealthCheck</button>
                <!--   <h3 v-if="success">File Uploaded Successfully. publicId: {{ success }}</h3> -->
            </div>
        </div>
    </main>
</template>

<style scoped>
.dropzone {
    height: fit-content;
    min-height: 200px;
    max-height: 400px;
    width: 400px;
    background: #fdfdfd;
    border-radius: 5px;
    border: 2px dashed #000;
    display: flex;
    flex-direction: column;
    justify-content: center;
    align-items: center;
    margin: 0 auto;
}

input[type="file"] {
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
    border: 2px solid #e74c3c;
    border-radius: 1em;
    color: #e74c3c;
    cursor: pointer;
    display: flex;
    align-self: center;
    font-size: 1rem;
    margin: 20px;
    padding: 1.2em 2.4em;
    text-align: center;
    text-transform: uppercase;
    font-family: "Montserrat", sans-serif;
    font-weight: 700;
}

.input-wrapper {
    display: flex;
    flex-direction: column;
}

.button-wrapper {
    display: flex;
}
</style>