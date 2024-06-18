<script lang="ts" setup>
import '~/css/theme/dark.css'
import {Terminal, TerminalApi, TerminalAsk} from '~/index'
import {Command, FailedFunc, Message, SuccessFunc} from "~/types";
import {ref} from "vue";
import { Command as Cmd } from '@tauri-apps/plugin-shell';



const terminals = ref<any>([
  {
    show: true,
    name: 'terminal',
    context: '/root',
    dragConf: {
      width: "60%",
      height: "50%",
      zIndex: 100,
      init: {
        x: 100,
        y: 70
      },
      pinned: true
    },
    showHeader: false
  }
])

const onExecCmd = async (key: string, command: Command, success: SuccessFunc, failed: FailedFunc, name: string) => {
    let result = await Cmd.create('exec-sh', [
      '-c',
      key,
    ]).execute();
    console.log(result.stdout, 'res');
    success(result.stdout)
}

const onActive = (name: string) => {
  terminals.value.forEach((o: any) => {
    if (o.name === name) {
      o.dragConf.zIndex = 101
    }
  })
}

const onInactive = (name: string) => {
  terminals.value.forEach((o: any) => {
    if (o.name === name) {
      o.dragConf.zIndex = 100
    }
  })
}
const pushMessageBefore = (message: Message, name: string) => {
  console.log(message, name)
}

const getCommand = () => {
  console.log("The current command is[", TerminalApi.getCommand(terminals.value[0].name), "]")
}

const setCommand = () => {
  // 设置指令参数
  TerminalApi.setCommand(terminals.value[0].name, "The custom command -a xxx")
}

</script>
<template>
  <div id="app">
    <div v-for="(item,i) in terminals" :key="i">
      <terminal
          v-show="item.show"
          :name="item.name"
          :title="item.name"
          :context="item.context"
          context-suffix=" > "
          :warn-log-count-limit="200"
          :drag-conf="item.dragConf"
          :show-header="item.showHeader"
          :push-message-before="pushMessageBefore"
          @exec-cmd="onExecCmd"
          @on-active="onActive"
          @on-inactive="onInactive"
          :log-size-limit="20"
          cursor-style="bar"
          :cursor-blink="true"
          :line-space="15"
          enable-hover-stripe
          :enable-fold="true"
          style="position: fixed">
        <!--        <template #header>-->
        <!--          <div class="custom-header">This is custom header</div>-->
        <!--        </template>-->
      </terminal>
    </div>
  </div>
</template>

<style>
body, html, #app {
  margin: 0;
  padding: 0;
  width: 100%;
  height: 100%;
  background-color: #ffffff;
  font-family: Menlo, Consolas, monospace;
  overflow: auto;
}

.custom-header {
  background-color: #0eb4b4;
  color: white;
  text-align: center;
  height: 100px;
}
</style>
