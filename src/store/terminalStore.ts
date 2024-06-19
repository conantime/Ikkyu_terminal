import {defineStore} from "pinia";
import {removeQuotes} from "@/utils";

const terminalStore = defineStore({
    id: "terminalStore",
    state() {
        return {
            terminals: [],
        }
    },
    actions: {
        updateTerminals(info) {
            if (!this.terminals) {
                return
            }

            this.terminals = [
                {
                    show: true,
                    name: info.ip,
                    context: info.path,
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
                    isSessionMode: true
                }
            ]
        }
    }
})