import {createApp} from "vue";
import App from "./App.vue";
import '~/css/theme/dark.css'
// import {Terminal} from "vue-web-terminal"


const app = createApp(App)
// @ts-ignore
// app.use(Terminal)
app.config.globalProperties.productionTip = false

app.mount("#app");

