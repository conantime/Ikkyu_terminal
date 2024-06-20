import {invoke} from "@tauri-apps/api/core";

async function aiCommand(command: string) {
    let contents = 'You are an expert at using shell commands. I need you to provide a response in the format ' +
        '{"command": "your_shell_command_here"}. ' + "linux" +
        'Only provide a single executable line of shell code as the value for the "command" key. Never output any text outside the JSON structure. ' +
        'The command will be directly executed in a shell. For example, if I ask to display the message "Hello, World!", you should respond with' +
        'json\n{"command": "echo Hello, World!"}. ' +
        'Between [], these are the last 1500 tokens from the previous command\'s output, you can use them as context: [' + String(command).replace('aimode', "")
        + '], if it\'s None, don\'t take it into consideration.'

    return await invoke("ai_mod", {
        command: contents
    })

}

interface Terminal_DTO {
    id: string
    show: boolean
    name: string
    context: string
    dragConf: {
        width: string
        height: string
        zIndex: number
        init: {
            x: number
            y: number
        },
        pinned: boolean
    },
    showHeader: boolean
}

class TerminalObj {
    obj

    constructor(dto: Terminal_DTO) {
        this.obj = dto
    }

    updatePath(path: string) {
        this.obj.context = path
    }

    execCmd(cmd: string) {

    }

}