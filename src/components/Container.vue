<script lang="ts" setup>
import '~/css/theme/dark.css'
import {Terminal, TerminalApi, TerminalAsk} from '~/index'
import {Command, FailedFunc, Message, SuccessFunc} from "~/types";
import {onBeforeMount, onMounted, ref} from "vue";
import {Command as Cmd} from '@tauri-apps/plugin-shell';
import {getPwd, removeQuotes} from "@/utils";
import {invoke} from "@tauri-apps/api/core";

let terminals: ref<Array<any>>
let loading = ref(false)

getPwd().then((pwdPath) => {
  console.log(pwdPath, typeof pwdPath)
  terminals = ref([
    {
      show: true,
      name: 'terminal',
      context: removeQuotes(pwdPath),
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
  console.log('init')
  loading.value = false
  console.log(terminals.value, 'terminals')
})


const onExecCmd = async (key: string, command: Command, success: SuccessFunc, failed: FailedFunc, name: string) => {
  console.log(command, typeof command)
  if (key === 'aimode') {
    let contents = 'You are an expert at using shell commands. I need you to provide a response in the format ' +
        '{"command": "your_shell_command_here"}. ' + "linux" +
    'Only provide a single executable line of shell code as the value for the "command" key. Never output any text outside the JSON structure. ' +
        'The command will be directly executed in a shell. For example, if I ask to display the message "Hello, World!", you should respond with' +
        'json\n{"command": "echo Hello, World!"}. ' +
        'Between [], these are the last 1500 tokens from the previous command\'s output, you can use them as context: [' + String(command).replace('aimode', "")
        + '], if it\'s None, don\'t take it into consideration.'


    const r = await invoke("ai_mod", {
      command: contents
    })
    console.log(r, 'ai_mod')
    success(r)

    return
  }
  if (terminals.value[0].sshMode) {
    const r = await invoke("execute_ssh_command", {
      command: command
    })
    console.log(r, 'execute_r')
    success(r)
    return
  }
  if (key.trim().startsWith('ssh')) {
    const r = await invoke("ssh", {
      ipWithPort: '124.70.15.11:22',
      username: "root",
      password: "wzh1230.",
      localFilePath: "",
      targetFilePath: "/",
      command: "pwd"
    });
    if (r) {
      terminals.value = [
        {
          show: true,
          name: 'terminal',
          context: r,
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
          showHeader: false,
          sshMode: true
        }
      ]
    }
    success('')
    return
  }
  console.log(name)
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
  <div id="app_cmd">
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
body, html, #app_cmd {
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
