import { createApp } from 'vue';
import App from './App.vue';
import router from './router';
import plugin from '@bazel-example/vue-library';

const app = createApp(App);

app.use(router);
app.use(plugin);
app.mount('#app');
