<!-- Conic Launcher -->
<!-- Copyright 2022-2026 Broken-Deer and contributors. All rights reserved. -->
<!-- SPDX-License-Identifier: GPL-3.0-only -->

<template>
  <div>
    <setting-group>
      <setting-item
        :title="$t('settings.general.language')"
        :description="$t('settings.general.languageDesc')"
        icon="language">
        <select-vue
          :display-name="['English', '简体中文']"
          :options="['en_us', 'zh_cn']"
          v-model="config.language"
          :default="1"></select-vue>
      </setting-item>
      <setting-item
        :title="$t('settings.general.updateChannel')"
        :description="$t('settings.general.updateChannelDesc')"
        icon="radio">
        <select-vue
          :display-name="[
            $t('settings.general.weekly'),
            $t('settings.general.snapshot'),
            $t('settings.general.release'),
          ]"
          :options="['Weekly', 'Snapshot', 'Release']"
          v-model="config.update_channel"
          :default="2"></select-vue>
      </setting-item>
      <setting-item
        :title="$t('settings.general.autoUpdate')"
        :description="$t('settings.general.autoUpdateDesc')"
        icon="clock">
        <toggle-switch v-model="config.auto_update"></toggle-switch>
      </setting-item>
      <setting-item :title="$t('settings.general.checkUpdate')" icon="circle-up" :clickAble="true">
        <i class="chevron-right" style="margin-right: 10px"></i>
      </setting-item>
    </setting-group>
  </div>
</template>

<script setup lang="ts">
import SettingItem from "@/components/SettingItem.vue";
import SelectVue from "@/components/Select.vue";
import ToggleSwitch from "@/components/ToggleSwitch.vue";
import SettingGroup from "@/components/SettingGroup.vue";
import { useConfigStore } from "@/store/config";

const config = useConfigStore();
</script>

<style lang="less"></style>
