<script setup lang="ts">
import { ref, onMounted } from "vue";
import { invoke } from "@tauri-apps/api/tauri";
import { open } from "@tauri-apps/api/dialog";
import { appLocalDataDir } from "@tauri-apps/api/path";

defineProps<{ meg: string }>();
const outputMsg = ref<string>("");

onMounted(() => {
  //  console.log(a * b);
})
const selectDir = async () => { 
  const selected = await open({
  directory: false,
  multiple: false,
  defaultPath:await appLocalDataDir(),
});
  if (selected === null) {
    // user cancelled the selection
    outputMsg.value = "选择文件";
  } else {
    // user selected a single directory
    // greetMsg.value = selected;
    console.log(selected);
    outputMsg.value = await invoke("dig_file", {path: selected});
  }
}

</script>



<template>

  <button @click="selectDir">选择文件</button>
  <div id="displayElementId" v-html="outputMsg" style="text-align: left;" ></div> 


</template>
