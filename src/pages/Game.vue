<template>
  <keep-alive>
    <div class="game-page-main">
      <div class="row-1">
        <instance-info minecraft-version="1.20.1" :instance-name="currentInstance.config.name" :installed="true"
          :game-button-type="gameButtonType" @game-button="() => {
            if (gameButtonType === 'launch') {
              tauri.invoke('launch')
            } else if (gameButtonType === 'install') {
              tauri.invoke('install')
            }
          }" :error-type="errorType"></instance-info>
        <assets-manager :instance-name="currentInstance.config.name" style="margin-top: 20px;"></assets-manager>
      </div>
      <div class="row-2">
        <!-- <div class="group-name"> 
          <div style="display: flex; justify-content: space-between; align-items: center; height: 100%;">
            <p style="margin-left: 4px;">帐户</p>
            <button class="group-button" style="margin-right: 6px;"><i class="chevron-right" style="font-size: 12px;"></i></button>
          </div>
        </div>
        <account-manager></account-manager> -->
        <div class="group-name"> <!--todo: move to a component-->
          <div style="display: flex; justify-content: space-between; align-items: center; height: 100%;">
            <p style="margin-left: 4px;">游戏</p>
            <button class="group-button" @click="show.instanceManager = true" style="margin-right: 6px;"><i
                class="chevron-right" style="font-size: 12px;"></i></button>
          </div>
        </div>
        <Instances :instances="instances" @select="setCurrentInstance"></Instances>
        <instance-manager :show="show.instanceManager" @close="show.instanceManager = false"
          :instances="instances" @update="update"></instance-manager>
        <div class="group-name">
          <div style="display: flex; justify-content: space-between; align-items: center; height: 100%;">
            <p style="margin-left: 4px;">好友</p>
            <button class="group-button" style="margin-right: 6px;"><i class="chevron-right"
                style="font-size: 12px;"></i></button>
          </div>
        </div>
      </div>
    </div>
  </keep-alive>
</template>

<script setup lang="ts">
import InstanceInfo from '@/components/InstanceInfo.vue';
import AssetsManager from '@/components/AssetsManager.vue';
import AccountManager from '@/components/AccountManager.vue';
import Instances from '@/components/Instances.vue';
import InstanceManager from "@/pages/dialogs/InstanceManager.vue";
import { reactive, ref, type Ref } from 'vue';
import { tauri } from '@tauri-apps/api';

interface Instance {
  config: {
    name: string,
    runtime: string,
  },
  installed: boolean
}

let currentInstance = ref<Instance>({
  config: {
    name: "",
    runtime: ""
  },
  installed: false
})
let show = ref({
  instanceManager: true
})
let instances = ref([])
let gameButtonType: Ref<"installing" | "launching" | "install" | "launch" | "error"> = ref("error")
let errorType: Ref<"launch" | "install" | undefined> = ref()

function update() {
  tauri.invoke("scan_instances_folder").then((res: any) => {
    instances.value = res
    console.log(instances.value)
  })
}

update()
let modIsLoading = ref(false)
let resourcepacksIsLoading = ref(false)
let shaderpackIsLoading = ref(false)
let savesIsLoading = ref(false)
let mods = ref([])
let resourcepacks = ref([])
let shaderpacks = ref([])
let saves = ref([])

function updateData() {
  modIsLoading.value = true
  resourcepacksIsLoading.value = true
  shaderpackIsLoading.value = true
  savesIsLoading.value = true
  tauri.invoke("scan_mod_folder").then((res: any) => {
    mods.value = res.sort((a: any, b: any) => a.name.localeCompare(b.name))
    modIsLoading.value = false
  })
  tauri.invoke('scan_saves_folder').then((res: any) => {
    saves.value = res
    savesIsLoading.value = false
  })
}

function setCurrentInstance(instance: Instance) {
  currentInstance.value = instance
  gameButtonType.value = instance.installed ? "launch" : "install"
  tauri.invoke('set_current_instance', { instanceName: instance.config.name })
}


// tauri.invoke("scan_instances_folder").then((res: any) => {
//   instances.value = res
// })
</script>

<style lang="less" scoped>
.game-page-main {
  width: 100%;
  height: 100%;
  display: flex;
}

.row-1,
.row-2 {
  height: 100%;
  display: flex;
  flex-direction: column;
  // border: 1px solid #fff;
}

.row-1 {
  padding: 24px 24px;
  width: 100%;
}

.row-2 {
  width: 328px;
  padding: 24px 24px 24px 0;
  flex-shrink: 0;
}

// todo: move to main.css
.group-name {
  width: 100%;
  height: 32px;
  font-size: 14.5px;
  margin-bottom: 8px;
}

.group-button {
  background: rgba(255, 255, 255, 0.08);
  border: none;
  border-radius: 4px;
  height: 20px;
  width: 20px;
  font-size: 10px;
}

.group-button,
.group-button * {
  cursor: pointer;
}

.group-button i::before {
  transform: scale(0.7)
}
</style>